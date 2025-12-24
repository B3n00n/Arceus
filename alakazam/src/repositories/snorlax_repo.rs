use crate::{error::Result, models::SnorlaxVersion};
use sqlx::PgPool;

pub struct SnorlaxRepository {
    pool: PgPool,
}

impl SnorlaxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get the current Snorlax version (is_current = true)
    pub async fn get_current_version(&self) -> Result<Option<SnorlaxVersion>> {
        let version = sqlx::query_as::<_, SnorlaxVersion>(
            "SELECT id, version, gcs_path, release_date, is_current, created_at
             FROM snorlax_versions
             WHERE is_current = true
             LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(version)
    }

    /// Get all Snorlax versions
    pub async fn get_all_versions(&self) -> Result<Vec<SnorlaxVersion>> {
        let versions = sqlx::query_as::<_, SnorlaxVersion>(
            "SELECT id, version, gcs_path, release_date, is_current, created_at
             FROM snorlax_versions
             ORDER BY release_date DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(versions)
    }

    /// Get a specific version by ID
    pub async fn get_version_by_id(&self, id: i32) -> Result<Option<SnorlaxVersion>> {
        let version = sqlx::query_as::<_, SnorlaxVersion>(
            "SELECT id, version, gcs_path, release_date, is_current, created_at
             FROM snorlax_versions
             WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(version)
    }

    /// Create a new Snorlax version
    pub async fn create_version(&self, version: &str, gcs_path: &str) -> Result<SnorlaxVersion> {
        let new_version = sqlx::query_as::<_, SnorlaxVersion>(
            "INSERT INTO snorlax_versions (version, gcs_path)
             VALUES ($1, $2)
             RETURNING id, version, gcs_path, release_date, is_current, created_at"
        )
        .bind(version)
        .bind(gcs_path)
        .fetch_one(&self.pool)
        .await?;

        Ok(new_version)
    }

    /// Set a version as current (and unset all others)
    pub async fn set_current_version(&self, id: i32) -> Result<()> {
        // Use a transaction to ensure atomicity
        let mut tx = self.pool.begin().await?;

        // Unset all is_current flags
        sqlx::query("UPDATE snorlax_versions SET is_current = false")
            .execute(&mut *tx)
            .await?;

        // Set the specified version as current
        sqlx::query("UPDATE snorlax_versions SET is_current = true WHERE id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    /// Delete a Snorlax version (only if not current)
    pub async fn delete_version(&self, id: i32) -> Result<()> {
        sqlx::query(
            "DELETE FROM snorlax_versions
             WHERE id = $1 AND is_current = false"
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
