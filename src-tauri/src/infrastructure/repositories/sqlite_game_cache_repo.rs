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

    /// Get all cached game entries
    pub async fn get_all_entries(&self) -> Result<Vec<CachedGameEntry>, RepositoryError> {
        let rows = sqlx::query(
            r#"
            SELECT
                game_id, game_name, assigned_version_id, assigned_version,
                installed_version_id, installed_version, installed_at
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
        let installed_at_str = entry.installed_at.map(|dt| dt.to_rfc3339());

        sqlx::query(
            r#"
            INSERT INTO game_cache (
                game_id, game_name, assigned_version_id, assigned_version,
                installed_version_id, installed_version, installed_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(game_id) DO UPDATE SET
                game_name = excluded.game_name,
                assigned_version_id = excluded.assigned_version_id,
                assigned_version = excluded.assigned_version,
                installed_version_id = excluded.installed_version_id,
                installed_version = excluded.installed_version,
                installed_at = excluded.installed_at
            "#,
        )
        .bind(entry.game_id)
        .bind(&entry.game_name)
        .bind(entry.assigned_version_id)
        .bind(&entry.assigned_version)
        .bind(entry.installed_version_id)
        .bind(&entry.installed_version)
        .bind(installed_at_str)
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
        let installed_at = metadata.installed_at.to_rfc3339();

        sqlx::query(
            r#"
            UPDATE game_cache
            SET installed_version_id = ?, installed_version = ?, installed_at = ?
            WHERE game_id = ?
            "#,
        )
        .bind(metadata.installed_version_id)
        .bind(&metadata.installed_version)
        .bind(installed_at)
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

            let installed_at_str = entry.installed_at.map(|dt| dt.to_rfc3339());

            sqlx::query(
                r#"
                INSERT INTO game_cache (
                    game_id, game_name, assigned_version_id, assigned_version,
                    installed_version_id, installed_version, installed_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT(game_id) DO UPDATE SET
                    game_name = excluded.game_name,
                    assigned_version_id = excluded.assigned_version_id,
                    assigned_version = excluded.assigned_version,
                    installed_version_id = excluded.installed_version_id,
                    installed_version = excluded.installed_version,
                    installed_at = excluded.installed_at
                "#,
            )
            .bind(entry.game_id)
            .bind(&entry.game_name)
            .bind(entry.assigned_version_id)
            .bind(&entry.assigned_version)
            .bind(entry.installed_version_id)
            .bind(&entry.installed_version)
            .bind(installed_at_str)
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

    /// Helper function to convert database row to CachedGameEntry
    fn row_to_entry(r: sqlx::sqlite::SqliteRow) -> Result<CachedGameEntry, RepositoryError> {
        let game_id: i32 = r.try_get("game_id")?;
        let game_name: String = r.try_get("game_name")?;
        let assigned_version_id: i32 = r.try_get("assigned_version_id")?;
        let assigned_version: String = r.try_get("assigned_version")?;
        let installed_version_id: Option<i32> = r.try_get("installed_version_id")?;
        let installed_version: Option<String> = r.try_get("installed_version")?;
        let installed_at_str: Option<String> = r.try_get("installed_at")?;

        let installed_at = installed_at_str
            .map(|s| DateTime::parse_from_rfc3339(&s)
                .map_err(|e| RepositoryError::SerializationError(e.to_string()))
                .map(|dt| dt.with_timezone(&Utc)))
            .transpose()?;

        let entry = CachedGameEntry {
            game_id,
            game_name,
            assigned_version_id,
            assigned_version,
            installed_version_id,
            installed_version,
            installed_at,
        };

        Ok(entry)
    }
}
