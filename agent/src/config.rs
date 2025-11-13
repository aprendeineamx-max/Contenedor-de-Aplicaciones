use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf};

/// Configuración básica del agente cargada desde variables de entorno.
#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    pub containers_root: PathBuf,
    pub telemetry_level: String,
    pub api_bind: SocketAddr,
    pub database_path: PathBuf,
}

impl AgentConfig {
    pub fn from_env() -> Self {
        let containers_root = std::env::var("ORBIT_CONTAINERS_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("sandboxes"));

        let telemetry_level = std::env::var("ORBIT_LOG").unwrap_or_else(|_| "info".to_string());

        let api_bind = std::env::var("ORBIT_API_BIND")
            .unwrap_or_else(|_| "127.0.0.1:7443".to_string())
            .parse()
            .expect("ORBIT_API_BIND debe tener formato ip:puerto");

        let database_path = std::env::var("ORBIT_DB_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("orbit-data/agent.db"));

        Self {
            containers_root,
            telemetry_level,
            api_bind,
            database_path,
        }
    }
}
