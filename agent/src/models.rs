use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::virtualization::Platform;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContainerStatus {
    Creating,
    Ready,
    Running,
    Error,
    Archived,
}

impl ContainerStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContainerStatus::Creating => "creating",
            ContainerStatus::Ready => "ready",
            ContainerStatus::Running => "running",
            ContainerStatus::Error => "error",
            ContainerStatus::Archived => "archived",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "creating" => ContainerStatus::Creating,
            "running" => ContainerStatus::Running,
            "error" => ContainerStatus::Error,
            "archived" => ContainerStatus::Archived,
            _ => ContainerStatus::Ready,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ContainerModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: ContainerStatus,
    pub platform: Platform,
    pub tags: Vec<String>,
    pub size_bytes: u64,
    pub created_at: String,
    pub updated_at: String,
}

impl ContainerModel {
    pub fn new(id: Uuid, name: String, description: Option<String>, platform: Platform) -> Self {
        let timestamp = current_timestamp();
        Self {
            id,
            name,
            description,
            status: ContainerStatus::Ready,
            platform,
            tags: vec![],
            size_bytes: 0,
            created_at: timestamp.clone(),
            updated_at: timestamp,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = current_timestamp();
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Queued => "queued",
            TaskStatus::Running => "running",
            TaskStatus::Succeeded => "succeeded",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "running" => TaskStatus::Running,
            "succeeded" => TaskStatus::Succeeded,
            "failed" => TaskStatus::Failed,
            "cancelled" => TaskStatus::Cancelled,
            _ => TaskStatus::Queued,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TaskModel {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub task_type: String,
    pub status: TaskStatus,
    pub progress: u8,
    pub message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl TaskModel {
    pub fn new(task_type: impl Into<String>) -> Self {
        let timestamp = current_timestamp();
        Self {
            id: Uuid::new_v4(),
            task_type: task_type.into(),
            status: TaskStatus::Queued,
            progress: 0,
            message: None,
            created_at: timestamp.clone(),
            updated_at: timestamp,
        }
    }

    pub fn with_status(mut self, status: TaskStatus) -> Self {
        self.status = status;
        self
    }

    pub fn set_progress(&mut self, progress: u8, message: impl Into<Option<String>>) {
        self.progress = progress;
        self.message = message.into();
        self.touch();
    }

    pub fn touch(&mut self) {
        self.updated_at = current_timestamp();
    }
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct AppEntryPoint {
    pub id: String,
    pub label: String,
    pub command: String,
    pub icon: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppStatus {
    Installing,
    Ready,
    Failed,
    Disabled,
}

impl AppStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppStatus::Installing => "installing",
            AppStatus::Ready => "ready",
            AppStatus::Failed => "failed",
            AppStatus::Disabled => "disabled",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "installing" => AppStatus::Installing,
            "failed" => AppStatus::Failed,
            "disabled" => AppStatus::Disabled,
            _ => AppStatus::Ready,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AppInstance {
    pub id: Uuid,
    pub container_id: Uuid,
    pub name: String,
    pub version: Option<String>,
    pub status: AppStatus,
    pub entry_points: Vec<AppEntryPoint>,
    pub created_at: String,
    pub updated_at: String,
}

impl AppInstance {
    pub fn new(container_id: Uuid, name: String, version: Option<String>) -> Self {
        let timestamp = current_timestamp();
        Self {
            id: Uuid::new_v4(),
            container_id,
            name,
            version,
            status: AppStatus::Ready,
            entry_points: vec![],
            created_at: timestamp.clone(),
            updated_at: timestamp,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = current_timestamp();
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SnapshotType {
    Full,
    Delta,
}

impl SnapshotType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SnapshotType::Full => "full",
            SnapshotType::Delta => "delta",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "delta" => SnapshotType::Delta,
            _ => SnapshotType::Full,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: Uuid,
    pub container_id: Uuid,
    pub label: Option<String>,
    pub snapshot_type: SnapshotType,
    pub base_snapshot_id: Option<Uuid>,
    pub size_bytes: u64,
    pub created_at: String,
}

impl Snapshot {
    pub fn new(container_id: Uuid, label: Option<String>, snapshot_type: SnapshotType) -> Self {
        Self {
            id: Uuid::new_v4(),
            container_id,
            label,
            snapshot_type,
            base_snapshot_id: None,
            size_bytes: 0,
            created_at: current_timestamp(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApiTokenInfo {
    pub id: Uuid,
    pub name: String,
    pub prefix: String,
    pub created_at: String,
    pub revoked_at: Option<String>,
}

fn current_timestamp() -> String {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".into())
}
