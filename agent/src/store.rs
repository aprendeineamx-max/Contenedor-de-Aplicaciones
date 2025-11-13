use anyhow::Result;
use serde_json;
use sqlx::{
    Error as SqlxError, Row, SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::{path::Path, str::FromStr};
use time::OffsetDateTime;
use tokio::fs;
use uuid::Uuid;

use crate::models::{
    ApiTokenInfo, AppInstance, AppStatus, ContainerModel, ContainerStatus, Snapshot, SnapshotType,
    TaskModel, TaskStatus,
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

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS api_tokens (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                hash TEXT NOT NULL,
                prefix TEXT NOT NULL,
                scopes TEXT NOT NULL,
                created_at TEXT NOT NULL,
                expires_at TEXT,
                last_used_at TEXT,
                revoked_at TEXT
            );
            "#,
        )
        .execute(&pool)
        .await?;

        Self::ensure_token_columns(&pool).await?;

        Ok(Self { pool })
    }

    async fn ensure_token_columns(pool: &SqlitePool) -> Result<()> {
        Self::add_column_if_missing(pool, "scopes TEXT NOT NULL DEFAULT '[]'").await?;
        Self::add_column_if_missing(pool, "expires_at TEXT").await?;
        Self::add_column_if_missing(pool, "last_used_at TEXT").await?;
        Ok(())
    }

    async fn add_column_if_missing(pool: &SqlitePool, definition: &str) -> Result<()> {
        let statement = format!("ALTER TABLE api_tokens ADD COLUMN {definition};");
        match sqlx::query(&statement).execute(pool).await {
            Ok(_) => Ok(()),
            Err(err) if is_duplicate_column_error(&err) => Ok(()),
            Err(err) => Err(err.into()),
        }
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

    pub async fn create_api_token(
        &self,
        name: String,
        scopes: Vec<String>,
        hash: String,
        prefix: String,
        expires_at: Option<String>,
    ) -> Result<ApiTokenInfo> {
        let info = ApiTokenInfo {
            id: Uuid::new_v4(),
            name,
            prefix,
            scopes,
            created_at: now_timestamp(),
            expires_at,
            last_used_at: None,
            revoked_at: None,
        };

        sqlx::query(
            r#"
            INSERT INTO api_tokens (id, name, hash, prefix, scopes, created_at, expires_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);
            "#,
        )
        .bind(info.id.to_string())
        .bind(&info.name)
        .bind(hash)
        .bind(&info.prefix)
        .bind(serde_json::to_string(&info.scopes)?)
        .bind(&info.created_at)
        .bind(&info.expires_at)
        .execute(&self.pool)
        .await?;

        Ok(info)
    }

    pub async fn list_api_tokens(&self) -> Result<Vec<ApiTokenInfo>> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, prefix, scopes, created_at, expires_at, last_used_at, revoked_at
            FROM api_tokens
            ORDER BY datetime(created_at) DESC;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .filter_map(|row| {
                let scopes_json: String = row.get("scopes");
                let scopes = serde_json::from_str(&scopes_json).ok()?;
                Some(ApiTokenInfo {
                    id: Uuid::parse_str(row.get::<String, _>("id").as_str()).ok()?,
                    name: row.get("name"),
                    prefix: row.get("prefix"),
                    scopes,
                    created_at: row.get("created_at"),
                    expires_at: row.get("expires_at"),
                    last_used_at: row.get("last_used_at"),
                    revoked_at: row.get("revoked_at"),
                })
            })
            .collect())
    }

    pub async fn revoke_api_token(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            r#"
            UPDATE api_tokens
            SET revoked_at = ?2
            WHERE id = ?1 AND revoked_at IS NULL;
            "#,
        )
        .bind(id.to_string())
        .bind(now_timestamp())
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn resolve_api_token(&self, hash: &str) -> Result<Option<ApiTokenInfo>> {
        let row = sqlx::query(
            r#"
            SELECT id, name, prefix, scopes, created_at, expires_at, last_used_at
            FROM api_tokens
            WHERE hash = ?1 AND revoked_at IS NULL
            LIMIT 1;
            "#,
        )
        .bind(hash)
        .fetch_optional(&self.pool)
        .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let expires_at: Option<String> = row.get("expires_at");
        if let Some(expiration) = &expires_at {
            if let Some(expires) = parse_timestamp(expiration) {
                if expires <= OffsetDateTime::now_utc() {
                    return Ok(None);
                }
            }
        }

        let scopes_json: String = row.get("scopes");
        let scopes: Vec<String> = serde_json::from_str(&scopes_json).unwrap_or_default();

        let mut info = ApiTokenInfo {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).unwrap(),
            name: row.get("name"),
            prefix: row.get("prefix"),
            scopes,
            created_at: row.get("created_at"),
            expires_at,
            last_used_at: row.get("last_used_at"),
            revoked_at: None,
        };

        let last_used = now_timestamp();
        sqlx::query(
            r#"
            UPDATE api_tokens
            SET last_used_at = ?2
            WHERE id = ?1;
            "#,
        )
        .bind(info.id.to_string())
        .bind(&last_used)
        .execute(&self.pool)
        .await?;
        info.last_used_at = Some(last_used);

        Ok(Some(info))
    }

    pub async fn count_active_tokens(&self) -> Result<i64> {
        let now = now_timestamp();
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(1) FROM api_tokens
            WHERE revoked_at IS NULL
              AND (expires_at IS NULL OR expires_at > ?1);
            "#,
        )
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(count)
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

fn now_timestamp() -> String {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".into())
}

fn normalize_sqlite_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn is_duplicate_column_error(err: &SqlxError) -> bool {
    matches!(
        err,
        SqlxError::Database(db_err) if db_err.message().contains("duplicate column name")
    )
}

fn parse_timestamp(value: &str) -> Option<OffsetDateTime> {
    OffsetDateTime::parse(value, &time::format_description::well_known::Rfc3339).ok()
}
