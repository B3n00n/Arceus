use crate::application::services::SensorService;
use crate::domain::models::Sensor;
use std::sync::Arc;
use tauri::State;

/// List all connected sensors
#[tauri::command]
pub async fn list_sensors(
    sensor_service: State<'_, Arc<SensorService>>,
) -> Result<Vec<Sensor>, String> {
    sensor_service
        .list_sensors()
        .await
        .map_err(|e| format!("Failed to list sensors: {}", e))
}

/// Get detailed info for a specific sensor
#[tauri::command]
pub async fn get_sensor_info(
    port: String,
    sensor_service: State<'_, Arc<SensorService>>,
) -> Result<Sensor, String> {
    sensor_service
        .get_sensor_info(&port)
        .await
        .map_err(|e| format!("Failed to get sensor info: {}", e))
}

/// Upload firmware to a sensor with a custom device name
#[tauri::command]
pub async fn upload_sensor_firmware(
    port: Option<String>,
    firmware_path: String,
    device_name: String,
    sensor_service: State<'_, Arc<SensorService>>,
) -> Result<(), String> {
    sensor_service
        .upload_firmware(
            port.as_deref(),
            firmware_path.into(),
            &device_name,
        )
        .await
        .map_err(|e| format!("Failed to upload firmware: {}", e))
}

/// Get the maximum allowed device name length
#[tauri::command]
pub fn get_max_sensor_name_length(
    sensor_service: State<'_, Arc<SensorService>>,
) -> usize {
    sensor_service.max_device_name_length()
}

/// Validate a firmware file (check if it has the name placeholder)
#[tauri::command]
pub async fn validate_sensor_firmware(
    firmware_path: String,
    sensor_service: State<'_, Arc<SensorService>>,
) -> Result<bool, String> {
    sensor_service
        .validate_firmware(firmware_path.into())
        .await
        .map_err(|e| format!("Failed to validate firmware: {}", e))
}
