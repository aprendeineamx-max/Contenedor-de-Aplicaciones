use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{config::SecurityConfig, store::SqliteStore};

#[derive(Clone)]
pub struct AuthManager {
    inner: Arc<AuthInner>,
}

struct AuthInner {
    config: RwLock<SecurityConfig>,
    store: SqliteStore,
}

#[derive(Debug, Clone)]
pub struct SecuritySnapshot {
    pub auth_enabled: bool,
    pub admin_token_present: bool,
    pub static_token_count: usize,
}

impl AuthManager {
    pub fn new(config: SecurityConfig, store: SqliteStore) -> Self {
        Self {
            inner: Arc::new(AuthInner {
                config: RwLock::new(config),
                store,
            }),
        }
    }

    pub async fn enabled(&self) -> bool {
        self.inner.config.read().await.auth_enabled
    }

    pub async fn snapshot(&self) -> SecuritySnapshot {
        let config = self.inner.config.read().await;
        SecuritySnapshot {
            auth_enabled: config.auth_enabled,
            admin_token_present: config.admin_token.is_some(),
            static_token_count: config.api_tokens.len(),
        }
    }

    pub async fn reload(&self, config: SecurityConfig) {
        *self.inner.config.write().await = config;
    }

    pub async fn authorize(&self, header: Option<&str>) -> bool {
        if !self.enabled().await {
            return true;
        }

        let Some(token) = header.and_then(parse_bearer) else {
            return false;
        };

        {
            let config = self.inner.config.read().await;
            if let Some(admin) = &config.admin_token {
                if *admin == token {
                    return true;
                }
            }

            if config.api_tokens.iter().any(|t| t == &token) {
                return true;
            }
        }

        match self
            .inner
            .store
            .verify_api_token_hash(&hash_token(&token))
            .await
        {
            Ok(valid) => valid,
            Err(err) => {
                tracing::error!(
                    ?err,
                    "No se pudo validar token de servicio en la base de datos"
                );
                false
            }
        }
    }
}

fn parse_bearer(header: &str) -> Option<String> {
    header
        .strip_prefix("Bearer ")
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(String::from)
}

pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub async fn auth_middleware(
    State(manager): State<AuthManager>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if manager.enabled().await {
        let header = req
            .headers()
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());
        if !manager.authorize(header).await {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    Ok(next.run(req).await)
}
