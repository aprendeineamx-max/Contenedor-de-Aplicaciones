mod config;
mod events;
mod server;
mod telemetry;
mod virtualization;

use anyhow::Result;
use config::AgentConfig;
use events::{AgentEvent, EventHub};
use server::AppState;
use tokio::sync::oneshot;
use tracing::{info, instrument};
use virtualization::{Platform, SandboxDescriptor, SandboxRuntime};

struct Agent {
    config: AgentConfig,
    events: EventHub,
}

impl Agent {
    pub fn new(config: AgentConfig, events: EventHub) -> Self {
        Self { config, events }
    }

    #[instrument(skip(self))]
    pub async fn bootstrap(&self) -> Result<()> {
        tokio::fs::create_dir_all(&self.config.containers_root).await?;

        let descriptor = SandboxDescriptor::new(
            "chrome-poc",
            Platform::WindowsX64,
            self.config.containers_root.join("chrome-poc"),
        );
        let task_id = descriptor.container_id;
        self.events.emit(AgentEvent::TaskCreated {
            id: task_id,
            task_type: "container.create".into(),
            status: "running".into(),
        });

        let sandbox = SandboxRuntime::new(descriptor);
        sandbox.prepare().await?;
        self.events.emit(AgentEvent::TaskProgress {
            id: task_id,
            progress: 45,
            message: "Filesystem/registry preparados".into(),
        });

        sandbox.persist_manifest().await?;
        self.events.emit(AgentEvent::TaskProgress {
            id: task_id,
            progress: 90,
            message: "Manifest creado".into(),
        });

        self.events.emit(AgentEvent::ContainerStatus {
            container_id: task_id,
            status: "ready".into(),
        });

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
    let agent = Agent::new(config.clone(), events.clone());
    let app_state = AppState::new(config.clone(), events.clone());

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
