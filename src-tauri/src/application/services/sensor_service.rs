use crate::domain::models::{Sensor, SensorConnectionStatus};
use crate::infrastructure::sensor::{
    DfuUploader, FirmwarePatcher, SensorError, SerialComm, XiaoDetector, XiaoMode,
};
use std::path::PathBuf;
use tokio::sync::Mutex;

/// Result type for sensor service operations
pub type Result<T> = std::result::Result<T, SensorServiceError>;

/// Errors that can occur in the sensor service
#[derive(Debug, thiserror::Error)]
pub enum SensorServiceError {
    #[error("Sensor error: {0}")]
    Sensor(#[from] SensorError),

    #[error("Firmware file not found: {0}")]
    FirmwareNotFound(String),

    #[error("Invalid firmware: {0}")]
    InvalidFirmware(String),

    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

/// Application service for sensor management.
///
/// Uses a mutex to serialize all serial port operations, preventing
/// concurrent access from multiple Tauri command invocations.
pub struct SensorService {
    serial_lock: Mutex<()>,
}

impl SensorService {
    pub fn new() -> Self {
        Self {
            serial_lock: Mutex::new(()),
        }
    }

    /// List all connected sensors (fast - doesn't open serial ports)
    pub async fn list_sensors(&self) -> Result<Vec<Sensor>> {
        let ports = XiaoDetector::find_all();

        let sensors: Vec<Sensor> = ports
            .into_iter()
            .map(|port| Sensor::from_port(&port.port, port.mode == XiaoMode::Bootloader))
            .collect();

        Ok(sensors)
    }

    /// Get detailed info for a specific sensor by port (opens serial port)
    pub async fn get_sensor_info(&self, port: &str) -> Result<Sensor> {
        let _guard = self.serial_lock.lock().await;

        let port_name = port.to_string();

        let info = tokio::task::spawn_blocking(move || {
            SerialComm::open_and_get_info(&port_name, 3)
        })
        .await
        .map_err(|e| SensorServiceError::OperationFailed(e.to_string()))??;

        let sensor = Sensor::new(port.to_string(), SensorConnectionStatus::Connected)
            .with_info(
                info.serial_number,
                info.mac_address,
                info.ble_mac_address,
                info.device_name,
                info.firmware_version,
            );

        Ok(sensor)
    }

    /// Upload firmware to a sensor with a custom device name
    pub async fn upload_firmware(
        &self,
        port: Option<&str>,
        firmware_path: PathBuf,
        device_name: &str,
    ) -> Result<()> {
        if !firmware_path.exists() {
            return Err(SensorServiceError::FirmwareNotFound(
                firmware_path.display().to_string(),
            ));
        }

        if firmware_path.extension().and_then(|s| s.to_str()) != Some("bin") {
            return Err(SensorServiceError::InvalidFirmware(
                "Firmware must have .bin extension".to_string(),
            ));
        }

        if device_name.trim().is_empty() {
            return Err(SensorServiceError::InvalidFirmware(
                "Device name cannot be empty".to_string(),
            ));
        }

        if device_name.len() > FirmwarePatcher::max_name_length() {
            return Err(SensorServiceError::InvalidFirmware(format!(
                "Device name too long (max {} characters)",
                FirmwarePatcher::max_name_length()
            )));
        }

        // Acquire serial lock to prevent concurrent port access
        let _guard = self.serial_lock.lock().await;

        tracing::info!(
            port = ?port,
            firmware = %firmware_path.display(),
            device_name = %device_name,
            "Starting firmware upload"
        );

        DfuUploader::upload_with_name(port, &firmware_path, device_name).await?;

        tracing::info!(
            device_name = %device_name,
            "Firmware upload completed successfully"
        );

        Ok(())
    }

    /// Get the maximum allowed device name length
    pub fn max_device_name_length(&self) -> usize {
        FirmwarePatcher::max_name_length()
    }

    /// Check if a firmware file contains the name placeholder
    pub async fn validate_firmware(&self, firmware_path: PathBuf) -> Result<bool> {
        let firmware = tokio::fs::read(&firmware_path)
            .await
            .map_err(|e| SensorServiceError::FirmwareNotFound(e.to_string()))?;

        Ok(FirmwarePatcher::has_placeholder(&firmware))
    }
}

impl Default for SensorService {
    fn default() -> Self {
        Self::new()
    }
}
