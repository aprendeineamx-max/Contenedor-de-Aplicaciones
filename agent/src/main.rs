mod config;
mod events;
mod telemetry;
mod virtualization;

use anyhow::Result;
use config::AgentConfig;
use events::{AgentEvent, emit};
use tracing::{info, instrument};
use virtualization::{Platform, SandboxDescriptor, SandboxRuntime};

struct Agent {
    config: AgentConfig,
}

impl Agent {
    pub fn new(config: AgentConfig) -> Self {
        Self { config }
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
        emit(AgentEvent::TaskCreated {
            id: task_id,
            task_type: "container.create".into(),
            status: "running".into(),
        });

        let sandbox = SandboxRuntime::new(descriptor);
        sandbox.prepare().await?;
        emit(AgentEvent::TaskProgress {
            id: task_id,
            progress: 45,
            message: "Filesystem/registry preparados".into(),
        });

        sandbox.persist_manifest().await?;
        emit(AgentEvent::TaskProgress {
            id: task_id,
            progress: 90,
            message: "Manifest creado".into(),
        });

        emit(AgentEvent::ContainerStatus {
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

    let agent = Agent::new(config);
    agent.bootstrap().await?;

    info!("PoC lista. Manteniendo proceso vivo (Ctrl+C para salir).");
    tokio::signal::ctrl_c().await?;
    info!("Señal de apagado recibida, cerrando agente.");
    Ok(())
}
