/// Connection-related packet handlers (DEVICE_CONNECTED, HEARTBEAT)

use crate::app::EventBus;
use crate::application::dto::DeviceStateDto;
use crate::application::services::ClientApkService;
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
/// Payload: [model: String][serial: String][version: String]
pub struct DeviceConnectedHandler {
    device_repo: Arc<dyn DeviceRepository>,
    device_name_repo: Arc<dyn DeviceNameRepository>,
    event_bus: Arc<EventBus>,
    session_manager: Arc<DeviceSessionManager>,
    client_apk_service: Arc<ClientApkService>,
}

impl DeviceConnectedHandler {
    pub fn new(
        device_repo: Arc<dyn DeviceRepository>,
        device_name_repo: Arc<dyn DeviceNameRepository>,
        event_bus: Arc<EventBus>,
        session_manager: Arc<DeviceSessionManager>,
        client_apk_service: Arc<ClientApkService>,
    ) -> Self {
        Self {
            device_repo,
            device_name_repo,
            event_bus,
            session_manager,
            client_apk_service,
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

        let model = cursor.read_string()?;
        let serial_str = cursor.read_string()?;
        let version = cursor.read_string()?;

        tracing::info!(
            device_id = %device_id,
            model = %model,
            serial = %serial_str,
            version = %version,
            "Device connected packet received"
        );

        let serial = Serial::new(serial_str)
            .map_err(|e| crate::app::error::ArceusError::DomainValidation(format!("Invalid serial: {}", e)))?;

        // Verify device exists (was created on TCP connection)
        if self.device_repo.find_by_id(device_id).await.ok().flatten().is_none() {
            tracing::warn!("Device connected packet without prior TCP connection");
            return Ok(());
        }

        // Create device with real info from the packet
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

        // Check if client needs update and mark device accordingly
        let version_str = device.version();
        let update_available = self.client_apk_service.should_update_client(version_str).await;

        if update_available {
            tracing::info!(
                device_id = %device_id,
                current_version = %version_str,
                "Client update available (manual update required)"
            );
        }

        let device = device.with_update_available(update_available);
        self.device_repo.save(device.clone()).await?;

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
