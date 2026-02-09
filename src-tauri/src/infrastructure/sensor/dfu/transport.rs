/// Serial transport layer for Nordic DFU.
///
/// Handles the 1200-baud touch to enter bootloader mode, then communicates
/// at 115200 baud using HCI-framed packets.

use super::hci::HciSequence;
use super::slip::slip_decode;
use super::{SensorError, XiaoDetector, XiaoMode};
use serialport::{ClearBuffer, SerialPort};
use std::time::{Duration, Instant};

const DFU_BAUD_RATE: u32 = 115200;
const TOUCH_BAUD_RATE: u32 = 1200;
const SLIP_END: u8 = 0xC0;

/// How long to poll for the bootloader device after 1200-baud touch.
const BOOTLOADER_SCAN_TIMEOUT: Duration = Duration::from_secs(10);
/// Interval between scans for the bootloader device.
const BOOTLOADER_SCAN_INTERVAL: Duration = Duration::from_millis(250);

pub struct DfuTransport {
    port: Box<dyn SerialPort>,
    seq: HciSequence,
}

impl DfuTransport {
    /// Open a serial port for DFU with 1200-baud touch to enter bootloader.
    pub fn open_with_touch(port_name: &str) -> Result<Self, SensorError> {
        // 1200-baud touch to trigger bootloader entry
        tracing::info!("Sending 1200-baud touch on {}", port_name);
        {
            let _touch = serialport::new(port_name, TOUCH_BAUD_RATE)
                .timeout(Duration::from_millis(500))
                .open()
                .map_err(|e| {
                    SensorError::UploadFailed(format!(
                        "Failed to open {} for 1200-baud touch: {}",
                        port_name, e
                    ))
                })?;
            std::thread::sleep(Duration::from_millis(100));
            // _touch drops here, closing the port
        }

        // Wait for device to re-enumerate in bootloader mode.
        // The device disconnects (normal PID) and reconnects (bootloader PID),
        // possibly on a different port name. Poll until we find it.
        let bootloader_port = Self::wait_for_bootloader()?;

        tracing::info!(
            "Opening {} at {} baud for DFU",
            bootloader_port,
            DFU_BAUD_RATE
        );
        let port = serialport::new(&bootloader_port, DFU_BAUD_RATE)
            .timeout(Duration::from_millis(30_000))
            .open()
            .map_err(|e| {
                SensorError::UploadFailed(format!(
                    "Failed to open {} for DFU upload: {}",
                    bootloader_port, e
                ))
            })?;

        std::thread::sleep(Duration::from_millis(100));

        Ok(Self {
            port,
            seq: HciSequence::new(),
        })
    }

    /// Poll for a XIAO device in bootloader mode after 1200-baud touch.
    fn wait_for_bootloader() -> Result<String, SensorError> {
        let start = Instant::now();

        // Give the device a moment to begin re-enumeration
        std::thread::sleep(Duration::from_millis(500));

        loop {
            let devices = XiaoDetector::find_all();
            if let Some(bl) = devices.iter().find(|d| d.mode == XiaoMode::Bootloader) {
                tracing::info!(
                    "Found bootloader on {} ({:.1}s after touch)",
                    bl.port,
                    start.elapsed().as_secs_f32()
                );
                return Ok(bl.port.clone());
            }

            if start.elapsed() > BOOTLOADER_SCAN_TIMEOUT {
                return Err(SensorError::UploadFailed(
                    "Bootloader device did not appear after 1200-baud touch (10s timeout)"
                        .to_string(),
                ));
            }

            std::thread::sleep(BOOTLOADER_SCAN_INTERVAL);
        }
    }

    /// Send an HCI-framed packet and wait for ACK.
    ///
    /// Matches Python nrfutil behavior: send packet, read one SLIP frame as ACK,
    /// don't validate the ACK number.
    pub fn send_and_ack(&mut self, payload: &[u8]) -> Result<(), SensorError> {
        let packet = self.seq.build_packet(payload);

        // Discard any stale data so we read the real response.
        self.port.clear(ClearBuffer::Input).ok();

        self.port.write_all(&packet).map_err(|e| {
            SensorError::UploadFailed(format!("Serial write failed: {}", e))
        })?;
        self.port.flush().map_err(|e| {
            SensorError::UploadFailed(format!("Serial flush failed: {}", e))
        })?;

        let _ack = self.read_slip_frame()?;
        Ok(())
    }

    /// Read a single SLIP frame: bytes between two 0xC0 delimiters, SLIP-decoded.
    fn read_slip_frame(&mut self) -> Result<Vec<u8>, SensorError> {
        let mut buf = [0u8; 1];

        // Skip until first SLIP_END
        loop {
            match self.port.read(&mut buf) {
                Ok(1) if buf[0] == SLIP_END => break,
                Ok(1) => continue,
                Ok(_) => continue,
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    return Err(SensorError::UploadFailed(
                        "Timeout waiting for DFU response".to_string(),
                    ));
                }
                Err(e) => {
                    return Err(SensorError::UploadFailed(format!(
                        "Serial read failed: {}",
                        e
                    )));
                }
            }
        }

        // Read until next SLIP_END
        let mut raw = Vec::with_capacity(32);
        loop {
            match self.port.read(&mut buf) {
                Ok(1) if buf[0] == SLIP_END => break,
                Ok(1) => raw.push(buf[0]),
                Ok(_) => continue,
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    return Err(SensorError::UploadFailed(
                        "Timeout reading DFU response body".to_string(),
                    ));
                }
                Err(e) => {
                    return Err(SensorError::UploadFailed(format!(
                        "Serial read failed: {}",
                        e
                    )));
                }
            }
        }

        Ok(slip_decode(&raw))
    }

    /// Consume the transport, closing the port.
    pub fn close(self) {
        drop(self.port);
    }
}
