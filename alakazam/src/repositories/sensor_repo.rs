use crate::{error::Result, models::SensorWithArcade};
use sqlx::PgPool;

pub struct SensorRepository {
    pool: PgPool,
}

impl SensorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get all sensors with arcade name
    pub async fn get_all(&self) -> Result<Vec<SensorWithArcade>> {
        let sensors = sqlx::query_as::<_, SensorWithArcade>(
            "SELECT s.id, s.serial_number, s.mac_address, s.firmware_version,
                    s.arcade_id, a.name as arcade_name, s.updated_at, s.created_at
             FROM sensors s
             LEFT JOIN arcades a ON s.arcade_id = a.id
             ORDER BY s.updated_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(sensors)
    }

    /// Upsert a sensor by serial number
    pub async fn upsert(
        &self,
        serial_number: &str,
        mac_address: Option<&str>,
        firmware_version: Option<&str>,
        arcade_id: Option<i32>,
    ) -> Result<()> {
        sqlx::query(
            "INSERT INTO sensors (serial_number, mac_address, firmware_version, arcade_id, updated_at)
             VALUES ($1, $2, $3, $4, NOW())
             ON CONFLICT (serial_number) DO UPDATE SET
                mac_address = COALESCE($2, sensors.mac_address),
                firmware_version = COALESCE($3, sensors.firmware_version),
                arcade_id = COALESCE($4, sensors.arcade_id),
                updated_at = NOW()"
        )
        .bind(serial_number)
        .bind(mac_address)
        .bind(firmware_version)
        .bind(arcade_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
