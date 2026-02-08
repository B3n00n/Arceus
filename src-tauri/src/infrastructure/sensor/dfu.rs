/// Nordic DFU (Device Firmware Update) over serial
///
/// Uses `adafruit-nrfutil` to create DFU packages and upload firmware
/// to XIAO BLE nRF52840 boards via the Arduino Mbed bootloader.
///
/// Upload flow:
/// 1. Write patched firmware binary to a temp file
/// 2. Create DFU ZIP package via `adafruit-nrfutil dfu genpkg`
/// 3. Upload via `adafruit-nrfutil dfu serial` with 1200-baud touch for bootloader entry

use super::{FirmwarePatcher, Result, SensorError, XiaoDetector};
use std::path::{Path, PathBuf};

/// Device type identifier for the nRF52840 DFU package
const DFU_DEV_TYPE: &str = "0x0052";

/// Baud rate for DFU serial upload
const DFU_BAUD_RATE: &str = "115200";

/// Baud rate for bootloader entry touch
const TOUCH_BAUD: &str = "1200";

/// Maximum retry attempts for DFU upload (port may be temporarily busy)
const MAX_UPLOAD_RETRIES: u32 = 3;

/// Delay between retry attempts in seconds
const RETRY_DELAY_SECS: u64 = 3;

/// Handles DFU firmware packaging and serial upload
pub struct DfuUploader;

impl DfuUploader {
    /// Ensure `adafruit-nrfutil` is available, auto-installing via pip if needed.
    async fn ensure_nrfutil_installed() -> Result<()> {
        // Check if already available
        if Self::is_nrfutil_available().await {
            return Ok(());
        }

        tracing::info!("adafruit-nrfutil not found, attempting pip install...");

        // Try pip commands: pip3 first on Linux, pip on Windows
        let pip_commands = if cfg!(target_os = "windows") {
            vec!["pip", "pip3"]
        } else {
            vec!["pip3", "pip"]
        };

        let mut last_error = String::new();
        for pip in &pip_commands {
            tracing::info!("Trying: {} install --user adafruit-nrfutil", pip);
            let result = tokio::process::Command::new(pip)
                .args(["install", "--user", "adafruit-nrfutil"])
                .output()
                .await;

            match result {
                Ok(output) if output.status.success() => {
                    tracing::info!("Successfully installed adafruit-nrfutil via {}", pip);
                    // Verify it's now available
                    if Self::is_nrfutil_available().await {
                        return Ok(());
                    }
                    last_error = "Installed but still not found on PATH. \
                        You may need to add ~/.local/bin to your PATH."
                        .to_string();
                }
                Ok(output) => {
                    last_error = Self::combined_output(&output);
                    tracing::warn!("{} install failed: {}", pip, last_error);
                }
                Err(e) => {
                    tracing::debug!("{} not available: {}", pip, e);
                }
            }
        }

        Err(SensorError::NrfutilNotFound(format!(
            "Could not auto-install adafruit-nrfutil. {}\n\
            Install manually: pip install adafruit-nrfutil",
            last_error
        )))
    }

    /// Check if `adafruit-nrfutil` is callable.
    async fn is_nrfutil_available() -> bool {
        tokio::process::Command::new("adafruit-nrfutil")
            .arg("version")
            .output()
            .await
            .is_ok_and(|o| o.status.success())
    }

    /// Complete firmware upload workflow: patch name, create DFU package, upload via serial.
    pub async fn upload_with_name(
        port: Option<&str>,
        firmware_path: &Path,
        device_name: &str,
    ) -> Result<()> {
        Self::ensure_nrfutil_installed().await?;

        let firmware = tokio::fs::read(firmware_path)
            .await
            .map_err(SensorError::Io)?;

        tracing::info!("Loaded firmware: {} bytes", firmware.len());

        let patched = FirmwarePatcher::patch_device_name(&firmware, device_name)?;
        tracing::info!("Patched device name: '{}'", device_name);

        let temp_dir = Self::create_temp_dir()?;
        let package_path = Self::create_package(&patched, &temp_dir).await?;
        tracing::info!("Created DFU package: {:?}", package_path);

        let port_name = match port {
            Some(p) => p.to_string(),
            None => XiaoDetector::find_first()?.port,
        };

        let result = Self::upload_serial_with_retry(&package_path, &port_name).await;

        // Clean up temp directory regardless of upload result
        let _ = std::fs::remove_dir_all(&temp_dir);

        result?;

        tracing::info!(
            "Firmware upload complete for device '{}'",
            device_name
        );

        Ok(())
    }

