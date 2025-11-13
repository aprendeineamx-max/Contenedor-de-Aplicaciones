use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::models::{ContainerModel, TaskModel};

#[derive(Clone)]
pub struct InMemoryStore {
    containers: Arc<RwLock<HashMap<Uuid, ContainerModel>>>,
    tasks: Arc<RwLock<HashMap<Uuid, TaskModel>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            containers: Arc::new(RwLock::new(HashMap::new())),
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn upsert_container(&self, container: ContainerModel) {
        self.containers
            .write()
            .await
            .insert(container.id, container);
    }

    pub async fn list_containers(&self) -> Vec<ContainerModel> {
        self.containers.read().await.values().cloned().collect()
    }

    pub async fn upsert_task(&self, task: TaskModel) {
        self.tasks.write().await.insert(task.id, task);
    }

    pub async fn list_tasks(&self) -> Vec<TaskModel> {
        self.tasks.read().await.values().cloned().collect()
    }

    pub async fn get_task(&self, id: Uuid) -> Option<TaskModel> {
        self.tasks.read().await.get(&id).cloned()
    }
}
