use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ProcessLauncher {
    workspace: PathBuf,
}

impl ProcessLauncher {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            workspace: root.as_ref().to_path_buf(),
        }
    }

    pub fn runtime_env(&self, container_id: Uuid) -> RuntimeEnv {
        let mut env = HashMap::new();
        env.insert("ORBIT_CONTAINER_ID".to_string(), container_id.to_string());
        env.insert(
            "ORBIT_RUNTIME_ROOT".to_string(),
            self.workspace.display().to_string(),
        );

        RuntimeEnv { env }
    }
}

#[derive(Debug, Serialize)]
pub struct RuntimeEnv {
    pub env: HashMap<String, String>,
}
