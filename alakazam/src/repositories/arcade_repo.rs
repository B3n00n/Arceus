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
}
