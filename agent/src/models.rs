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

fn current_timestamp() -> String {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".into())
}
