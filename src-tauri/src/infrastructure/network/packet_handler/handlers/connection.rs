/// Connection-related packet handlers (DEVICE_CONNECTED, HEARTBEAT)

use crate::app::EventBus;
use crate::application::dto::DeviceStateDto;
use crate::application::services::ClientApkService;
use crate::domain::commands::{Command, InstallApkCommand};
use crate::domain::models::{Device, DeviceId, Serial};
use crate::domain::repositories::{DeviceNameRepository, DeviceRepository};
use crate::infrastructure::network::device_session_manager::DeviceSessionManager;
use crate::infrastructure::protocol::{opcodes, RawPacket};
use crate::net::io::ProtocolReadExt;
use async_trait::async_trait;
use std::io::Cursor;
use std::sync::Arc;

use super::super::{PacketHandler, Result};

/// Handles DEVICE_CONNECTED (0x01) packets
/// Payload: [model: String][serial: String]
pub struct DeviceConnectedHandler {
    device_repo: Arc<dyn DeviceRepository>,
    device_name_repo: Arc<dyn DeviceNameRepository>,
    event_bus: Arc<EventBus>,
    session_manager: Arc<DeviceSessionManager>,
}

impl DeviceConnectedHandler {
    pub fn new(
        device_repo: Arc<dyn DeviceRepository>,
        device_name_repo: Arc<dyn DeviceNameRepository>,
        event_bus: Arc<EventBus>,
        session_manager: Arc<DeviceSessionManager>,
    ) -> Self {
        Self {
            device_repo,
            device_name_repo,
            event_bus,
            session_manager,
        }
    }

    /// Helper to send initial status requests to a newly connected device
    async fn send_initial_status_requests(device_id: DeviceId, session_manager: Arc<DeviceSessionManager>) {
        // Brief delay to ensure device is ready
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let Some(session) = session_manager.get_session(&device_id) else {
            return;
        };

        // Request battery status
        let _ = session.send_packet(RawPacket {
            opcode: opcodes::REQUEST_BATTERY,
            payload: vec![],
        }).await;

        // Request volume status
        let _ = session.send_packet(RawPacket {
            opcode: opcodes::GET_VOLUME,
            payload: vec![],
        }).await;

        tracing::debug!(device_id = %device_id, "Sent initial battery and volume requests");
    }
}

#[async_trait]
impl PacketHandler for DeviceConnectedHandler {
    fn opcode(&self) -> u8 {
        opcodes::DEVICE_CONNECTED
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);

        // Read device information from packet
        let model = cursor.read_string()?;
        let serial_str = cursor.read_string()?;

        // Get version from session metadata (set during VERSION_CHECK phase)
        let version = self.session_manager
            .get_client_version(&device_id)
            .ok_or_else(|| {
                crate::app::error::ArceusError::DomainValidation(
                    "VERSION_CHECK must be sent before DEVICE_CONNECTED".to_string()
                )
            })?;

        tracing::info!(
            device_id = %device_id,
            model = %model,
            serial = %serial_str,
            version = %version,
            "Device connected packet received"
        );

        let serial = Serial::new(serial_str)
            .map_err(|e| crate::app::error::ArceusError::DomainValidation(format!("Invalid serial: {}", e)))?;

        // Verify session exists (TCP connection was established and VERSION_CHECK was received)
        if !self.session_manager.has_session(&device_id) {
            tracing::warn!(
                device_id = %device_id,
                "DEVICE_CONNECTED received without active session - ignoring"
            );
            return Ok(());
        }

        // Create device with real info from the packet (first time device is created!)
        let device = Device::new(device_id, serial.clone(), model.clone(), version);

        // Load custom name from database if exists
        let custom_name = self.device_name_repo.get_name(&serial).await.ok().flatten();
        let device = device.with_custom_name(custom_name.clone());

        self.device_repo.save(device.clone()).await?;

        tracing::info!(
            device_id = %device_id,
            serial = %serial.as_str(),
            "Device connected"
        );

