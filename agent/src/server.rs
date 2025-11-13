use std::{convert::Infallible, time::Duration};

use anyhow::Result;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
};
use futures_core::stream::Stream;
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::oneshot};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use uuid::Uuid;

use crate::{
    config::AgentConfig,
    events::EventHub,
    models::{ContainerModel, TaskModel},
    services::ContainerService,
    store::InMemoryStore,
    virtualization::Platform,
};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct AppState {
    pub config: AgentConfig,
    pub events: EventHub,
    pub store: InMemoryStore,
    pub containers: ContainerService,
    pub started_at: OffsetDateTime,
}

impl AppState {
    pub fn new(
        config: AgentConfig,
        events: EventHub,
        store: InMemoryStore,
        containers: ContainerService,
    ) -> Self {
        Self {
            config,
            events,
            store,
            containers,
            started_at: OffsetDateTime::now_utc(),
        }
    }
}

pub async fn serve(state: AppState, shutdown: oneshot::Receiver<()>) -> Result<()> {
    let app = Router::new()
        .route("/system/info", get(system_info))
        .route("/containers", get(list_containers).post(create_container))
        .route("/tasks", get(list_tasks))
        .route("/tasks/:task_id", get(task_detail))
        .route("/events/stream", get(events_stream))
        .with_state(state.clone());

    let listener = TcpListener::bind(state.config.api_bind).await?;
    tracing::info!("API escuchando en {}", state.config.api_bind);

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let _ = shutdown.await;
            tracing::info!("Recibida señal de apagado para el servidor HTTP");
        })
        .await?;

    Ok(())
}

async fn system_info(State(state): State<AppState>) -> Json<SystemInfo> {
    let uptime = OffsetDateTime::now_utc() - state.started_at;
    let build = option_env!("VERGEN_GIT_DESCRIBE")
        .unwrap_or("dev")
        .to_string();

    Json(SystemInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        build,
        uptime_seconds: uptime.whole_seconds().max(0) as u64,
        driver_status: "not-configured".into(),
    })
}

#[derive(Serialize)]
struct SystemInfo {
    version: String,
    build: String,
    uptime_seconds: u64,
    driver_status: String,
}

async fn events_stream(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.events.subscribe();
    let stream = BroadcastStream::new(rx).filter_map(|event| match event {
        Ok(envelope) => match serde_json::to_string(&envelope) {
            Ok(json) => Some(Ok(Event::default().data(json))),
            Err(err) => {
                tracing::error!(?err, "No se pudo serializar evento SSE");
                None
            }
        },
        Err(err) => {
            tracing::warn!(?err, "Error recibiendo evento SSE");
            None
        }
    });

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive"),
    )
}

async fn list_containers(State(state): State<AppState>) -> Json<Vec<ContainerModel>> {
    let containers = state.store.list_containers().await;
    Json(containers)
}

async fn create_container(
    State(state): State<AppState>,
    Json(payload): Json<CreateContainerRequest>,
) -> Result<Json<TaskModel>, (StatusCode, String)> {
    let platform = payload.platform.into_platform();
    state
        .containers
        .create_container(payload.name, platform, payload.description)
        .await
        .map(Json)
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("No se pudo crear el contenedor: {err}"),
            )
        })
}

async fn list_tasks(State(state): State<AppState>) -> Json<Vec<TaskModel>> {
    let tasks = state.store.list_tasks().await;
    Json(tasks)
}

async fn task_detail(
    Path(task_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<TaskModel>, StatusCode> {
    state
        .store
        .get_task(task_id)
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[derive(Deserialize)]
struct CreateContainerRequest {
    name: String,
    description: Option<String>,
    platform: PlatformInput,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
enum PlatformInput {
    WindowsX64,
    WindowsArm64,
}

impl PlatformInput {
    fn into_platform(self) -> Platform {
        match self {
            PlatformInput::WindowsX64 => Platform::WindowsX64,
            PlatformInput::WindowsArm64 => Platform::WindowsArm64,
        }
    }
}
