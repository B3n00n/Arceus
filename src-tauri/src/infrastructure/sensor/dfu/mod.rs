/// Pure-Rust Nordic DFU (Device Firmware Update) over serial.
///
/// Implements the legacy Nordic DFU protocol natively, replacing the
/// previous dependency on the Python `adafruit-nrfutil` tool.
///
/// Upload flow:
/// 1. Patch device name into firmware binary
/// 2. Build init packet in memory (no ZIP intermediate)
/// 3. 1200-baud touch to enter bootloader mode
/// 4. Upload via serial DFU: START → INIT → DATA → STOP

mod crc16;
mod hci;
mod init_packet;
mod protocol;
mod slip;
mod transport;

use super::{FirmwarePatcher, Result, SensorError, XiaoDetector, XiaoMode};
use std::path::Path;
use transport::DfuTransport;

/// Maximum retry attempts for DFU upload (port may be temporarily busy)
const MAX_UPLOAD_RETRIES: u32 = 3;

/// Delay between retry attempts in seconds
const RETRY_DELAY_SECS: u64 = 3;

/// Handles DFU firmware upload to XIAO BLE nRF52840 boards.
pub struct DfuUploader;

impl DfuUploader {
    /// Complete firmware upload workflow: patch name, upload via serial DFU.
    pub async fn upload_with_name(
        port: Option<&str>,
        firmware_path: &Path,
        device_name: &str,
    ) -> Result<()> {
        let firmware = tokio::fs::read(firmware_path)
            .await
            .map_err(SensorError::Io)?;

        tracing::info!("Loaded firmware: {} bytes", firmware.len());

        let patched = FirmwarePatcher::patch_device_name(&firmware, device_name)?;
        tracing::info!("Patched device name: '{}'", device_name);

        let port_name = match port {
            Some(p) => p.to_string(),
            None => XiaoDetector::find_first()?.port,
        };

        Self::upload_with_retry(&patched, &port_name).await?;

        tracing::info!(
            "Firmware upload complete for device '{}'",
            device_name
        );

        Ok(())
    }

    /// Upload with retry logic for transient port-busy errors.
    async fn upload_with_retry(firmware: &[u8], port: &str) -> Result<()> {
        let mut last_error = None;

        for attempt in 0..MAX_UPLOAD_RETRIES {
            if attempt > 0 {
                tracing::info!(
                    "Retrying DFU upload (attempt {}/{}), waiting {}s...",
                    attempt + 1,
                    MAX_UPLOAD_RETRIES,
                    RETRY_DELAY_SECS
                );
                tokio::time::sleep(std::time::Duration::from_secs(RETRY_DELAY_SECS)).await;
            }

            // Clone data for the blocking task
            let fw = firmware.to_vec();
            let port_name = port.to_string();

            let result = tokio::task::spawn_blocking(move || {
                Self::upload_blocking(&fw, &port_name)
            })
            .await
            .map_err(|e| {
                SensorError::UploadFailed(format!("DFU task panicked: {}", e))
            })?;

            match result {
                Ok(()) => return Ok(()),
                Err(e) => {
                    let err_msg = e.to_string();
                    let is_port_busy = err_msg.contains("Device or resource busy")
                        || err_msg.contains("could not open port");

                    if is_port_busy && attempt < MAX_UPLOAD_RETRIES - 1 {
                        tracing::warn!("Port busy, will retry: {}", err_msg);
                        last_error = Some(e);
                        continue;
                    }

                    return Err(e);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| {
            SensorError::UploadFailed("DFU upload failed after retries".to_string())
        }))
    }

    /// Synchronous DFU upload — runs inside spawn_blocking.
    fn upload_blocking(firmware: &[u8], port_name: &str) -> Result<()> {
        tracing::info!(
            "Starting DFU upload on {} (with 1200-baud touch)",
            port_name
        );

        let mut transport = DfuTransport::open_with_touch(port_name)?;
        let result = protocol::run_dfu_upload(&mut transport, firmware);
        transport.close();

        result
    }
}
