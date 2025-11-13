use std::{path::Path, sync::Arc};

use anyhow::{Context, Result};
use rand::{Rng, distr::Alphanumeric, rng};
use tokio::{fs, sync::Mutex};
use uuid::Uuid;

use crate::{
    config::AgentConfig,
    events::{AgentEvent, EventHub},
    models::{
        ApiTokenInfo, AppInstance, ContainerModel, Snapshot, SnapshotType, TaskModel, TaskStatus,
    },
    security::hash_token,
    store::SqliteStore,
    virtualization::{Platform, SandboxDescriptor, SandboxRuntime},
};

#[derive(Clone)]
pub struct ContainerService {
    inner: Arc<ContainerServiceInner>,
}

struct ContainerServiceInner {
    config: AgentConfig,
    events: EventHub,
    store: SqliteStore,
    mutex: Mutex<()>,
}

impl ContainerService {
    pub fn new(config: AgentConfig, events: EventHub, store: SqliteStore) -> Self {
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
        task.set_progress(5, Some("Inicializando creacion".into()));
        self.inner.store.upsert_task(&task).await?;
        self.inner.events.emit(AgentEvent::TaskCreated {
            id: task.id,
            task_type: task.task_type.clone(),
            status: "running".into(),
        });

        let sandbox_root = container_root(&self.inner.config.containers_root, &name);
        let descriptor = SandboxDescriptor::new(name.clone(), platform.clone(), sandbox_root);
        let sandbox = SandboxRuntime::new(descriptor);

        sandbox
            .prepare()
            .await
            .context("No se pudo preparar el filesystem del contenedor")?;
        task.set_progress(40, Some("Filesystem/registry preparados".into()));
        self.inner.store.upsert_task(&task).await?;
        self.inner.events.emit(AgentEvent::TaskProgress {
            id: task.id,
            progress: 40,
            message: "Filesystem/registry preparados".into(),
        });

        sandbox
            .persist_manifest()
            .await
            .context("No se pudo persistir el manifest del contenedor")?;
        task.set_progress(80, Some("Manifest generado".into()));
        self.inner.store.upsert_task(&task).await?;
        self.inner.events.emit(AgentEvent::TaskProgress {
            id: task.id,
            progress: 80,
            message: "Manifest creado".into(),
        });

        let descriptor = sandbox.descriptor().clone();
        let mut container =
            ContainerModel::new(descriptor.container_id, name, description, platform);
        container.touch();
        self.inner.store.upsert_container(&container).await?;

        task.status = TaskStatus::Succeeded;
        task.set_progress(100, Some("Contenedor listo".into()));
        self.inner.store.upsert_task(&task).await?;
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

    pub async fn get_container(&self, id: Uuid) -> Result<Option<ContainerModel>> {
        self.inner.store.get_container(id).await
    }

    pub async fn delete_container(&self, id: Uuid) -> Result<Option<TaskModel>> {
        let Some(container) = self.inner.store.get_container(id).await? else {
            return Ok(None);
        };

        let mut task = TaskModel::new("container.delete").with_status(TaskStatus::Running);
        task.set_progress(5, Some("Eliminando contenedor".into()));
        self.inner.store.upsert_task(&task).await?;

        let sandbox_root = container_root(&self.inner.config.containers_root, &container.name);
        if fs::metadata(&sandbox_root).await.is_ok() {
            if let Err(err) = fs::remove_dir_all(&sandbox_root).await {
                tracing::warn!(
                    ?err,
                    ?sandbox_root,
                    "No se pudo eliminar el directorio del contenedor"
                );
            }
        }

        self.inner.store.delete_container(id).await?;
        task.status = TaskStatus::Succeeded;
        task.set_progress(100, Some("Contenedor eliminado".into()));
        self.inner.store.upsert_task(&task).await?;
        self.inner.events.emit(AgentEvent::TaskProgress {
            id: task.id,
            progress: 100,
            message: "Contenedor eliminado".into(),
        });
        self.inner.events.emit(AgentEvent::ContainerStatus {
            container_id: id,
            status: "archived".into(),
        });

        Ok(Some(task))
    }
}

#[derive(Clone)]
pub struct AppService {
    events: EventHub,
    store: SqliteStore,
}

impl AppService {
    pub fn new(events: EventHub, store: SqliteStore) -> Self {
        Self { events, store }
    }

    pub async fn list(&self, container_id: Uuid) -> Result<Vec<AppInstance>> {
        self.store.list_apps(container_id).await
    }