    /// Create a unique temporary directory for DFU files
    fn create_temp_dir() -> Result<PathBuf> {
        let dir = std::env::temp_dir().join(format!("arceus-dfu-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).map_err(SensorError::Io)?;
        Ok(dir)
    }

    /// Create a DFU ZIP package from raw firmware binary using `adafruit-nrfutil`.
    async fn create_package(firmware_bin: &[u8], temp_dir: &Path) -> Result<PathBuf> {
        let bin_path = temp_dir.join("firmware.bin");
        let zip_path = temp_dir.join("firmware.zip");

        tokio::fs::write(&bin_path, firmware_bin)
            .await
            .map_err(SensorError::Io)?;

        let output = tokio::process::Command::new("adafruit-nrfutil")
            .args([
                "dfu",
                "genpkg",
                "--dev-type",
                DFU_DEV_TYPE,
                "--application",
                &bin_path.to_string_lossy(),
                &zip_path.to_string_lossy(),
            ])
            .output()
            .await
            .map_err(|e| {
                SensorError::UploadFailed(format!(
                    "Failed to run adafruit-nrfutil: {}. Is it installed?",
                    e
                ))
            })?;

        let combined = Self::combined_output(&output);
        if !output.status.success() || combined.contains("Failed") {
            return Err(SensorError::UploadFailed(format!(
                "DFU package creation failed: {}",
                combined
            )));
        }

        tracing::debug!("adafruit-nrfutil genpkg: {}", combined);

        Ok(zip_path)
    }

    /// Upload with retry logic for transient port-busy errors.
    ///
    /// After a previous upload, the device reboots and re-enumerates on USB.
    /// System services (e.g. ModemManager) may briefly claim the port,
    /// causing "Device or resource busy" errors.
    async fn upload_serial_with_retry(package_path: &Path, port: &str) -> Result<()> {
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

            match Self::upload_serial(package_path, port).await {
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

    /// Upload a DFU package to the device via serial.
    ///
    /// Uses the `-t 1200` flag to trigger a 1200-baud touch before upload,
    /// which causes the Arduino Mbed bootloader to enter DFU mode.
    async fn upload_serial(package_path: &Path, port: &str) -> Result<()> {
        tracing::info!(
            "Starting DFU serial upload on {} (with 1200-baud touch)",
            port
        );

        let output = tokio::process::Command::new("adafruit-nrfutil")
            .args([
                "dfu",
                "serial",
                "-pkg",
                &package_path.to_string_lossy(),
                "-p",
                port,
                "-b",
                DFU_BAUD_RATE,
                "--singlebank",
                "-t",
                TOUCH_BAUD,
            ])
            .output()
            .await
            .map_err(|e| {
                SensorError::UploadFailed(format!(
                    "Failed to run adafruit-nrfutil: {}. Is it installed?",
                    e
                ))
            })?;

        let combined = Self::combined_output(&output);

        // adafruit-nrfutil may exit with code 0 even on failure,
        // so also check output for error indicators.
        if !output.status.success() || combined.contains("Failed to upgrade") {
            return Err(SensorError::UploadFailed(combined));
        }

        if !combined.is_empty() {
            tracing::info!("nrfutil: {}", combined);
        }

        Ok(())
    }

    /// Combine stdout and stderr into a single trimmed string for error reporting.
    fn combined_output(output: &std::process::Output) -> String {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let mut combined = stdout.trim().to_string();
        let stderr_trimmed = stderr.trim();
        if !stderr_trimmed.is_empty() {
            if !combined.is_empty() {
                combined.push('\n');
            }
            combined.push_str(stderr_trimmed);
        }
        combined
    }
}
