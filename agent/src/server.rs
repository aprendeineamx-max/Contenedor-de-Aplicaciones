use std::{collections::BTreeSet, convert::Infallible, time::Duration as StdDuration};

use anyhow::Result;
use axum::{
    Json, Router,
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    middleware::from_fn_with_state,
    response::sse::{Event, KeepAlive, Sse},
    routing::{delete, get, post},
};
use futures_core::stream::Stream;
use serde::{Deserialize, Serialize};
use tokio::{net::TcpListener, sync::oneshot};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use uuid::Uuid;

use crate::{
    config::{self, AgentConfig, ConfigSnapshot, ConfigSourcesView, SecurityConfig},
    events::EventHub,
    models::{ApiTokenInfo, AppInstance, ContainerModel, Snapshot, SnapshotType, TaskModel},
    security::{AuthContext, AuthManager, SecuritySnapshot, auth_middleware},
    services::{AppService, ContainerService, SnapshotService, TokenService, TokenSpec},
    store::SqliteStore,
    virtualization::Platform,
};
use time::{Duration, OffsetDateTime, format_description::well_known::Rfc3339};

const SCOPE_CONTAINERS_READ: &str = "containers:read";
const SCOPE_CONTAINERS_WRITE: &str = "containers:write";
const SCOPE_TASKS_READ: &str = "tasks:read";

#[derive(Clone)]
pub struct AppState {
    pub config: AgentConfig,
    pub events: EventHub,
    pub store: SqliteStore,
    pub containers: ContainerService,
    pub apps: AppService,
    pub snapshots: SnapshotService,
    pub tokens: TokenService,
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
        tokens: TokenService,
        auth: AuthManager,
    ) -> Self {
        Self {
            config,
            events,
            store,
            containers,
            apps,
            snapshots,
            tokens,
            auth,
            started_at: OffsetDateTime::now_utc(),
        }
    }
}

