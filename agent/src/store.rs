use anyhow::Result;
use serde_json;
use sqlx::{
    Row, SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::{path::Path, str::FromStr};
use tokio::fs;
use uuid::Uuid;

use crate::{
    models::{ContainerModel, ContainerStatus, TaskModel, TaskStatus},
    virtualization::Platform,
};

#[derive(Clone)]
pub struct SqliteStore {
    pool: SqlitePool,
}

impl SqliteStore {
    pub async fn new(db_path: impl AsRef<Path>) -> Result<Self> {
        let db_path = db_path.as_ref();
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let db_url = format!("sqlite://{}", normalize_sqlite_path(db_path));
        let connect_opts = SqliteConnectOptions::from_str(&db_url)?.create_if_missing(true);
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(connect_opts)
            .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS containers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                platform TEXT NOT NULL,
                tags TEXT NOT NULL,
                size_bytes INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                type TEXT NOT NULL,
                status TEXT NOT NULL,
                progress INTEGER NOT NULL,
                message TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn upsert_container(&self, container: &ContainerModel) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO containers (id, name, description, status, platform, tags, size_bytes, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT(id) DO UPDATE SET
                name=excluded.name,
                description=excluded.description,
                status=excluded.status,
                platform=excluded.platform,
                tags=excluded.tags,
                size_bytes=excluded.size_bytes,
                created_at=excluded.created_at,
                updated_at=excluded.updated_at;
            "#,
        )
        .bind(container.id.to_string())
        .bind(&container.name)
        .bind(&container.description)
        .bind(container.status.as_str())
        .bind(container.platform.as_str())
        .bind(serde_json::to_string(&container.tags)?)
        .bind(container.size_bytes as i64)
        .bind(&container.created_at)
        .bind(&container.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_containers(&self) -> Result<Vec<ContainerModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, description, status, platform, tags, size_bytes, created_at, updated_at
            FROM containers
            ORDER BY datetime(created_at) DESC;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .filter_map(|row| {
                Some(ContainerModel {
                    id: Uuid::parse_str(row.get::<String, _>("id").as_str()).ok()?,
                    name: row.get::<String, _>("name"),
                    description: row.get::<Option<String>, _>("description"),
                    status: ContainerStatus::from_str(&row.get::<String, _>("status")),
                    platform: Platform::from_str(&row.get::<String, _>("platform")),
                    tags: serde_json::from_str(&row.get::<String, _>("tags")).ok()?,
                    size_bytes: row.get::<i64, _>("size_bytes") as u64,
                    created_at: row.get::<String, _>("created_at"),
                    updated_at: row.get::<String, _>("updated_at"),
                })
            })
            .collect())
    }

    pub async fn upsert_task(&self, task: &TaskModel) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO tasks (id, type, status, progress, message, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ON CONFLICT(id) DO UPDATE SET
                type=excluded.type,
                status=excluded.status,
                progress=excluded.progress,
                message=excluded.message,
                created_at=excluded.created_at,
                updated_at=excluded.updated_at;
            "#,
        )
        .bind(task.id.to_string())
        .bind(&task.task_type)
        .bind(task.status.as_str())
        .bind(task.progress as i64)
        .bind(&task.message)
        .bind(&task.created_at)
        .bind(&task.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_tasks(&self) -> Result<Vec<TaskModel>> {
        let rows = sqlx::query(
            r#"
            SELECT id, type, status, progress, message, created_at, updated_at
            FROM tasks
            ORDER BY datetime(created_at) DESC;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .filter_map(|row| {
                Some(TaskModel {
                    id: Uuid::parse_str(row.get::<String, _>("id").as_str()).ok()?,
                    task_type: row.get::<String, _>("type"),
                    status: TaskStatus::from_str(&row.get::<String, _>("status")),
                    progress: row.get::<i64, _>("progress") as u8,
                    message: row.get::<Option<String>, _>("message"),
                    created_at: row.get::<String, _>("created_at"),
                    updated_at: row.get::<String, _>("updated_at"),
                })
            })
            .collect())
    }

    pub async fn get_task(&self, id: Uuid) -> Result<Option<TaskModel>> {
        let row = sqlx::query(
            r#"
            SELECT id, type, status, progress, message, created_at, updated_at
            FROM tasks WHERE id = ?1;
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| TaskModel {
            id,
            task_type: row.get::<String, _>("type"),
            status: TaskStatus::from_str(&row.get::<String, _>("status")),
            progress: row.get::<i64, _>("progress") as u8,
            message: row.get::<Option<String>, _>("message"),
            created_at: row.get::<String, _>("created_at"),
            updated_at: row.get::<String, _>("updated_at"),
        }))
    }
}

fn normalize_sqlite_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
