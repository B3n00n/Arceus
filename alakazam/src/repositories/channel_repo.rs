use crate::{error::Result, models::ReleaseChannel};
use sqlx::PgPool;

pub struct ChannelRepository {
    pool: PgPool,
}

impl ChannelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// List all channels
    pub async fn list_all(&self) -> Result<Vec<ReleaseChannel>> {
        let channels = sqlx::query_as::<_, ReleaseChannel>(
            "SELECT id, name, description, created_at
             FROM release_channels
             ORDER BY id ASC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(channels)
    }

    /// Get channel by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Option<ReleaseChannel>> {
        let channel = sqlx::query_as::<_, ReleaseChannel>(
            "SELECT id, name, description, created_at
             FROM release_channels
             WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(channel)
    }

    /// Create new channel
    pub async fn create(&self, name: &str, description: Option<&str>) -> Result<ReleaseChannel> {
        let channel = sqlx::query_as::<_, ReleaseChannel>(
            "INSERT INTO release_channels (name, description)
             VALUES ($1, $2)
             RETURNING id, name, description, created_at"
        )
        .bind(name)
        .bind(description)
        .fetch_one(&self.pool)
        .await?;

        Ok(channel)
    }

    /// Update channel (only description can be changed, name is immutable)
    pub async fn update(&self, id: i32, description: Option<&str>) -> Result<ReleaseChannel> {
        let channel = if let Some(desc) = description {
            sqlx::query_as::<_, ReleaseChannel>(
                "UPDATE release_channels
                 SET description = $2
                 WHERE id = $1
                 RETURNING id, name, description, created_at"
            )
            .bind(id)
            .bind(desc)
            .fetch_one(&self.pool)
            .await?
        } else {
            // Nothing to update, just fetch current
            self.get_by_id(id).await?.unwrap()
        };

        Ok(channel)
    }

    /// Delete channel (will fail if arcades or versions are using it)
    pub async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM release_channels WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
