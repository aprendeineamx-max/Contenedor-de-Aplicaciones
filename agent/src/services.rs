use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::sync::Mutex;

use crate::{
    config::AgentConfig,
    events::{AgentEvent, EventHub},
    models::{ContainerModel, TaskModel, TaskStatus},
    store::InMemoryStore,
    virtualization::{Platform, SandboxDescriptor, SandboxRuntime},
};

#[derive(Clone)]
pub struct ContainerService {
    inner: Arc<ContainerServiceInner>,
}

struct ContainerServiceInner {
    config: AgentConfig,
    events: EventHub,
    store: InMemoryStore,
    mutex: Mutex<()>,
}

impl ContainerService {
    pub fn new(config: AgentConfig, events: EventHub, store: InMemoryStore) -> Self {
        Self {
            inner: Arc::new(ContainerServiceInner {
                config,
                events,
                store,
                mutex: Mutex::new(()),
            }),
        }
    }

    pub async fn create_container(
        &self,
        name: String,
        platform: Platform,
        description: Option<String>,
    ) -> Result<TaskModel> {
        let _guard = self.inner.mutex.lock().await;

        let mut task = TaskModel::new("container.create").with_status(TaskStatus::Running);
        task.set_progress(5, Some("Inicializando creación".to_string()));
        self.inner.store.upsert_task(task.clone()).await;
        self.inner.events.emit(AgentEvent::TaskCreated {
            id: task.id,
            task_type: task.task_type.clone(),
            status: "running".into(),
        });

        let sanitized = name.replace(['/', '\\'], "_");
        let sandbox_root = self.inner.config.containers_root.join(&sanitized);

        let descriptor = SandboxDescriptor::new(name.clone(), platform.clone(), &sandbox_root);
        let sandbox = SandboxRuntime::new(descriptor);

        if let Err(err) = sandbox
            .prepare()
            .await
            .context("No se pudo preparar el filesystem del contenedor")
        {
            self.fail_task(task.clone(), err.to_string()).await;
            return Err(err);
        }
        task.set_progress(40, Some("Filesystem y registro preparados".to_string()));
        self.inner.store.upsert_task(task.clone()).await;
        self.inner.events.emit(AgentEvent::TaskProgress {
            id: task.id,
            progress: 40,
            message: "Filesystem/registry preparados".into(),
        });

        if let Err(err) = sandbox
            .persist_manifest()
            .await
            .context("No se pudo persistir el manifest del contenedor")
        {
            self.fail_task(task.clone(), err.to_string()).await;
            return Err(err);
        }
        task.set_progress(80, Some("Manifest generado".to_string()));
        self.inner.store.upsert_task(task.clone()).await;
        self.inner.events.emit(AgentEvent::TaskProgress {
            id: task.id,
            progress: 80,
            message: "Manifest creado".into(),
        });

        let descriptor = sandbox.descriptor().clone();
        let mut container =
            ContainerModel::new(descriptor.container_id, name.clone(), description, platform);
        container.touch();
        self.inner.store.upsert_container(container.clone()).await;

        task.status = TaskStatus::Succeeded;
        task.set_progress(100, Some("Contenedor listo".to_string()));
        self.inner.store.upsert_task(task.clone()).await;
        self.inner.events.emit(AgentEvent::TaskProgress {
            id: task.id,
            progress: 100,
            message: "Contenedor listo".into(),
        });
        self.inner.events.emit(AgentEvent::ContainerStatus {
            container_id: descriptor.container_id,
            status: "ready".into(),
        });

        Ok(task)
    }

    pub async fn fail_task(&self, mut task: TaskModel, error: String) -> TaskModel {
        task.status = TaskStatus::Failed;
        task.set_progress(task.progress, Some(error));
        self.inner.store.upsert_task(task.clone()).await;
        task
    }
}
