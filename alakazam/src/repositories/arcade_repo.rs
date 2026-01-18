use crate::{error::Result, models::Arcade};
use sqlx::PgPool;

pub struct ArcadeRepository {
    pool: PgPool,
}

impl ArcadeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Find arcade by MAC address
    pub async fn find_by_mac_address(&self, mac_address: &str) -> Result<Option<Arcade>> {
        let arcade = sqlx::query_as::<_, Arcade>(
            "SELECT id, name, mac_address, status, last_seen_at, created_at
             FROM arcades
             WHERE mac_address = $1"
        )
        .bind(mac_address)
        .fetch_optional(&self.pool)
        .await?;

        Ok(arcade)
    }

    /// Update last seen timestamp
    pub async fn update_last_seen(&self, arcade_id: i32) -> Result<()> {
        sqlx::query(
            "UPDATE arcades
             SET last_seen_at = NOW()
             WHERE id = $1"
        )
        .bind(arcade_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Create new arcade
    pub async fn create(&self, name: &str, mac_address: &str, status: &str) -> Result<Arcade> {
        let arcade = sqlx::query_as::<_, Arcade>(
            "INSERT INTO arcades (name, mac_address, status)
             VALUES ($1, $2, $3)
             RETURNING id, name, mac_address, status, last_seen_at, created_at"
        )
        .bind(name)
        .bind(mac_address)
        .bind(status)
        .fetch_one(&self.pool)
        .await?;

        Ok(arcade)
    }

    /// List all arcades
    pub async fn list_all(&self) -> Result<Vec<Arcade>> {
        let arcades = sqlx::query_as::<_, Arcade>(
            "SELECT id, name, mac_address, status, last_seen_at, created_at
             FROM arcades
             ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(arcades)
    }

    /// Get arcade by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Option<Arcade>> {
        let arcade = sqlx::query_as::<_, Arcade>(
            "SELECT id, name, mac_address, status, last_seen_at, created_at
             FROM arcades
             WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(arcade)
    }

    /// Update arcade
    pub async fn update(&self, id: i32, name: &str, status: &str) -> Result<Arcade> {
        let arcade = sqlx::query_as::<_, Arcade>(
            "UPDATE arcades
             SET name = $2, status = $3
             WHERE id = $1
             RETURNING id, name, mac_address, status, last_seen_at, created_at"
        )
        .bind(id)
        .bind(name)
        .bind(status)
        .fetch_one(&self.pool)
        .await?;

        Ok(arcade)
    }

    /// Delete arcade
    pub async fn delete(&self, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM arcades WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