    pub async fn install(
        &self,
        container_id: Uuid,
        name: String,
        version: Option<String>,
    ) -> Result<TaskModel> {
        let mut task = TaskModel::new("app.install").with_status(TaskStatus::Running);
        task.set_progress(20, Some("Iniciando instalacion".into()));
        self.store.upsert_task(&task).await?;
        self.events.emit(AgentEvent::TaskCreated {
            id: task.id,
            task_type: task.task_type.clone(),
            status: "running".into(),
        });

        let mut app = AppInstance::new(container_id, name, version);
        app.touch();
        self.store.insert_app(&app).await?;

        task.status = TaskStatus::Succeeded;
        task.set_progress(100, Some("Aplicacion instalada".into()));
        self.store.upsert_task(&task).await?;
        self.events.emit(AgentEvent::TaskProgress {
            id: task.id,
            progress: 100,
            message: "Aplicacion instalada".into(),
        });

        Ok(task)
    }

    pub async fn launch(&self, app_id: Uuid) -> Result<Option<TaskModel>> {
        let Some(app) = self.store.get_app(app_id).await? else {
            return Ok(None);
        };
        let mut task = TaskModel::new("app.launch").with_status(TaskStatus::Running);
        task.set_progress(10, Some(format!("Lanzando {}", app.name)));
        self.store.upsert_task(&task).await?;
        task.status = TaskStatus::Succeeded;
        task.set_progress(100, Some("Aplicacion lanzada".into()));
        self.store.upsert_task(&task).await?;
        Ok(Some(task))
    }
}

#[derive(Clone)]
pub struct SnapshotService {
    store: SqliteStore,
    events: EventHub,
}

impl SnapshotService {
    pub fn new(events: EventHub, store: SqliteStore) -> Self {
        Self { store, events }
    }

    pub async fn list(&self, container_id: Uuid) -> Result<Vec<Snapshot>> {
        self.store.list_snapshots(container_id).await
    }

    pub async fn create(
        &self,
        container_id: Uuid,
        label: Option<String>,
        snapshot_type: SnapshotType,
    ) -> Result<TaskModel> {
        let mut task = TaskModel::new("snapshot.create").with_status(TaskStatus::Running);
        task.set_progress(25, Some("Capturando snapshot".into()));
        self.store.upsert_task(&task).await?;
        self.events.emit(AgentEvent::TaskCreated {
            id: task.id,
            task_type: task.task_type.clone(),
            status: "running".into(),
        });

        let mut snapshot = Snapshot::new(container_id, label, snapshot_type);
        snapshot.size_bytes = 0;
        self.store.insert_snapshot(&snapshot).await?;

        task.status = TaskStatus::Succeeded;
        task.set_progress(100, Some("Snapshot creado".into()));
        self.store.upsert_task(&task).await?;
        self.events.emit(AgentEvent::TaskProgress {
            id: task.id,
            progress: 100,
            message: "Snapshot creado".into(),
        });
        Ok(task)
    }

    pub async fn restore(&self, snapshot_id: Uuid) -> Result<Option<TaskModel>> {
        let Some(_snapshot) = self.store.get_snapshot(snapshot_id).await? else {
            return Ok(None);
        };
        let mut task = TaskModel::new("snapshot.restore").with_status(TaskStatus::Running);
        task.set_progress(30, Some("Preparando restauracion".into()));
        self.store.upsert_task(&task).await?;

        task.status = TaskStatus::Succeeded;
        task.set_progress(100, Some("Snapshot restaurado".into()));
        self.store.upsert_task(&task).await?;
        self.events.emit(AgentEvent::TaskProgress {
            id: task.id,
            progress: 100,
            message: "Snapshot restaurado".into(),
        });
        Ok(Some(task))
    }
}

fn container_root(root: &Path, name: &str) -> std::path::PathBuf {
    let sanitized = name
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect::<String>();
    root.join(sanitized)
}

#[derive(Clone)]
pub struct TokenService {
    store: SqliteStore,
}

pub struct IssuedToken {
    pub secret: String,
    pub info: ApiTokenInfo,
}

impl TokenService {
    pub fn new(store: SqliteStore) -> Self {
        Self { store }
    }

    pub async fn issue(&self, name: String) -> Result<IssuedToken> {
        let secret = generate_service_token();
        let prefix = token_prefix(&secret);
        let hash = hash_token(&secret);
        let info = self.store.create_api_token(name, hash, prefix).await?;

        Ok(IssuedToken { secret, info })
    }

    pub async fn list(&self) -> Result<Vec<ApiTokenInfo>> {
        self.store.list_api_tokens().await
    }

    pub async fn revoke(&self, id: Uuid) -> Result<bool> {
        self.store.revoke_api_token(id).await
    }
}

fn generate_service_token() -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(48)
        .map(char::from)
        .collect()
}

fn token_prefix(token: &str) -> String {
    token.chars().take(8).collect()
}
