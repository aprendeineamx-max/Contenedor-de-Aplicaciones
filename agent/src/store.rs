use anyhow::Result;
use serde_json;
use sqlx::{
    Row, SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::{path::Path, str::FromStr};
use tokio::fs;
use uuid::Uuid;

use crate::models::{
    AppInstance, AppStatus, ContainerModel, ContainerStatus, Snapshot, SnapshotType, TaskModel,
    TaskStatus,
};
use crate::virtualization::Platform;

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

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS apps (
                id TEXT PRIMARY KEY,
                container_id TEXT NOT NULL,
                name TEXT NOT NULL,
                version TEXT,
                status TEXT NOT NULL,
                entry_points TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS snapshots (
                id TEXT PRIMARY KEY,
                container_id TEXT NOT NULL,
                label TEXT,
                snapshot_type TEXT NOT NULL,
                base_snapshot_id TEXT,
                size_bytes INTEGER NOT NULL,
                created_at TEXT NOT NULL
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

    pub async fn get_container(&self, id: Uuid) -> Result<Option<ContainerModel>> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, status, platform, tags, size_bytes, created_at, updated_at
            FROM containers WHERE id = ?1;
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.and_then(map_container_row))
    }

    pub async fn delete_container(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM containers WHERE id = ?1;")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn list_containers(&self, status: Option<String>) -> Result<Vec<ContainerModel>> {
        let mut query = String::from(
            "SELECT id, name, description, status, platform, tags, size_bytes, created_at, updated_at FROM containers",
        );
        if status.is_some() {
            query.push_str(" WHERE status = ?1");
        }
        query.push_str(" ORDER BY datetime(created_at) DESC;");

        let mut statement = sqlx::query(&query);
        if let Some(status) = status {
            statement = statement.bind(status);
        }

        let rows = statement.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().filter_map(map_container_row).collect())
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

    pub async fn list_tasks(
        self: &SqliteStore,
        status: Option<String>,
        limit: Option<i64>,
    ) -> Result<Vec<TaskModel>> {
        let mut query = String::from(
            "SELECT id, type, status, progress, message, created_at, updated_at FROM tasks",
        );
        if status.is_some() {
            query.push_str(" WHERE status = ?1");
        }
        query.push_str(" ORDER BY datetime(created_at) DESC");
        if limit.is_some() {
            query.push_str(" LIMIT ?2");
        }
        query.push(';');

        let mut statement = sqlx::query(&query);
        if let Some(status) = status {
            statement = statement.bind(status);
        }
        if let Some(limit) = limit {
            statement = statement.bind(limit);
        }

        let rows = statement.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().filter_map(map_task_row).collect())
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

        Ok(row.and_then(map_task_row))
    }

    pub async fn insert_app(&self, app: &AppInstance) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO apps (id, container_id, name, version, status, entry_points, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT(id) DO UPDATE SET
                name=excluded.name,
                version=excluded.version,
                status=excluded.status,
                entry_points=excluded.entry_points,
                updated_at=excluded.updated_at;
            "#,
        )
        .bind(app.id.to_string())
        .bind(app.container_id.to_string())
        .bind(&app.name)
        .bind(&app.version)
        .bind(app.status.as_str())
        .bind(serde_json::to_string(&app.entry_points)?)
        .bind(&app.created_at)
        .bind(&app.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_apps(&self, container_id: Uuid) -> Result<Vec<AppInstance>> {
        let rows = sqlx::query(
            r#"
            SELECT id, container_id, name, version, status, entry_points, created_at, updated_at
            FROM apps WHERE container_id = ?1
            ORDER BY datetime(created_at) DESC;
            "#,
        )
        .bind(container_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().filter_map(map_app_row).collect())
    }

    pub async fn get_app(&self, app_id: Uuid) -> Result<Option<AppInstance>> {
        let row = sqlx::query(
            r#"
            SELECT id, container_id, name, version, status, entry_points, created_at, updated_at
            FROM apps WHERE id = ?1;
            "#,
        )
        .bind(app_id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.and_then(map_app_row))
    }

    pub async fn insert_snapshot(&self, snapshot: &Snapshot) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO snapshots (id, container_id, label, snapshot_type, base_snapshot_id, size_bytes, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ON CONFLICT(id) DO UPDATE SET
                label = excluded.label,
                snapshot_type = excluded.snapshot_type,
                base_snapshot_id = excluded.base_snapshot_id,
                size_bytes = excluded.size_bytes;
            "#,
        )
        .bind(snapshot.id.to_string())
        .bind(snapshot.container_id.to_string())
        .bind(&snapshot.label)
        .bind(snapshot.snapshot_type.as_str())
        .bind(snapshot.base_snapshot_id.map(|id| id.to_string()))
        .bind(snapshot.size_bytes as i64)
        .bind(&snapshot.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_snapshots(&self, container_id: Uuid) -> Result<Vec<Snapshot>> {
        let rows = sqlx::query(
            r#"
            SELECT id, container_id, label, snapshot_type, base_snapshot_id, size_bytes, created_at
            FROM snapshots WHERE container_id = ?1
            ORDER BY datetime(created_at) DESC;
            "#,
        )
        .bind(container_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().filter_map(map_snapshot_row).collect())
    }

    pub async fn get_snapshot(&self, snapshot_id: Uuid) -> Result<Option<Snapshot>> {
        let row = sqlx::query(
            r#"
            SELECT id, container_id, label, snapshot_type, base_snapshot_id, size_bytes, created_at
            FROM snapshots WHERE id = ?1;
            "#,
        )
        .bind(snapshot_id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.and_then(map_snapshot_row))
    }
}

fn map_container_row(row: sqlx::sqlite::SqliteRow) -> Option<ContainerModel> {
    Some(ContainerModel {
        id: Uuid::parse_str(row.get::<String, _>("id").as_str()).ok()?,
        name: row.get("name"),
        description: row.get("description"),
        status: ContainerStatus::from_str(&row.get::<String, _>("status")),
        platform: Platform::from_str(&row.get::<String, _>("platform")),
        tags: serde_json::from_str(&row.get::<String, _>("tags")).ok()?,
        size_bytes: row.get::<i64, _>("size_bytes") as u64,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

fn map_task_row(row: sqlx::sqlite::SqliteRow) -> Option<TaskModel> {
    Some(TaskModel {
        id: Uuid::parse_str(row.get::<String, _>("id").as_str()).ok()?,
        task_type: row.get("type"),
        status: TaskStatus::from_str(&row.get::<String, _>("status")),
        progress: row.get::<i64, _>("progress") as u8,
        message: row.get("message"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

fn map_app_row(row: sqlx::sqlite::SqliteRow) -> Option<AppInstance> {
    Some(AppInstance {
        id: Uuid::parse_str(row.get::<String, _>("id").as_str()).ok()?,
        container_id: Uuid::parse_str(row.get::<String, _>("container_id").as_str()).ok()?,
        name: row.get("name"),
        version: row.get("version"),
        status: AppStatus::from_str(&row.get::<String, _>("status")),
        entry_points: serde_json::from_str(&row.get::<String, _>("entry_points")).ok()?,
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

fn map_snapshot_row(row: sqlx::sqlite::SqliteRow) -> Option<Snapshot> {
    Some(Snapshot {
        id: Uuid::parse_str(row.get::<String, _>("id").as_str()).ok()?,
        container_id: Uuid::parse_str(row.get::<String, _>("container_id").as_str()).ok()?,
        label: row.get("label"),
        snapshot_type: SnapshotType::from_str(&row.get::<String, _>("snapshot_type")),
        base_snapshot_id: row
            .get::<Option<String>, _>("base_snapshot_id")
            .and_then(|value| Uuid::parse_str(&value).ok()),
        size_bytes: row.get::<i64, _>("size_bytes") as u64,
        created_at: row.get("created_at"),
    })
}

fn normalize_sqlite_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
