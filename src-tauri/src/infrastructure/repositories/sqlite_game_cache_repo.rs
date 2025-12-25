use crate::application::dto::game_version::{CachedGameEntry, GameAssignment, LocalGameMetadata};
use crate::domain::repositories::RepositoryError;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};

pub struct SqliteGameCacheRepository {
    pool: SqlitePool,
}

impl SqliteGameCacheRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Get a cached game entry by ID
    pub async fn get_entry(&self, game_id: i32) -> Result<Option<CachedGameEntry>, RepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT
                game_id, game_name, assigned_version, current_version,
                background_image_url, local_metadata, cached_at, last_synced
            FROM game_cache
            WHERE game_id = ?
            "#,
        )
        .bind(game_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(Self::row_to_entry(r)?)),
            None => Ok(None),
        }
    }

    /// Get a cached game entry by name using the name index
    pub async fn get_entry_by_name(&self, game_name: &str) -> Result<Option<CachedGameEntry>, RepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT
                game_id, game_name, assigned_version, current_version,
                background_image_url, local_metadata, cached_at, last_synced
            FROM game_cache
            WHERE game_name = ?
            "#,
        )
        .bind(game_name)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(Self::row_to_entry(r)?)),
            None => Ok(None),
        }
    }

    /// Get all cached game entries
    pub async fn get_all_entries(&self) -> Result<Vec<CachedGameEntry>, RepositoryError> {
        let rows = sqlx::query(
            r#"
            SELECT
                game_id, game_name, assigned_version, current_version,
                background_image_url, local_metadata, cached_at, last_synced
            FROM game_cache
            ORDER BY game_name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|r| Self::row_to_entry(r))
            .collect()
    }

    /// Set (insert or update) a cached game entry
    pub async fn set_entry(&self, entry: &CachedGameEntry) -> Result<(), RepositoryError> {
        let assigned_version_json = serde_json::to_string(&entry.assigned_version)?;
        let current_version_json = entry.current_version
            .as_ref()
            .map(|v| serde_json::to_string(v))
            .transpose()?;
        let local_metadata_json = entry.local_metadata
            .as_ref()
            .map(|m| serde_json::to_string(m))
            .transpose()?;
        let cached_at_str = entry.cached_at.to_rfc3339();
        let last_synced_str = entry.last_synced.map(|dt| dt.to_rfc3339());

        sqlx::query(
            r#"
            INSERT INTO game_cache (
                game_id, game_name, assigned_version, current_version,
                background_image_url, local_metadata, cached_at, last_synced
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(game_id) DO UPDATE SET
                game_name = excluded.game_name,
                assigned_version = excluded.assigned_version,
                current_version = excluded.current_version,
                background_image_url = excluded.background_image_url,
                local_metadata = excluded.local_metadata,
                cached_at = excluded.cached_at,
                last_synced = excluded.last_synced
            "#,
        )
        .bind(entry.game_id)
        .bind(&entry.game_name)
        .bind(assigned_version_json)
        .bind(current_version_json)
        .bind(&entry.background_image_url)
        .bind(local_metadata_json)
        .bind(cached_at_str)
        .bind(last_synced_str)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Update only the local metadata portion of a cached entry
    pub async fn update_local_metadata(
        &self,
        game_id: i32,
        metadata: LocalGameMetadata,
    ) -> Result<(), RepositoryError> {
        let metadata_json = serde_json::to_string(&metadata)?;
        let cached_at = Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            UPDATE game_cache
            SET local_metadata = ?, cached_at = ?
            WHERE game_id = ?
            "#,
        )
        .bind(metadata_json)
        .bind(cached_at)
        .bind(game_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Sync cache from Alakazam assignments with local metadata lookup
    pub async fn sync_from_assignments<F>(
        &self,
        assignments: Vec<GameAssignment>,
        local_metadata_fn: F,
    ) -> Result<(), RepositoryError>
    where
        F: Fn(&str) -> Option<LocalGameMetadata>,
    {
        // Use a transaction for batch operations
        let mut tx = self.pool.begin().await?;

        for assignment in assignments {
            let local_metadata = local_metadata_fn(&assignment.game_name);
            let entry = CachedGameEntry::from_assignment_and_metadata(assignment, local_metadata);

            let assigned_version_json = serde_json::to_string(&entry.assigned_version)?;
            let current_version_json = entry.current_version
                .as_ref()
                .map(|v| serde_json::to_string(v))
                .transpose()?;
            let local_metadata_json = entry.local_metadata
                .as_ref()
                .map(|m| serde_json::to_string(m))
                .transpose()?;
            let cached_at_str = entry.cached_at.to_rfc3339();
            let last_synced_str = entry.last_synced.map(|dt| dt.to_rfc3339());

            sqlx::query(
                r#"
                INSERT INTO game_cache (
                    game_id, game_name, assigned_version, current_version,
                    background_image_url, local_metadata, cached_at, last_synced
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT(game_id) DO UPDATE SET
                    game_name = excluded.game_name,
                    assigned_version = excluded.assigned_version,
                    current_version = excluded.current_version,
                    background_image_url = excluded.background_image_url,
                    local_metadata = excluded.local_metadata,
                    cached_at = excluded.cached_at,
                    last_synced = excluded.last_synced
                "#,
            )
            .bind(entry.game_id)
            .bind(&entry.game_name)
            .bind(assigned_version_json)
            .bind(current_version_json)
            .bind(&entry.background_image_url)
            .bind(local_metadata_json)
            .bind(cached_at_str)
            .bind(last_synced_str)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    /// Check if the cache is empty
    pub async fn is_empty(&self) -> Result<bool, RepositoryError> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM game_cache")
            .fetch_one(&self.pool)
            .await?;

        Ok(count == 0)
    }

    /// Clear all cached entries (for recovery)
    pub async fn clear_all(&self) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM game_cache")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Remove a cached entry and its name index
    pub async fn remove_entry(&self, game_id: i32) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM game_cache WHERE game_id = ?")
            .bind(game_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Helper function to convert database row to CachedGameEntry
    fn row_to_entry(r: sqlx::sqlite::SqliteRow) -> Result<CachedGameEntry, RepositoryError> {
        let game_id: i32 = r.try_get("game_id")?;
        let game_name: String = r.try_get("game_name")?;
        let assigned_version_str: String = r.try_get("assigned_version")?;
        let current_version_str: Option<String> = r.try_get("current_version")?;
        let background_image_url: Option<String> = r.try_get("background_image_url")?;
        let local_metadata_str: Option<String> = r.try_get("local_metadata")?;
        let cached_at_str: String = r.try_get("cached_at")?;
        let last_synced_str: Option<String> = r.try_get("last_synced")?;

        let entry = CachedGameEntry {
            game_id,
            game_name,
            assigned_version: serde_json::from_str(&assigned_version_str)?,
            current_version: current_version_str
                .map(|v| serde_json::from_str(&v))
                .transpose()?,
            background_image_url,
            local_metadata: local_metadata_str
                .map(|m| serde_json::from_str(&m))
                .transpose()?,
            cached_at: DateTime::parse_from_rfc3339(&cached_at_str)
                .map_err(|e| RepositoryError::SerializationError(e.to_string()))?
                .with_timezone(&Utc),
            last_synced: last_synced_str
                .map(|s| DateTime::parse_from_rfc3339(&s)
                    .map_err(|e| RepositoryError::SerializationError(e.to_string()))
                    .map(|dt| dt.with_timezone(&Utc)))
                .transpose()?,
        };

        Ok(entry)
    }
}
