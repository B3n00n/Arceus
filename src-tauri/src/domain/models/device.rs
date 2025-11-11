/// This is an immutable aggregate - all mutations return new instances.
use super::{Battery, DeviceId, IpAddress, Serial, Volume};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Device aggregate - the root entity for a connected device
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// Unique device identifier
    id: DeviceId,
    /// Device serial number (MAC address)
    serial: Serial,
    /// Device model name (e.g., "Meta Quest 3")
    model: String,
    /// IP address of the device
    ip: IpAddress,
    /// When the device first connected
    connected_at: DateTime<Utc>,
    /// When the device was last seen (heartbeat)
    last_seen: DateTime<Utc>,
    /// Optional custom name set by the user
    custom_name: Option<String>,
    /// Battery information (if available)
    battery: Option<Battery>,
    /// Volume information (if available)
    volume: Option<Volume>,
}

impl Device {
    pub fn new(id: DeviceId, serial: Serial, model: String, ip: IpAddress) -> Self {
        let now = Utc::now();
        Self {
            id,
            serial,
            model,
            ip,
            connected_at: now,
            last_seen: now,
            custom_name: None,
            battery: None,
            volume: None,
        }
    }

    pub fn id(&self) -> DeviceId {
        self.id
    }

    pub fn serial(&self) -> &Serial {
        &self.serial
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn ip(&self) -> &IpAddress {
        &self.ip
    }

    pub fn connected_at(&self) -> DateTime<Utc> {
        self.connected_at
    }

    pub fn custom_name(&self) -> Option<&str> {
        self.custom_name.as_deref()
    }

    pub fn battery(&self) -> Option<&Battery> {
        self.battery.as_ref()
    }

    pub fn volume(&self) -> Option<&Volume> {
        self.volume.as_ref()
    }

    /// Update the last seen timestamp (called on heartbeat)
    pub fn update_last_seen(mut self) -> Self {
        self.last_seen = Utc::now();
        self
    }

    /// Set a custom name for the device
    pub fn with_custom_name(mut self, name: Option<String>) -> Self {
        self.custom_name = name;
        self
    }

    /// Update battery information
    pub fn with_battery(mut self, battery: Battery) -> Self {
        self.battery = Some(battery);
        self.last_seen = Utc::now();
        self
    }

    /// Update volume information
    pub fn with_volume(mut self, volume: Volume) -> Self {
        self.volume = Some(volume);
        self.last_seen = Utc::now();
        self
    }
}
