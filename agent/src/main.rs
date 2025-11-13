mod config;
mod events;
mod models;
mod server;
mod services;
mod store;
mod telemetry;
mod virtualization;

use anyhow::Result;
use config::AgentConfig;
use events::EventHub;
use server::AppState;
use services::ContainerService;
use store::InMemoryStore;
use tokio::sync::oneshot;
use tracing::info;
use virtualization::Platform;

struct Agent {
    containers: ContainerService,
}

impl Agent {
    pub fn new(containers: ContainerService) -> Self {
        Self { containers }
    }

    pub async fn bootstrap(&self) -> Result<()> {
        // Crear un contenedor de demostración para validar el flujo end-to-end.
        let _ = self
            .containers
            .create_container(
                "chrome-poc".into(),
                Platform::WindowsX64,
                Some("Contenedor de demostración inicial".into()),
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
    let store = InMemoryStore::new();
    let container_service = ContainerService::new(config.clone(), events.clone(), store.clone());

    let agent = Agent::new(container_service.clone());
    let app_state = AppState::new(
        config.clone(),
        events.clone(),
        store.clone(),
        container_service.clone(),
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
    info!("Señal de apagado recibida, cerrando agente.");
    let _ = shutdown_tx.send(());
    let _ = server_handle.await;
    Ok(())
}
