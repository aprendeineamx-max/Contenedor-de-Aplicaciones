use std::{convert::Infallible, time::Duration};

use anyhow::Result;
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    middleware::from_fn_with_state,
    response::sse::{Event, KeepAlive, Sse},
    routing::{get, post},
};
use futures_core::stream::Stream;
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::oneshot};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use uuid::Uuid;

use crate::{
    config::AgentConfig,
    events::EventHub,
    models::{AppInstance, ContainerModel, Snapshot, SnapshotType, TaskModel},
    security::{AuthManager, auth_middleware},
    services::{AppService, ContainerService, SnapshotService},
    store::SqliteStore,
    virtualization::Platform,
};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct AppState {
    pub config: AgentConfig,
    pub events: EventHub,
    pub store: SqliteStore,
    pub containers: ContainerService,
    pub apps: AppService,
    pub snapshots: SnapshotService,
    pub auth: AuthManager,
    pub started_at: OffsetDateTime,
}

impl AppState {
    pub fn new(
        config: AgentConfig,
        events: EventHub,
        store: SqliteStore,
        containers: ContainerService,
        apps: AppService,
        snapshots: SnapshotService,
        auth: AuthManager,
    ) -> Self {
        Self {
            config,
            events,
            store,
            containers,
            apps,
            snapshots,
            auth,
            started_at: OffsetDateTime::now_utc(),
        }
    }
}

pub async fn serve(state: AppState, shutdown: oneshot::Receiver<()>) -> Result<()> {
    let app = Router::new()
        .route("/system/info", get(system_info))
        .route("/containers", get(list_containers).post(create_container))
        .route(
            "/containers/:container_id",
            get(get_container).delete(delete_container),
        )
        .route(
            "/containers/:container_id/apps",
            get(list_apps).post(install_app),
        )
        .route("/apps/:app_id/launch", post(launch_app))
        .route(
            "/containers/:container_id/snapshots",
            get(list_snapshots).post(create_snapshot),
        )
        .route("/snapshots/:snapshot_id/restore", post(restore_snapshot))
        .route("/tasks", get(list_tasks))
        .route("/tasks/:task_id", get(task_detail))
        .route("/events/stream", get(events_stream))
        .with_state(state.clone())
        .layer(from_fn_with_state(state.auth.clone(), auth_middleware));

    let listener = TcpListener::bind(state.config.api_bind).await?;
    tracing::info!("API escuchando en {}", state.config.api_bind);

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let _ = shutdown.await;
            tracing::info!("Recibida senal de apagado para el servidor HTTP");
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

#[derive(Deserialize)]
struct ContainersQuery {
    status: Option<String>,
}

async fn list_containers(
    Query(params): Query<ContainersQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ContainerModel>>, StatusCode> {
    state
        .store
        .list_containers(params.status.map(|s| s.to_lowercase()))
        .await
        .map(Json)
        .map_err(|err| {
            tracing::error!(?err, "No se pudieron listar los contenedores");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

#[derive(Deserialize)]
struct CreateContainerRequest {
    name: String,
    description: Option<String>,
    platform: String,
}

async fn create_container(
    State(state): State<AppState>,
    Json(payload): Json<CreateContainerRequest>,
) -> Result<Json<TaskModel>, (StatusCode, String)> {
    let platform = Platform::from_str(&payload.platform);
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

async fn get_container(
    Path(container_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<ContainerModel>, StatusCode> {
    state
        .containers
        .get_container(container_id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Error consultando contenedor");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn delete_container(
    Path(container_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<TaskModel>, StatusCode> {
    state
        .containers
        .delete_container(container_id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Error eliminando contenedor");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[derive(Deserialize)]
struct AppInstallRequest {
    name: Option<String>,
    version: Option<String>,
    installer_path: Option<String>,
    silent_args: Option<String>,
}

async fn list_apps(
    Path(container_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<AppInstance>>, StatusCode> {
    state
        .apps
        .list(container_id)
        .await
        .map(Json)
        .map_err(|err| {
            tracing::error!(?err, "No se pudieron listar las apps");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

async fn install_app(
    Path(container_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<AppInstallRequest>,
) -> Result<Json<TaskModel>, (StatusCode, String)> {
    let AppInstallRequest {
        name,
        version,
        installer_path,
        silent_args,
    } = payload;
    let resolved_name = name
        .or(installer_path.clone())
        .unwrap_or_else(|| "Aplicacion".into());
    let _ = (installer_path, silent_args);
    state
        .apps
        .install(container_id, resolved_name, version)
        .await
        .map(Json)
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("No se pudo instalar la app: {err}"),
            )
        })
}

#[derive(Deserialize)]
struct LaunchAppRequest {
    entry_point_id: Option<String>,
    args: Option<Vec<String>>,
}

async fn launch_app(
    Path(app_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<LaunchAppRequest>,
) -> Result<Json<TaskModel>, StatusCode> {
    let _ = (payload.entry_point_id, payload.args);
    state
        .apps
        .launch(app_id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Error lanzando app");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[derive(Deserialize)]
struct SnapshotRequest {
    label: Option<String>,
    #[serde(rename = "type")]
    snapshot_type: Option<String>,
    base_snapshot_id: Option<Uuid>,
}

async fn list_snapshots(
    Path(container_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Snapshot>>, StatusCode> {
    state
        .snapshots
        .list(container_id)
        .await
        .map(Json)
        .map_err(|err| {
            tracing::error!(?err, "No se pudieron listar snapshots");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

async fn create_snapshot(
    Path(container_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<SnapshotRequest>,
) -> Result<Json<TaskModel>, (StatusCode, String)> {
    let SnapshotRequest {
        label,
        snapshot_type,
        base_snapshot_id,
    } = payload;
    let _ = base_snapshot_id;
    let resolved_type = snapshot_type
        .map(|value| SnapshotType::from_str(&value))
        .unwrap_or(SnapshotType::Full);
    state
        .snapshots
        .create(container_id, label, resolved_type)
        .await
        .map(Json)
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("No se pudo crear el snapshot: {err}"),
            )
        })
}

async fn restore_snapshot(
    Path(snapshot_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<TaskModel>, StatusCode> {
    state
        .snapshots
        .restore(snapshot_id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Error restaurando snapshot");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[derive(Deserialize)]
struct TasksQuery {
    status: Option<String>,
    limit: Option<i64>,
}

async fn list_tasks(
    Query(params): Query<TasksQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<TaskModel>>, StatusCode> {
    state
        .store
        .list_tasks(
            params.status.map(|s| s.to_lowercase()),
            params.limit.map(|l| l.clamp(1, 500)),
        )
        .await
        .map(Json)
        .map_err(|err| {
            tracing::error!(?err, "No se pudieron listar tareas");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

async fn task_detail(
    Path(task_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<TaskModel>, StatusCode> {
    state
        .store
        .get_task(task_id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Error consultando tarea");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
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