        // Emit DeviceConnected event to frontend
        let device_state = DeviceStateDto::from(&Arc::new(device.clone()));
        self.event_bus.device_connected(device_state);

        // Request initial connection data
        tokio::spawn(Self::send_initial_status_requests(
            device.id(),
            self.session_manager.clone(),
        ));

        Ok(())
    }
}

/// Handles VERSION_CHECK (0x05) packets
/// Payload: [version: String]
/// This is the first packet sent by a client after TCP connection.
/// Server checks version and either sends VERSION_OK or INSTALL_APK.
pub struct VersionCheckHandler {
    session_manager: Arc<DeviceSessionManager>,
    client_apk_service: Arc<ClientApkService>,
}

impl VersionCheckHandler {
    pub fn new(
        session_manager: Arc<DeviceSessionManager>,
        client_apk_service: Arc<ClientApkService>,
    ) -> Self {
        Self {
            session_manager,
            client_apk_service,
        }
    }

    /// Helper to send a packet to a device
    async fn send_packet(
        &self,
        device_id: &DeviceId,
        packet: RawPacket,
        success_msg: &str,
        error_msg: &str,
    ) -> Result<()> {
        match self.session_manager.get_session(device_id) {
            Some(session) => {
                session.send_packet(packet).await.map_err(|e| {
                    tracing::error!(device_id = %device_id, error = %e, "{}", error_msg);
                    crate::app::error::ArceusError::Network(
                        crate::app::error::NetworkError::SendFailed(e.to_string())
                    )
                })?;
                tracing::debug!(device_id = %device_id, "{}", success_msg);
                Ok(())
            }
            None => {
                tracing::warn!(device_id = %device_id, "Cannot send packet: session not found");
                Err(crate::app::error::ArceusError::Network(
                    crate::app::error::NetworkError::ConnectionFailed(
                        format!("Session not found for device {}", device_id)
                    )
                ))
            }
        }
    }
}

#[async_trait]
impl PacketHandler for VersionCheckHandler {
    fn opcode(&self) -> u8 {
        opcodes::VERSION_CHECK
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);
        let version = cursor.read_string()?;

        tracing::info!(
            device_id = %device_id,
            version = %version,
            "VERSION_CHECK received from client"
        );

        // Store version in session metadata for later use
        self.session_manager.set_client_version(&device_id, version.clone());

        // Check if client needs update
        if self.client_apk_service.should_update_client(&version).await {
            let apk_url = self.client_apk_service.get_download_url();

            tracing::info!(
                device_id = %device_id,
                current_version = %version,
                apk_url = %apk_url,
                "Client is outdated - sending INSTALL_APK command"
            );

            // Send INSTALL_APK command
            let install_cmd = InstallApkCommand::new(apk_url.clone());
            let _ = self.send_packet(
                &device_id,
                RawPacket {
                    opcode: install_cmd.opcode(),
                    payload: install_cmd.serialize()?,
                },
                "INSTALL_APK command sent - client will update and reconnect",
                "Failed to send INSTALL_APK command",
            ).await;
        } else {
            // Client is up to date - send VERSION_OK
            tracing::info!(
                device_id = %device_id,
                version = %version,
                "Client version is up to date - sending VERSION_OK"
            );

            let _ = self.send_packet(
                &device_id,
                RawPacket {
                    opcode: opcodes::VERSION_OK,
                    payload: vec![],
                },
                "VERSION_OK sent - awaiting DEVICE_CONNECTED",
                "Failed to send VERSION_OK",
            ).await;
        }

        Ok(())
    }
}

/// Handles HEARTBEAT (0x02) packets
/// No payload
pub struct HeartbeatHandler {}

impl HeartbeatHandler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl PacketHandler for HeartbeatHandler {
    fn opcode(&self) -> u8 {
        opcodes::HEARTBEAT
    }

    async fn handle(&self, device_id: DeviceId, _payload: Vec<u8>) -> Result<()> {
        tracing::trace!(device_id = %device_id, "Heartbeat received");

        // Update last_seen is already handled in the message loop
        // This handler just acknowledges the heartbeat

        Ok(())
    }
}
