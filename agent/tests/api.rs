use std::{net::SocketAddr, time::Duration};

use agent::{
    config::{AgentConfig, SecurityConfig},
    events::EventHub,
    security::AuthManager,
    server::{self, AppState},
    services::{AppService, ContainerService, SnapshotService, TokenService},
    store::SqliteStore,
};
use reqwest::{Client, StatusCode};
use serde_json;
use tempfile::TempDir;
use tokio::sync::oneshot;
use uuid::Uuid;

fn next_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .and_then(|sock| sock.local_addr())
        .map(|addr| addr.port())
        .expect("no se pudo asignar puerto libre")
}

fn set_env_var(key: &str, value: &str) {
    unsafe { std::env::set_var(key, value) };
}

fn restore_env_var(key: &str, value: Option<String>) {
    match value {
        Some(v) => unsafe { std::env::set_var(key, v) },
        None => unsafe { std::env::remove_var(key) },
    }
}

#[tokio::test]
async fn containers_endpoint_creates_tasks() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let port = next_port();

    let config = AgentConfig {
        containers_root: temp.path().join("containers"),
        telemetry_level: "info".into(),
        api_bind: SocketAddr::from(([127, 0, 0, 1], port)),
        database_path: temp.path().join("agent.db"),
        security: SecurityConfig {
            auth_enabled: false,
            admin_token: None,
            api_tokens: vec![],
        },
    };

    let events = EventHub::new(32);
    let store = SqliteStore::new(&config.database_path).await?;
    let containers = ContainerService::new(config.clone(), events.clone(), store.clone());
    let apps = AppService::new(events.clone(), store.clone());
    let snapshots = SnapshotService::new(events.clone(), store.clone());
    let tokens = TokenService::new(store.clone());
    let auth = AuthManager::new(config.security.clone(), store.clone());
    let state = AppState::new(
        config.clone(),
        events.clone(),
        store.clone(),
        containers.clone(),
        apps.clone(),
        snapshots.clone(),
        tokens.clone(),
        auth,
    );

    let (tx, rx) = oneshot::channel();
    let server_handle = tokio::spawn(async move { server::serve(state, rx).await });

    tokio::time::sleep(Duration::from_millis(200)).await;

    let client = Client::new();
    let create_res = client
        .post(format!("http://{}/containers", config.api_bind))
        .json(&serde_json::json!({
            "name": "test-container",
            "platform": "windows-x64"
        }))
        .send()
        .await?;
    assert!(create_res.status().is_success());
    let task: serde_json::Value = create_res.json().await?;
    let task_id = Uuid::parse_str(task.get("id").unwrap().as_str().unwrap())?;
    let task_id_str = task_id.to_string();

    tokio::time::sleep(Duration::from_secs(1)).await;

    let tasks: Vec<serde_json::Value> = client
        .get(format!("http://{}/tasks", config.api_bind))
        .send()
        .await?
        .json()
        .await?;
    assert!(tasks.iter().any(|item| {
        item.get("id")
            .and_then(|value| value.as_str())
            .map(|value| value == task_id_str)
            .unwrap_or(false)
    }));
    let containers_list: Vec<serde_json::Value> = client
        .get(format!("http://{}/containers", config.api_bind))
        .send()
        .await?
        .json()
        .await?;
    let container_id_str = containers_list
        .first()
        .and_then(|item| item.get("id"))
        .and_then(|value| value.as_str())
        .expect("container id");

    let fetched: serde_json::Value = client
        .get(format!(
            "http://{}/containers/{}",
            config.api_bind, container_id_str
        ))
        .send()
        .await?
        .json()
        .await?;
    assert_eq!(
        fetched.get("id").unwrap().as_str().unwrap(),
        container_id_str
    );

    let delete_task: serde_json::Value = client
        .delete(format!(
            "http://{}/containers/{}",
            config.api_bind, container_id_str
        ))
        .send()
        .await?
        .json()
        .await?;
    assert_eq!(delete_task.get("type").unwrap(), "container.delete");

    let status = client
        .get(format!(
            "http://{}/containers/{}",
            config.api_bind, container_id_str
        ))
        .send()
        .await?;
    assert_eq!(status.status(), StatusCode::NOT_FOUND);

    let _ = tx.send(());
    let _ = server_handle.await?;

    Ok(())
}

#[tokio::test]
async fn auth_rejects_without_token() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let port = next_port();

    let config = AgentConfig {
        containers_root: temp.path().join("containers"),
        telemetry_level: "info".into(),
        api_bind: SocketAddr::from(([127, 0, 0, 1], port)),
        database_path: temp.path().join("agent.db"),
        security: SecurityConfig {
            auth_enabled: true,
            admin_token: Some("secret-token".into()),
            api_tokens: vec![],
        },
    };

    let events = EventHub::new(16);
    let store = SqliteStore::new(&config.database_path).await?;
    let containers = ContainerService::new(config.clone(), events.clone(), store.clone());
    let apps = AppService::new(events.clone(), store.clone());
    let snapshots = SnapshotService::new(events.clone(), store.clone());
    let tokens = TokenService::new(store.clone());
    let auth = AuthManager::new(config.security.clone(), store.clone());
    let state = AppState::new(
        config.clone(),
        events.clone(),
        store.clone(),
        containers.clone(),
        apps.clone(),
        snapshots.clone(),
        tokens.clone(),
        auth,
    );

    let (tx, rx) = oneshot::channel();
    let server_handle = tokio::spawn(async move { server::serve(state, rx).await });

    tokio::time::sleep(Duration::from_millis(200)).await;

    let client = Client::new();
    let body = serde_json::json!({ "name": "secure-test", "platform": "windows-x64" });

    let unauthorized = client
        .post(format!("http://{}/containers", config.api_bind))
        .json(&body)
        .send()
        .await?;
    assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);

    let authorized = client
        .post(format!("http://{}/containers", config.api_bind))
        .header("Authorization", "Bearer secret-token")
        .json(&body)
        .send()
        .await?;
    assert!(authorized.status().is_success());

    let _ = tx.send(());
    let _ = server_handle.await?;

    Ok(())
}

