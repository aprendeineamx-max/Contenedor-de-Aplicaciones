use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf};

/// Configuracion basica del agente cargada desde variables de entorno.
#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    pub containers_root: PathBuf,
    pub telemetry_level: String,
    pub api_bind: SocketAddr,
    pub database_path: PathBuf,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SecurityConfig {
    pub auth_enabled: bool,
    pub admin_token: Option<String>,
    #[serde(default)]
    pub api_tokens: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            auth_enabled: false,
            admin_token: None,
            api_tokens: Vec::new(),
        }
    }
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

        let security = SecurityConfig::from_env();

        Self {
            containers_root,
            telemetry_level,
            api_bind,
            database_path,
            security,
        }
    }
}

impl SecurityConfig {
    pub fn from_env() -> Self {
        let auth_enabled = std::env::var("ORBIT_AUTH_ENABLED")
            .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE"))
            .unwrap_or(false);

        let admin_token = std::env::var("ORBIT_ADMIN_TOKEN").ok();

        let api_tokens = std::env::var("ORBIT_API_TOKENS")
            .map(|raw| {
                raw.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        Self {
            auth_enabled,
            admin_token,
            api_tokens,
        }
    }
}
