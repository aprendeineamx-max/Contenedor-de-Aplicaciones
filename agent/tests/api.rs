use std::{net::SocketAddr, time::Duration};

use agent::{
    config::AgentConfig,
    events::EventHub,
    server::{self, AppState},
    services::ContainerService,
    store::SqliteStore,
};
use reqwest::Client;
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
    };

    let events = EventHub::new(32);
    let store = SqliteStore::new(&config.database_path).await?;
    let containers = ContainerService::new(config.clone(), events.clone(), store.clone());
    let state = AppState::new(
        config.clone(),
        events.clone(),
        store.clone(),
        containers.clone(),
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

    let _ = tx.send(());
    let _ = server_handle.await?;

    Ok(())
}
