use crate::core::error::{ProtocolError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageType {
    // Client → Server Messages (0x01-0x0F)
    /// Payload: [model: String, serial: String]
    DeviceConnected = 0x01,

    /// Payload: empty
    Heartbeat = 0x02,

    /// Payload: [headset_level: u8, is_charging: u8]
    BatteryStatus = 0x03,

    /// Payload: [success: u8, message: String]
    CommandResponse = 0x04,

    /// Payload: [message: String]
    Error = 0x05,

    /// Payload: [volume_percentage: u8, current_volume: u8, max_volume: u8]
    VolumeStatus = 0x06,

    // ===== Server → Client Commands (0x10-0x1F) =====
    /// Payload: [package_name: String]
    LaunchApp = 0x10,

    /// Payload: [command: String]
    ExecuteShell = 0x12,

    /// Payload: empty
    RequestBattery = 0x13,

    /// Payload: empty
    GetInstalledApps = 0x14,

    /// Payload: empty
    GetDeviceInfo = 0x15,

    /// Payload: empty
    Ping = 0x16,

    /// Payload: [url: String]
    DownloadAndInstallApk = 0x17,

    /// Payload: empty (restart)
    ShutdownDevice = 0x18,

    /// Payload: [package_name: String]
    UninstallApp = 0x19,

    /// Payload: [volume: u8] (0-100)
    SetVolume = 0x1A,

    /// Payload: empty
    GetVolume = 0x1B,

    /// Payload: [url: String]
    InstallLocalApk = 0x1C,
}

impl MessageType {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0x01 => Ok(MessageType::DeviceConnected),
            0x02 => Ok(MessageType::Heartbeat),
            0x03 => Ok(MessageType::BatteryStatus),
            0x04 => Ok(MessageType::CommandResponse),
            0x05 => Ok(MessageType::Error),
            0x06 => Ok(MessageType::VolumeStatus),
            0x10 => Ok(MessageType::LaunchApp),
            0x12 => Ok(MessageType::ExecuteShell),
            0x13 => Ok(MessageType::RequestBattery),
            0x14 => Ok(MessageType::GetInstalledApps),
            0x15 => Ok(MessageType::GetDeviceInfo),
            0x16 => Ok(MessageType::Ping),
            0x17 => Ok(MessageType::DownloadAndInstallApk),
            0x18 => Ok(MessageType::ShutdownDevice),
            0x19 => Ok(MessageType::UninstallApp),
            0x1A => Ok(MessageType::SetVolume),
            0x1B => Ok(MessageType::GetVolume),
            0x1C => Ok(MessageType::InstallLocalApk),
            _ => Err(ProtocolError::InvalidMessageType(value).into()),
        }
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }

    pub fn is_client_message(self) -> bool {
        matches!(
            self,
            MessageType::DeviceConnected
                | MessageType::Heartbeat
                | MessageType::BatteryStatus
                | MessageType::CommandResponse
                | MessageType::Error
                | MessageType::VolumeStatus
        )
    }

    pub fn is_server_message(self) -> bool {
        !self.is_client_message()
    }

    pub fn name(self) -> &'static str {
        match self {
            MessageType::DeviceConnected => "DeviceConnected",
            MessageType::Heartbeat => "Heartbeat",
            MessageType::BatteryStatus => "BatteryStatus",
            MessageType::CommandResponse => "CommandResponse",
            MessageType::Error => "Error",
            MessageType::VolumeStatus => "VolumeStatus",
            MessageType::LaunchApp => "LaunchApp",
            MessageType::ExecuteShell => "ExecuteShell",
            MessageType::RequestBattery => "RequestBattery",
            MessageType::GetInstalledApps => "GetInstalledApps",
            MessageType::GetDeviceInfo => "GetDeviceInfo",
            MessageType::Ping => "Ping",
            MessageType::DownloadAndInstallApk => "DownloadAndInstallApk",
            MessageType::ShutdownDevice => "ShutdownDevice",
            MessageType::UninstallApp => "UninstallApp",
            MessageType::SetVolume => "SetVolume",
            MessageType::GetVolume => "GetVolume",
            MessageType::InstallLocalApk => "InstallLocalApk",
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl TryFrom<u8> for MessageType {
    type Error = crate::core::error::ArceusError;

    fn try_from(value: u8) -> Result<Self> {
        MessageType::from_u8(value)
    }
}

impl From<MessageType> for u8 {
    fn from(msg_type: MessageType) -> Self {
        msg_type.to_u8()
    }
}