#[tokio::test]
async fn service_tokens_flow_and_reload() -> anyhow::Result<()> {
    let temp = TempDir::new()?;
    let port = next_port();
    let bind = SocketAddr::from(([127, 0, 0, 1], port));

    let config = AgentConfig {
        containers_root: temp.path().join("containers"),
        telemetry_level: "info".into(),
        api_bind: bind,
        database_path: temp.path().join("agent.db"),
        security: SecurityConfig {
            auth_enabled: true,
            admin_token: Some("root-admin".into()),
            api_tokens: vec![],
        },
    };

    let events = EventHub::new(16);
    let store = SqliteStore::new(&config.database_path).await?;
    let containers = ContainerService::new(config.clone(), events.clone(), store.clone());
    let apps = AppService::new(events.clone(), store.clone());
    let snapshots = SnapshotService::new(events.clone(), store.clone());
    let tokens = TokenService::new(store.clone());
    let auth = AuthManager::new(config.security.clone(), store.clone());
    let state = AppState::new(
        config.clone(),
        events.clone(),
        store.clone(),
        containers.clone(),
        apps.clone(),
        snapshots.clone(),
        tokens.clone(),
        auth,
    );

    let (tx, rx) = oneshot::channel();
    let server_handle = tokio::spawn(async move { server::serve(state, rx).await });
    tokio::time::sleep(Duration::from_millis(200)).await;

    let client = Client::new();
    let base = format!("http://{}", config.api_bind);
    let body = serde_json::json!({ "name": "cli-token", "platform": "windows-x64" });

    // Emit token usando el admin.
    let create_res = client
        .post(format!("{base}/security/tokens"))
        .header("Authorization", "Bearer root-admin")
        .json(&serde_json::json!({ "name": "cli" }))
        .send()
        .await?;
    assert_eq!(create_res.status(), StatusCode::CREATED);
    let created: serde_json::Value = create_res.json().await?;
    let issued_token = created
        .get("token")
        .and_then(|v| v.as_str())
        .expect("token emitido")
        .to_string();
    let issued_id = created
        .get("id")
        .and_then(|v| v.as_str())
        .expect("id emitido");
    let issued_id = Uuid::parse_str(issued_id)?;

    let unauthorized = client
        .post(format!("{base}/containers"))
        .json(&body)
        .send()
        .await?;
    assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);

    let via_token = client
        .post(format!("{base}/containers"))
        .header("Authorization", format!("Bearer {issued_token}"))
        .json(&body)
        .send()
        .await?;
    assert!(via_token.status().is_success());

    let revoke_status = client
        .delete(format!("{base}/security/tokens/{issued_id}"))
        .header("Authorization", "Bearer root-admin")
        .send()
        .await?
        .status();
    assert_eq!(revoke_status, StatusCode::NO_CONTENT);

    let revoked_attempt = client
        .post(format!("{base}/containers"))
        .header("Authorization", format!("Bearer {issued_token}"))
        .json(&body)
        .send()
        .await?;
    assert_eq!(revoked_attempt.status(), StatusCode::UNAUTHORIZED);

    // Config reload con token estatico.
    let prev_auth = std::env::var("ORBIT_AUTH_ENABLED").ok();
    let prev_admin = std::env::var("ORBIT_ADMIN_TOKEN").ok();
    let prev_tokens = std::env::var("ORBIT_API_TOKENS").ok();
    set_env_var("ORBIT_AUTH_ENABLED", "true");
    set_env_var("ORBIT_ADMIN_TOKEN", "root-admin");
    set_env_var("ORBIT_API_TOKENS", "reload-token");

    let reload = client
        .post(format!("{base}/system/security/reload"))
        .header("Authorization", "Bearer root-admin")
        .send()
        .await?;
    assert!(reload.status().is_success());

    let static_token_call = client
        .post(format!("{base}/containers"))
        .header("Authorization", "Bearer reload-token")
        .json(&serde_json::json!({ "name": "static", "platform": "windows-x64" }))
        .send()
        .await?;
    assert!(static_token_call.status().is_success());

    // Restaurar entorno.
    restore_env_var("ORBIT_AUTH_ENABLED", prev_auth);
    restore_env_var("ORBIT_ADMIN_TOKEN", prev_admin);
    restore_env_var("ORBIT_API_TOKENS", prev_tokens);

    let _ = tx.send(());
    let _ = server_handle.await?;
    Ok(())
}
