use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::config::SecurityConfig;

#[derive(Clone)]
pub struct AuthManager {
    inner: Arc<AuthInner>,
}

struct AuthInner {
    config: SecurityConfig,
}

impl AuthManager {
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            inner: Arc::new(AuthInner { config }),
        }
    }

    pub fn enabled(&self) -> bool {
        self.inner.config.auth_enabled
    }

    pub fn authorize(&self, header: Option<&str>) -> bool {
        if !self.enabled() {
            return true;
        }

        let Some(token) = header.and_then(parse_bearer) else {
            return false;
        };

        if let Some(admin) = &self.inner.config.admin_token {
            if *admin == token {
                return true;
            }
        }

        self.inner.config.api_tokens.iter().any(|t| t == &token)
    }
}

fn parse_bearer(header: &str) -> Option<String> {
    header
        .strip_prefix("Bearer ")
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(String::from)
}

pub async fn auth_middleware(
    State(manager): State<AuthManager>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if manager.enabled() {
        let header = req
            .headers()
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());
        if !manager.authorize(header) {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    Ok(next.run(req).await)
}
