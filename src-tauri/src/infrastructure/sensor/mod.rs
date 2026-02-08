/// Sensor/Arduino management module
/// Handles XIAO BLE nRF52840 device detection, communication, and firmware upload

mod detector;
mod dfu;
mod patcher;
mod serial_comm;

pub use detector::{XiaoDetector, XiaoMode};
pub use dfu::DfuUploader;
pub use patcher::FirmwarePatcher;
pub use serial_comm::SerialComm;

use thiserror::Error;

/// XIAO BLE nRF52840 USB identifiers
pub const XIAO_VID: u16 = 0x2886;
pub const XIAO_NORMAL_PID: u16 = 0x8045;
pub const XIAO_BOOTLOADER_PID: u16 = 0x0044;

#[derive(Error, Debug)]
pub enum SensorError {
    #[error("No XIAO devices found")]
    NoDeviceFound,

    #[error("Serial port error: {0}")]
    SerialPort(#[from] serialport::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse device response: {0}")]
    ParseError(String),

    #[error("Firmware placeholder not found")]
    PlaceholderNotFound,

    #[error("Device name too long (max {max} characters)")]
    NameTooLong { max: usize },

    #[error("Firmware upload failed: {0}")]
    UploadFailed(String),
}

pub type Result<T> = std::result::Result<T, SensorError>;
