use serde::{Deserialize, Serialize};
use std::{
    fs,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::OnceLock,
};

/// Configuracion basica del agente cargada desde variables de entorno.
#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    pub containers_root: PathBuf,
    pub telemetry_level: String,
    pub api_bind: SocketAddr,
    pub database_path: PathBuf,
    pub security: SecurityConfig,
}

static CONFIG_SOURCES: OnceLock<ConfigSources> = OnceLock::new();

#[derive(Debug, Clone, Default)]
pub struct ConfigSources {
    pub defaults_file: Option<PathBuf>,
    pub local_file: Option<PathBuf>,
    pub env_overrides: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfigSourcesView {
    pub defaults_file: Option<String>,
    pub local_file: Option<String>,
    pub env_overrides: Vec<String>,
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
        let (file_config, mut sources) = FileConfig::load();
        let containers_root = env_path("ORBIT_CONTAINERS_ROOT", &mut sources.env_overrides)
            .or(file_config.containers_root)
            .unwrap_or_else(|| PathBuf::from("sandboxes"));

        let telemetry_level = env_string("ORBIT_LOG", &mut sources.env_overrides)
            .or(file_config.telemetry_level)
            .unwrap_or_else(|| "info".into());

        let api_bind = env_string("ORBIT_API_BIND", &mut sources.env_overrides)
            .or(file_config.api_bind)
            .unwrap_or_else(|| "127.0.0.1:7443".into())
            .parse()
            .expect("ORBIT_API_BIND debe tener formato ip:puerto");

        let database_path = env_path("ORBIT_DB_PATH", &mut sources.env_overrides)
            .or(file_config.database_path)
            .unwrap_or_else(|| PathBuf::from("orbit-data/agent.db"));

        let security =
            SecurityConfig::from_layers(file_config.security, &mut sources.env_overrides);

        CONFIG_SOURCES.get_or_init(|| sources.clone());

        AgentConfig {
            containers_root,
            telemetry_level,
            api_bind,
            database_path,
            security,
        }
    }

    pub fn snapshot(&self) -> ConfigSnapshot {
        ConfigSnapshot {
            containers_root: self.containers_root.display().to_string(),
            telemetry_level: self.telemetry_level.clone(),
            api_bind: self.api_bind.to_string(),
            database_path: self.database_path.display().to_string(),
            security: ConfigSecurityView {
                auth_enabled: self.security.auth_enabled,
                admin_token_present: self.security.admin_token.is_some(),
                static_tokens: self.security.api_tokens.len() as u64,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfigSnapshot {
    pub containers_root: String,
    pub telemetry_level: String,
    pub api_bind: String,
    pub database_path: String,
    pub security: ConfigSecurityView,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfigSecurityView {
    pub auth_enabled: bool,
    pub admin_token_present: bool,
    pub static_tokens: u64,
}

impl SecurityConfig {
    pub fn from_env() -> Self {
        Self::from_layers(None, &mut Vec::new())
    }

    fn from_layers(file: Option<FileSecurityConfig>, env_overrides: &mut Vec<String>) -> Self {
        let file = file.unwrap_or_default();
        let auth_enabled = env_bool("ORBIT_AUTH_ENABLED", env_overrides)
            .or(file.auth_enabled)
            .unwrap_or(false);

        let admin_token = env_string("ORBIT_ADMIN_TOKEN", env_overrides).or(file.admin_token);

        let api_tokens = env_string("ORBIT_API_TOKENS", env_overrides)
            .map(parse_token_list)
            .or(file.api_tokens)
            .unwrap_or_default();

        SecurityConfig {
            auth_enabled,
            admin_token,
            api_tokens,
        }
    }
}

pub fn config_sources() -> ConfigSources {
    CONFIG_SOURCES.get().cloned().unwrap_or_default()
}

pub fn config_sources_view() -> ConfigSourcesView {
    let sources = config_sources();
    ConfigSourcesView {
        defaults_file: sources.defaults_file.map(|path| path.display().to_string()),
        local_file: sources.local_file.map(|path| path.display().to_string()),
        env_overrides: sources.env_overrides,
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
struct FileSecurityConfig {
    auth_enabled: Option<bool>,
    admin_token: Option<String>,
    api_tokens: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct FileConfig {
    containers_root: Option<PathBuf>,
    telemetry_level: Option<String>,
    api_bind: Option<String>,
    database_path: Option<PathBuf>,
    security: Option<FileSecurityConfig>,
}

impl FileConfig {
    fn load() -> (Self, ConfigSources) {
        let mut merged = FileConfig::default();
        let mut defaults_file = None;
        let defaults_path = Path::new("config/orbit.toml");
        if let Some(cfg) = load_file(defaults_path) {
            merged.merge(cfg);
            defaults_file = Some(defaults_path.to_path_buf());
        }

        let mut local_file = None;
        let local_path = Path::new("orbit-data/config.local.toml");
        if let Some(cfg) = load_file(local_path) {
            merged.merge(cfg);
            local_file = Some(local_path.to_path_buf());
        }

        (
            merged,
            ConfigSources {
                defaults_file,
                local_file,
                env_overrides: Vec::new(),
            },
        )
    }

    fn merge(&mut self, other: FileConfig) {
        if other.containers_root.is_some() {
            self.containers_root = other.containers_root;
        }
        if other.telemetry_level.is_some() {
            self.telemetry_level = other.telemetry_level;
        }
        if other.api_bind.is_some() {
            self.api_bind = other.api_bind;
        }
        if other.database_path.is_some() {
            self.database_path = other.database_path;
        }
        if other.security.is_some() {
            self.security = other.security;
        }
    }
}

fn load_file(path: &Path) -> Option<FileConfig> {
    let content = fs::read_to_string(path).ok()?;
    toml::from_str(&content).ok()
}

fn env_string(var: &'static str, overrides: &mut Vec<String>) -> Option<String> {
    std::env::var(var).ok().map(|value| {
        overrides.push(var.into());
        value
    })
}

fn env_path(var: &'static str, overrides: &mut Vec<String>) -> Option<PathBuf> {
    env_string(var, overrides).map(PathBuf::from)
}

fn env_bool(var: &'static str, overrides: &mut Vec<String>) -> Option<bool> {
    env_string(var, overrides).map(|value| matches!(value.as_str(), "1" | "true" | "TRUE"))
}

fn parse_token_list(raw: String) -> Vec<String> {
    raw.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
