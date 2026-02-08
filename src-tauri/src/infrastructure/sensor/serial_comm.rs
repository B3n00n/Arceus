/// Serial communication with XIAO BLE nRF52840 boards

use super::{Result, SensorError};
use serialport::SerialPort;
use std::io::{Read, Write};
use std::time::Duration;

/// Device information read from a sensor
#[derive(Debug, Clone, Default)]
pub struct SensorInfo {
    /// Hardware serial number (from nRF52840 FICR)
    pub serial_number: Option<String>,
    /// Hardware MAC address
    pub mac_address: Option<String>,
    /// BLE MAC address
    pub ble_mac_address: Option<String>,
    /// Current device name (if readable)
    pub device_name: Option<String>,
    /// Firmware version
    pub firmware_version: Option<String>,
}

/// Serial communication handler
pub struct SerialComm {
    port: Box<dyn SerialPort>,
}

impl SerialComm {
    /// Open a serial connection to a XIAO board
    fn open(port_name: &str) -> Result<Self> {
        let mut port = serialport::new(port_name, 115200)
        .timeout(Duration::from_secs(5))
        .open()?;

    port.write_data_terminal_ready(true)?;

    let ready_delay = if cfg!(target_os = "windows") { 2000 } else { 500 };
    std::thread::sleep(Duration::from_millis(ready_delay));

    Ok(Self {port})
    }

    /// Send a command and read the response.
    ///
    /// The device continuously sends sensor readings (~every 50ms), so a
    /// blocking read loop would never time out. Instead, after giving the
    /// device time to process, we drain only what's already buffered using
    /// `bytes_to_read()` — no waiting, no firmware-format coupling.
    fn send_command(&mut self, command: &str) -> Result<String> {
        // Discard any stale input data (instant, uses tcflush)
        self.port.clear(serialport::ClearBuffer::Input)?;

        let cmd = format!("{}\n", command);
        self.port.write_all(cmd.as_bytes())?;

        // The device processes and responds in microseconds. After 500ms
        // the complete response (plus some sensor readings) is sitting in
        // the kernel buffer — ready to be drained without blocking.
        std::thread::sleep(Duration::from_millis(500));

        // Read everything currently buffered, don't wait for more.
        let mut buffer = vec![0u8; 4096];
        let mut response = String::new();

        while self.port.bytes_to_read().unwrap_or(0) > 0 {
            match self.port.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => response.push_str(&String::from_utf8_lossy(&buffer[..n])),
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => break,
                Err(e) => return Err(SensorError::Io(e)),
            }
        }

        Ok(response)
    }

    /// Get complete device information
    fn get_device_info(&mut self) -> Result<SensorInfo> {
        let response = self.send_command("INFO")?;
        tracing::debug!(response = %response, "Raw INFO response");

        let mut info = SensorInfo::default();

        for line in response.lines() {
            let line = line.trim();

            if let Some(value) = line.strip_prefix("Device Name:") {
                let value = value.trim();
                if !value.is_empty() {
                    info.device_name = Some(value.to_string());
                }
            } else if let Some(value) = line.strip_prefix("Firmware:") {
                let value = value.trim();
                if !value.is_empty() {
                    info.firmware_version = Some(value.to_string());
                }
            } else if let Some(value) = line.strip_prefix("SERIAL:") {
                let value = value.trim();
                if !value.is_empty() && value != "Not available" {
                    info.serial_number = Some(value.to_uppercase());
                }
            } else if let Some(value) = line.strip_prefix("MAC:") {
                let value = value.trim();
                if !value.is_empty() && value != "Not available" {
                    info.mac_address = Some(value.to_uppercase());
                }
            } else if let Some(value) = line.strip_prefix("BLE_MAC:") {
                let value = value.trim();
                if !value.is_empty() && value != "Not available" {
                    info.ble_mac_address = Some(value.to_uppercase());
                }
            }
        }

        Ok(info)
    }

    pub fn open_and_get_info(port_name: &str, max_retries: u32) -> Result<SensorInfo> {
        let mut last_error = None;

        for attempt in 0..max_retries {
            match SerialComm::open(port_name) {
                Ok(mut serial) => match serial.get_device_info() {
                    Ok(info) if info.serial_number.is_some() => return Ok(info),
                    Ok(_) => {
                        tracing::debug!(
                            "Attempt {}/{}: No serial number in response",
                            attempt + 1,
                            max_retries
                        );
                    }
                    Err(e) => {
                        tracing::debug!(
                            "Attempt {}/{}: Read error: {}",
                            attempt + 1,
                            max_retries,
                            e
                        );
                        last_error = Some(e);
                    }
                },
                Err(e) => {
                    tracing::debug!(
                        "Attempt {}/{}: Open error: {}",
                        attempt + 1,
                        max_retries,
                        e
                    );
                    last_error = Some(e);
                }
            }

            if attempt < max_retries - 1 {
                std::thread::sleep(Duration::from_secs(1));
            }
        }

        Err(last_error.unwrap_or(SensorError::ParseError(
            "Could not read device info after retries".to_string(),
        )))
    }
}
