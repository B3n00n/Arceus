use crate::{
    error::Result,
    models::SensorWithArcade,
    repositories::{ArcadeRepository, SensorRepository},
};
use std::sync::Arc;

pub struct SensorService {
    sensor_repo: Arc<SensorRepository>,
    arcade_repo: Arc<ArcadeRepository>,
}

impl SensorService {
    pub fn new(sensor_repo: Arc<SensorRepository>, arcade_repo: Arc<ArcadeRepository>) -> Self {
        Self {
            sensor_repo,
            arcade_repo,
        }
    }

    /// Get all tracked sensors (for admin dashboard)
    pub async fn get_all_sensors(&self) -> Result<Vec<SensorWithArcade>> {
        self.sensor_repo.get_all().await
    }

    /// Report a sensor from an arcade (upsert by serial number)
    pub async fn report_sensor(
        &self,
        machine_id: &str,
        serial_number: &str,
        mac_address: Option<&str>,
        firmware_version: Option<&str>,
    ) -> Result<()> {
        // Look up arcade by machine_id to get arcade_id
        let arcade = self.arcade_repo.find_by_machine_id(machine_id).await?;
        let arcade_id = arcade.map(|a| a.id);

        self.sensor_repo
            .upsert(serial_number, mac_address, firmware_version, arcade_id)
            .await
    }
}
