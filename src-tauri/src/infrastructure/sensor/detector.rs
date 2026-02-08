/// USB device detection for XIAO BLE nRF52840 boards

use super::{Result, SensorError, XIAO_BOOTLOADER_PID, XIAO_NORMAL_PID, XIAO_VID};

/// Operating mode of a XIAO board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XiaoMode {
    /// Normal application mode - serial communication available
    Normal,
    /// DFU bootloader mode - serial DFU upload available
    Bootloader,
}

/// A detected XIAO port
#[derive(Debug, Clone)]
pub struct XiaoPort {
    /// Serial port path (e.g., "/dev/ttyACM0" or "COM3")
    pub port: String,
    /// Operating mode
    pub mode: XiaoMode,
}

/// Detects connected XIAO BLE nRF52840 boards
pub struct XiaoDetector;

impl XiaoDetector {
    /// Find all connected XIAO boards
    pub fn find_all() -> Vec<XiaoPort> {
        let mut devices = Vec::new();

        if let Ok(ports) = serialport::available_ports() {
            for port in ports {
                if let serialport::SerialPortType::UsbPort(usb_info) = &port.port_type {
                    if usb_info.vid == XIAO_VID {
                        let mode = if usb_info.pid == XIAO_NORMAL_PID {
                            XiaoMode::Normal
                        } else if usb_info.pid == XIAO_BOOTLOADER_PID {
                            XiaoMode::Bootloader
                        } else {
                            continue;
                        };

                        devices.push(XiaoPort {
                            port: port.port_name,
                            mode,
                        });
                    }
                }
            }
        }

        devices
    }

    /// Find XIAO boards in normal mode only
    pub fn find_normal() -> Vec<XiaoPort> {
        Self::find_all()
            .into_iter()
            .filter(|d| d.mode == XiaoMode::Normal)
            .collect()
    }

    /// Find the first available XIAO in normal mode
    pub fn find_first() -> Result<XiaoPort> {
        Self::find_normal()
            .into_iter()
            .next()
            .ok_or(SensorError::NoDeviceFound)
    }
}
