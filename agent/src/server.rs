use std::{convert::Infallible, time::Duration};

use anyhow::Result;
use axum::{
    Json, Router,
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
};
use futures_core::stream::Stream;
use serde::Serialize;
use tokio::{net::TcpListener, sync::oneshot};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use crate::{config::AgentConfig, events::EventHub};
use time::OffsetDateTime;

#[derive(Clone)]
pub struct AppState {
    pub config: AgentConfig,
    pub events: EventHub,
    pub started_at: OffsetDateTime,
}

impl AppState {
    pub fn new(config: AgentConfig, events: EventHub) -> Self {
        Self {
            config,
            events,
            started_at: OffsetDateTime::now_utc(),
        }
    }
}

pub async fn serve(state: AppState, shutdown: oneshot::Receiver<()>) -> Result<()> {
    let app = Router::new()
        .route("/system/info", get(system_info))
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
