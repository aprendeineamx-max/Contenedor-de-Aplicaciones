use std::path::PathBuf;

use serde::Deserialize;

/// Configuración básica del agente cargada desde variables de entorno.
#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    pub containers_root: PathBuf,
    pub telemetry_level: String,
}

impl AgentConfig {
    pub fn from_env() -> Self {
        let containers_root = std::env::var("ORBIT_CONTAINERS_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("sandboxes"));

        let telemetry_level = std::env::var("ORBIT_LOG").unwrap_or_else(|_| "info".to_string());

        Self {
            containers_root,
            telemetry_level,
        }
    }
}
