use crate::domain::models::Serial;
use crate::domain::repositories::device_name_repository::{DeviceNameRepository, Result};
use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

pub struct SqliteDeviceNameRepository {
    pool: SqlitePool,
}

impl SqliteDeviceNameRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DeviceNameRepository for SqliteDeviceNameRepository {
    async fn get_name(&self, serial: &Serial) -> Result<Option<String>> {
        let serial_str = serial.as_str();

        let result: Option<String> = sqlx::query("SELECT custom_name FROM device_names WHERE serial = ?")
            .bind(serial_str)
            .fetch_optional(&self.pool)
            .await?
            .map(|row| row.try_get("custom_name"))
            .transpose()?;

        Ok(result)
    }

    async fn set_name(&self, serial: &Serial, name: Option<String>) -> Result<()> {
        let serial_str = serial.as_str();

        match name {
            Some(custom_name) => {
                sqlx::query(
                    r#"
                    INSERT INTO device_names (serial, custom_name)
                    VALUES (?, ?)
                    ON CONFLICT(serial) DO UPDATE SET custom_name = excluded.custom_name
                    "#,
                )
                .bind(serial_str)
                .bind(&custom_name)
                .execute(&self.pool)
                .await?;
            }
            None => {
                sqlx::query("DELETE FROM device_names WHERE serial = ?")
                    .bind(serial_str)
                    .execute(&self.pool)
                    .await?;
            }
        }

        Ok(())
    }
}
