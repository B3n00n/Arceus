use crate::{error::Result, models::Arcade};
use sqlx::PgPool;

pub struct ArcadeRepository {
    pool: PgPool,
}

impl ArcadeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Find arcade by machine ID
    pub async fn find_by_machine_id(&self, machine_id: &str) -> Result<Option<Arcade>> {
        let arcade = sqlx::query_as::<_, Arcade>(
            "SELECT id, name, machine_id, status, channel_id, installed_games, last_seen_at, created_at
             FROM arcades
             WHERE machine_id = $1"
        )
        .bind(machine_id)
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

    /// Create new arcade (defaults to production channel_id=1)
    pub async fn create(&self, name: &str, machine_id: &str, status: &str, channel_id: i32) -> Result<Arcade> {
        let arcade = sqlx::query_as::<_, Arcade>(
            "INSERT INTO arcades (name, machine_id, status, channel_id)
             VALUES ($1, $2, $3, $4)
             RETURNING id, name, machine_id, status, channel_id, installed_games, last_seen_at, created_at"
        )
        .bind(name)
        .bind(machine_id)
        .bind(status)
        .bind(channel_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(arcade)
    }

    /// List all arcades
    pub async fn list_all(&self) -> Result<Vec<Arcade>> {
        let arcades = sqlx::query_as::<_, Arcade>(
            "SELECT id, name, machine_id, status, channel_id, installed_games, last_seen_at, created_at
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
            "SELECT id, name, machine_id, status, channel_id, installed_games, last_seen_at, created_at
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
             RETURNING id, name, machine_id, status, channel_id, installed_games, last_seen_at, created_at"
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

    /// Update arcade's release channel
    pub async fn update_channel(&self, arcade_id: i32, channel_id: i32) -> Result<Arcade> {
        let arcade = sqlx::query_as::<_, Arcade>(
            "UPDATE arcades
             SET channel_id = $2
             WHERE id = $1
             RETURNING id, name, machine_id, status, channel_id, installed_games, last_seen_at, created_at"
        )
        .bind(arcade_id)
        .bind(channel_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(arcade)
    }

    /// Update installed games JSON for an arcade
    pub async fn update_installed_games(&self, arcade_id: i32, games_json: serde_json::Value) -> Result<()> {
        sqlx::query(
            "UPDATE arcades
             SET installed_games = $2
             WHERE id = $1"
        )
        .bind(arcade_id)
        .bind(games_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
