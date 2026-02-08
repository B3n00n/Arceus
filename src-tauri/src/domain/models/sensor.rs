/// Sensor (XIAO BLE nRF52840) domain model

use serde::{Deserialize, Serialize};

/// Connection status of a sensor
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SensorConnectionStatus {
    /// Sensor is connected and communicating
    Connected,
    /// Sensor is in bootloader mode
    Bootloader,
    /// Sensor is disconnected
    Disconnected,
}

/// Represents a XIAO BLE nRF52840 sensor board
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sensor {
    /// Serial port path (e.g., "/dev/ttyACM0" or "COM3")
    pub port: String,

    /// Hardware serial number from nRF52840 FICR
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,

    /// Hardware MAC address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,

    /// BLE MAC address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ble_mac_address: Option<String>,

    /// Current BLE device name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,

    /// Firmware version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,

    /// Connection status
    pub status: SensorConnectionStatus,
}

impl Sensor {
    /// Create a new sensor with just port info
    pub fn new(port: String, status: SensorConnectionStatus) -> Self {
        Self {
            port,
            serial_number: None,
            mac_address: None,
            ble_mac_address: None,
            device_name: None,
            firmware_version: None,
            status,
        }
    }

    /// Create a sensor from detected port info
    pub fn from_port(port: &str, in_bootloader: bool) -> Self {
        Self::new(
            port.to_string(),
            if in_bootloader {
                SensorConnectionStatus::Bootloader
            } else {
                SensorConnectionStatus::Connected
            },
        )
    }

    /// Populate optional fields from individual values (used after serial info read)
    pub fn with_info(
        mut self,
        serial_number: Option<String>,
        mac_address: Option<String>,
        ble_mac_address: Option<String>,
        device_name: Option<String>,
        firmware_version: Option<String>,
    ) -> Self {
        self.serial_number = serial_number;
        self.mac_address = mac_address;
        self.ble_mac_address = ble_mac_address;
        self.device_name = device_name;
        self.firmware_version = firmware_version;
        self
    }
}
