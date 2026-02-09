use crate::app::events::EventBus;
use crate::app::models::config::AlakazamConfig;
use crate::app::config::get_machine_id;
use crate::domain::models::{Sensor, SensorConnectionStatus};
use crate::infrastructure::sensor::{
    DfuUploader, FirmwarePatcher, SensorError, SerialComm, XiaoDetector, XiaoMode,
};
use std::path::PathBuf;
use std::sync::Arc;
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
    event_bus: Arc<EventBus>,
    alakazam_config: AlakazamConfig,
}

impl SensorService {
    pub fn new(event_bus: Arc<EventBus>, alakazam_config: AlakazamConfig) -> Self {
        Self {
            serial_lock: Mutex::new(()),
            event_bus,
            alakazam_config,
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

        let port_str = port.unwrap_or("auto").to_string();

        tracing::info!(
            port = ?port,
            firmware = %firmware_path.display(),
            device_name = %device_name,
            "Starting firmware upload"
        );

        self.event_bus.sensor_upload_progress(
            port_str.clone(),
            "starting".to_string(),
            0.0,
        );

        let event_bus = self.event_bus.clone();
        let progress_port = port_str.clone();
        let on_progress: Arc<dyn Fn(f32) + Send + Sync> = Arc::new(move |pct| {
            event_bus.sensor_upload_progress(
                progress_port.clone(),
                "uploading".to_string(),
                pct,
            );
        });

        let result = DfuUploader::upload_with_name(port, &firmware_path, device_name, on_progress).await;

        match &result {
            Ok(()) => {
                self.event_bus.sensor_upload_progress(
                    port_str,
                    "completed".to_string(),
                    100.0,
                );
                tracing::info!(
                    device_name = %device_name,
                    "Firmware upload completed successfully"
                );

                // Fire-and-forget: report sensor to Alakazam
                self.spawn_report_to_alakazam();
            }
            Err(e) => {
                self.event_bus.sensor_upload_progress(
                    port_str,
                    "failed".to_string(),
                    0.0,
                );
                tracing::error!(
                    device_name = %device_name,
                    error = %e,
                    "Firmware upload failed"
                );
            }
        }

        result?;
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

    /// Report sensor info to Alakazam (fire-and-forget).
    /// Waits for device reboot, scans for it, reads info, then POSTs to Alakazam.
    fn spawn_report_to_alakazam(&self) {
        let base_url = self.alakazam_config.base_url.clone();

        tokio::spawn(async move {
            // Wait for device to reboot after firmware upload
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;

            // Scan for the sensor (port may have changed after reboot)
            let port_name = match XiaoDetector::find_first() {
                Ok(d) => d.port,
                Err(e) => {
                    tracing::warn!("Could not find sensor for reporting: {}", e);
                    return;
                }
            };

            let info = match tokio::task::spawn_blocking(move || {
                SerialComm::open_and_get_info(&port_name, 3)
            })
            .await
            {
                Ok(Ok(info)) => info,
                Ok(Err(e)) => {
                    tracing::warn!("Could not read sensor info for reporting: {}", e);
                    return;
                }
                Err(e) => {
                    tracing::warn!("Sensor info task panicked: {}", e);
                    return;
                }
            };

            // serial_number is required for reporting — skip if unavailable
            let serial_number = match &info.serial_number {
                Some(s) if !s.is_empty() => s.clone(),
                _ => {
                    tracing::warn!("Sensor has no serial number, skipping Alakazam report");
                    return;
                }
            };

            let machine_id = match get_machine_id() {
                Ok(id) => id,
                Err(e) => {
                    tracing::warn!("Could not get machine ID for sensor reporting: {}", e);
                    return;
                }
            };

            let url = format!("{}/api/arcade/sensors/report", base_url);
            let body = serde_json::json!({
                "serial_number": serial_number,
                "mac_address": info.mac_address.or(info.ble_mac_address),
                "firmware_version": info.firmware_version,
            });

            match reqwest::Client::new()
                .post(&url)
                .header("X-Machine-ID", &machine_id)
                .json(&body)
                .timeout(std::time::Duration::from_secs(10))
                .send()
                .await
            {
                Ok(resp) => {
                    tracing::info!(
                        "Sensor reported to Alakazam (status: {})",
                        resp.status()
                    );
                }
                Err(e) => {
                    tracing::warn!("Failed to report sensor to Alakazam: {}", e);
                }
            }
        });
    }
}

impl Default for SensorService {
    fn default() -> Self {
        panic!("SensorService requires EventBus and AlakazamConfig — use SensorService::new()")
    }
}