pub async fn serve(state: AppState, shutdown: oneshot::Receiver<()>) -> Result<()> {
    let app = Router::new()
        .route("/system/info", get(system_info))
        .route("/system/config", get(system_config))
        .route("/system/security/reload", post(reload_security))
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
        .route(
            "/security/tokens",
            get(list_api_tokens).post(create_api_token),
        )
        .route("/security/tokens/:token_id", delete(revoke_api_token))
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

async fn system_config(
    Extension(ctx): Extension<AuthContext>,
    State(state): State<AppState>,
) -> Result<Json<ConfigResponse>, StatusCode> {
    ensure_admin(&ctx)?;
    let config_snapshot = state.config.snapshot();
    let security_snapshot = state.auth.snapshot().await;
    let tokens = state.tokens.list().await.map_err(|err| {
        tracing::error!(?err, "No se pudieron listar tokens administrados");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let sources = config::config_sources_view();
    let security_view = build_security_response(&security_snapshot, &tokens);

    Ok(Json(ConfigResponse {
        config: config_snapshot,
        security: security_view,
        sources,
    }))
}

#[derive(Serialize)]
struct SystemInfo {
    version: String,
    build: String,
    uptime_seconds: u64,
    driver_status: String,
}

#[derive(Serialize)]
struct ConfigResponse {
    config: ConfigSnapshot,
    security: SecurityReloadResponse,
    sources: ConfigSourcesView,
}

#[derive(Serialize)]
struct SecurityReloadResponse {
    auth_enabled: bool,
    admin_token_present: bool,
    static_token_count: usize,
    managed_token_count: u64,
    expiring_token_count: u64,
    scopes_catalog: Vec<String>,
}

async fn reload_security(
    Extension(ctx): Extension<AuthContext>,
    State(state): State<AppState>,
) -> Result<Json<SecurityReloadResponse>, StatusCode> {
    ensure_admin(&ctx)?;
    let latest = SecurityConfig::from_env();
    state.auth.reload(latest).await;
    let snapshot = state.auth.snapshot().await;
    let tokens = state.tokens.list().await.map_err(|err| {
        tracing::error!(?err, "No se pudieron listar tokens administrados");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let response = build_security_response(&snapshot, &tokens);

    Ok(Json(response))
}

#[derive(Deserialize)]
struct ContainersQuery {
    status: Option<String>,
}

async fn list_containers(
    Extension(ctx): Extension<AuthContext>,
    Query(params): Query<ContainersQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ContainerModel>>, StatusCode> {
    ensure_scope(&ctx, SCOPE_CONTAINERS_READ)?;
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
    Extension(ctx): Extension<AuthContext>,
    State(state): State<AppState>,
    Json(payload): Json<CreateContainerRequest>,
) -> Result<Json<TaskModel>, (StatusCode, String)> {
    ensure_scope(&ctx, SCOPE_CONTAINERS_WRITE).map_err(forbidden_with_message)?;
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
    Extension(ctx): Extension<AuthContext>,
    Path(container_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<ContainerModel>, StatusCode> {
    ensure_scope(&ctx, SCOPE_CONTAINERS_READ)?;
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
    Extension(ctx): Extension<AuthContext>,
    Path(container_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<TaskModel>, StatusCode> {
    ensure_scope(&ctx, SCOPE_CONTAINERS_WRITE)?;
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
    Extension(ctx): Extension<AuthContext>,
    Path(container_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<AppInstance>>, StatusCode> {
    ensure_scope(&ctx, SCOPE_CONTAINERS_READ)?;
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
    Extension(ctx): Extension<AuthContext>,
    Path(container_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<AppInstallRequest>,
) -> Result<Json<TaskModel>, (StatusCode, String)> {
    ensure_scope(&ctx, SCOPE_CONTAINERS_WRITE).map_err(forbidden_with_message)?;
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
    Extension(ctx): Extension<AuthContext>,
    Path(app_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<LaunchAppRequest>,
) -> Result<Json<TaskModel>, StatusCode> {
    ensure_scope(&ctx, SCOPE_CONTAINERS_WRITE)?;
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
    Extension(ctx): Extension<AuthContext>,
    Path(container_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Snapshot>>, StatusCode> {
    ensure_scope(&ctx, SCOPE_CONTAINERS_READ)?;
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
    Extension(ctx): Extension<AuthContext>,
    Path(container_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<SnapshotRequest>,
) -> Result<Json<TaskModel>, (StatusCode, String)> {
    ensure_scope(&ctx, SCOPE_CONTAINERS_WRITE).map_err(forbidden_with_message)?;
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
    Extension(ctx): Extension<AuthContext>,
    Path(snapshot_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<TaskModel>, StatusCode> {
    ensure_scope(&ctx, SCOPE_CONTAINERS_WRITE)?;
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
    Extension(ctx): Extension<AuthContext>,
    Query(params): Query<TasksQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<TaskModel>>, StatusCode> {
    ensure_scope(&ctx, SCOPE_TASKS_READ)?;
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
    Extension(ctx): Extension<AuthContext>,
    Path(task_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<TaskModel>, StatusCode> {
    ensure_scope(&ctx, SCOPE_TASKS_READ)?;
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
    Extension(ctx): Extension<AuthContext>,
    State(state): State<AppState>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    ensure_scope(&ctx, SCOPE_TASKS_READ)?;
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

    Ok(Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(StdDuration::from_secs(10))
            .text("keep-alive"),
    ))
}

#[derive(Deserialize)]
struct CreateTokenRequest {
    name: String,
    scopes: Option<Vec<String>>,
    expires_at: Option<String>,
}

#[derive(Serialize)]
struct CreateTokenResponse {
    token: String,
    #[serde(flatten)]
    info: ApiTokenInfo,
}

async fn list_api_tokens(
    Extension(ctx): Extension<AuthContext>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ApiTokenInfo>>, StatusCode> {
    ensure_admin(&ctx)?;
    state.tokens.list().await.map(Json).map_err(|err| {
        tracing::error!(?err, "No se pudieron listar tokens");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn create_api_token(
    Extension(ctx): Extension<AuthContext>,
    State(state): State<AppState>,
    Json(payload): Json<CreateTokenRequest>,
) -> Result<(StatusCode, Json<CreateTokenResponse>), (StatusCode, String)> {
    ensure_admin(&ctx).map_err(forbidden_with_message)?;
    let CreateTokenRequest {
        name,
        scopes,
        expires_at,
    } = payload;

    if name.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "El nombre es obligatorio".into()));
    }

    let scopes = scopes.unwrap_or_else(|| vec!["containers:read".into(), "tasks:read".into()]);
    let expires_at = parse_expiration(expires_at)?;
    let spec = TokenSpec {
        name: name.trim().to_string(),
        scopes,
        expires_at,
    };

    let issued = state.tokens.issue(spec).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("No se pudo emitir token: {err}"),
        )
    })?;

    let response = CreateTokenResponse {
        token: issued.secret,
        info: issued.info,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

async fn revoke_api_token(
    Extension(ctx): Extension<AuthContext>,
    Path(token_id): Path<Uuid>,
    State(state): State<AppState>,
) -> StatusCode {
    if let Err(status) = ensure_admin(&ctx) {
        return status;
    }
    match state.tokens.revoke(token_id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(err) => {
            tracing::error!(?err, token_id = %token_id, "No se pudo revocar token");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

fn ensure_admin(ctx: &AuthContext) -> Result<(), StatusCode> {
    match ctx {
        AuthContext::Admin => Ok(()),
        other => {
            tracing::warn!(?other, "Intento de acceso admin sin privilegios");
            Err(StatusCode::FORBIDDEN)
        }
    }
}

fn ensure_scope(ctx: &AuthContext, scope: &str) -> Result<(), StatusCode> {
    if has_scope(ctx, scope) {
        Ok(())
    } else {
        tracing::warn!(?ctx, scope, "Token sin scope requerido");
        Err(StatusCode::FORBIDDEN)
    }
}

fn has_scope(ctx: &AuthContext, scope: &str) -> bool {
    match ctx {
        AuthContext::Admin | AuthContext::StaticToken { .. } => true,
        AuthContext::ServiceToken { token } => token.scopes.iter().any(|s| s == scope),
    }
}

fn forbidden_with_message(status: StatusCode) -> (StatusCode, String) {
    (
        status,
        "Permisos insuficientes para la operacion solicitada".into(),
    )
}

fn parse_expiration(raw: Option<String>) -> Result<Option<OffsetDateTime>, (StatusCode, String)> {
    match raw {
        Some(value) => {
            let parsed = OffsetDateTime::parse(&value, &Rfc3339).map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "expires_at debe seguir el formato RFC3339".into(),
                )
            })?;
            if parsed <= OffsetDateTime::now_utc() {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "expires_at debe ser una fecha futura".into(),
                ));
            }
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}

fn expires_within(token: &ApiTokenInfo, window: Duration) -> bool {
    token
        .expires_at
        .as_deref()
        .and_then(parse_rfc3339_timestamp)
        .map(|expires| {
            let now = OffsetDateTime::now_utc();
            expires > now && expires <= now + window
        })
        .unwrap_or(false)
}

fn parse_rfc3339_timestamp(value: &str) -> Option<OffsetDateTime> {
    OffsetDateTime::parse(value, &Rfc3339).ok()
}

fn build_security_response(
    snapshot: &SecuritySnapshot,
    tokens: &[ApiTokenInfo],
) -> SecurityReloadResponse {
    let expiring_window = Duration::hours(24);
    let expiring_token_count = tokens
        .iter()
        .filter(|token| expires_within(token, expiring_window))
        .count();
    let scopes_catalog = tokens
        .iter()
        .flat_map(|token| token.scopes.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    SecurityReloadResponse {
        auth_enabled: snapshot.auth_enabled,
        admin_token_present: snapshot.admin_token_present,
        static_token_count: snapshot.static_token_count,
        managed_token_count: snapshot.managed_token_count.max(0) as u64,
        expiring_token_count: expiring_token_count as u64,
        scopes_catalog,
    }
}
