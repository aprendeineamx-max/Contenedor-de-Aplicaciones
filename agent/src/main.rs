use agent::{
    config::AgentConfig,
    events::EventHub,
    security::AuthManager,
    server::{self, AppState},
    services::{AppService, ContainerService, SnapshotService, TokenService},
    store::SqliteStore,
    telemetry,
    virtualization::Platform,
};
use anyhow::Result;
use tokio::sync::oneshot;
use tracing::info;

struct Agent {
    containers: ContainerService,
}

impl Agent {
    pub fn new(containers: ContainerService) -> Self {
        Self { containers }
    }

    pub async fn bootstrap(&self) -> Result<()> {
        let _ = self
            .containers
            .create_container(
                "chrome-poc".into(),
                Platform::WindowsX64,
                Some("Contenedor de demostracion inicial".into()),
            )
            .await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = AgentConfig::from_env();
    telemetry::init(&config.telemetry_level)?;

    info!(
        "Inicializando agente. Root de contenedores: {}",
        config.containers_root.display()
    );

    let events = EventHub::new(128);
    let store = SqliteStore::new(&config.database_path).await?;
    let container_service = ContainerService::new(config.clone(), events.clone(), store.clone());
    let app_service = AppService::new(events.clone(), store.clone());
    let snapshot_service = SnapshotService::new(events.clone(), store.clone());
    let token_service = TokenService::new(store.clone());
    let auth_manager = AuthManager::new(config.security.clone(), store.clone());

    let agent = Agent::new(container_service.clone());
    let app_state = AppState::new(
        config.clone(),
        events.clone(),
        store.clone(),
        container_service.clone(),
        app_service,
        snapshot_service,
        token_service.clone(),
        auth_manager.clone(),
    );

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let server_handle = tokio::spawn(async move {
        if let Err(err) = server::serve(app_state, shutdown_rx).await {
            tracing::error!(?err, "Servidor API detenido con error");
        }
    });

    agent.bootstrap().await?;

    info!("PoC lista. API disponible en {}", config.api_bind);
    tokio::signal::ctrl_c().await?;
    info!("Senal de apagado recibida, cerrando agente.");
    let _ = shutdown_tx.send(());
    let _ = server_handle.await;
    Ok(())
}
