use std::{net::SocketAddr, time::Duration};

use agent::{
    config::{AgentConfig, SecurityConfig},
    events::EventHub,
    security::AuthManager,
    server::{self, AppState},
    services::{AppService, ContainerService, SnapshotService},
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
    let auth = AuthManager::new(config.security.clone());
    let state = AppState::new(
        config.clone(),
        events.clone(),
        store.clone(),
        containers.clone(),
        apps.clone(),
        snapshots.clone(),
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
    let auth = AuthManager::new(config.security.clone());
    let state = AppState::new(
        config.clone(),
        events.clone(),
        store.clone(),
        containers.clone(),
        apps.clone(),
        snapshots.clone(),
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
