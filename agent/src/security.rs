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

use crate::{config::SecurityConfig, models::ApiTokenInfo, store::SqliteStore};

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
    pub managed_token_count: i64,
}

#[derive(Debug, Clone)]
pub enum AuthContext {
    Admin,
    StaticToken { token: String },
    ServiceToken { token: ApiTokenInfo },
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
        let (auth_enabled, admin_token_present, static_token_count) = {
            let config = self.inner.config.read().await;
            (
                config.auth_enabled,
                config.admin_token.is_some(),
                config.api_tokens.len(),
            )
        };
        let managed_token_count = self.inner.store.count_active_tokens().await.unwrap_or(0);
        SecuritySnapshot {
            auth_enabled,
            admin_token_present,
            static_token_count,
            managed_token_count,
        }
    }

    pub async fn reload(&self, config: SecurityConfig) {
        *self.inner.config.write().await = config;
    }

    pub async fn authorize(&self, header: Option<&str>) -> Option<AuthContext> {
        if !self.enabled().await {
            return Some(AuthContext::Admin);
        }

        let token = header.and_then(parse_bearer)?;

        {
            let config = self.inner.config.read().await;
            if let Some(admin) = &config.admin_token {
                if *admin == token {
                    return Some(AuthContext::Admin);
                }
            }

            if config.api_tokens.iter().any(|t| t == &token) {
                return Some(AuthContext::StaticToken {
                    token: token.clone(),
                });
            }
        }

        match self
            .inner
            .store
            .resolve_api_token(&hash_token(&token))
            .await
        {
            Ok(Some(info)) => Some(AuthContext::ServiceToken { token: info }),
            Ok(None) => None,
            Err(err) => {
                tracing::error!(
                    ?err,
                    "No se pudo validar token de servicio en la base de datos"
                );
                None
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
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let context = if manager.enabled().await {
        let header = req
            .headers()
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());
        match manager.authorize(header).await {
            Some(ctx) => ctx,
            None => return Err(StatusCode::UNAUTHORIZED),
        }
    } else {
        AuthContext::Admin
    };

    req.extensions_mut().insert(context);
    Ok(next.run(req).await)
}
