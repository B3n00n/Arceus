/// Connection Handler
/// Manages device lifecycle for a single connection.
use crate::app::{EventBus, Result};
use crate::domain::models::DeviceId;
use crate::domain::repositories::DeviceRepository;
use crate::infrastructure::network::device_session::DeviceSession;
use crate::infrastructure::network::device_session_manager::DeviceSessionManager;
use crate::infrastructure::network::packet_handler::PacketHandlerRegistry;
use crate::infrastructure::protocol::RawPacket;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

/// Handles the lifecycle of a device connection
pub struct ConnectionHandler {
    device_repo: Arc<dyn DeviceRepository>,
    event_bus: Arc<EventBus>,
    packet_handler: Arc<PacketHandlerRegistry>,
    session_manager: Arc<DeviceSessionManager>,
    heartbeat_timeout: Duration,
}

impl ConnectionHandler {
    pub fn new(
        device_repo: Arc<dyn DeviceRepository>,
        event_bus: Arc<EventBus>,
        packet_handler: Arc<PacketHandlerRegistry>,
        session_manager: Arc<DeviceSessionManager>,
        heartbeat_timeout: Duration,
    ) -> Self {
        Self {
            device_repo,
            event_bus,
            packet_handler,
            session_manager,
            heartbeat_timeout,
        }
    }

    /// Handle a complete device connection lifecycle
    pub async fn handle_connection(
        &self,
        stream: tokio::net::TcpStream,
        addr: SocketAddr,
    ) -> Result<()> {
        let span = tracing::info_span!("connection", %addr);
        let _enter = span.enter();

        tracing::info!("New connection established");

        // Create device ID and session only (no device yet)
        // Device will be created when DEVICE_CONNECTED packet is received
        let device_id = DeviceId::new();
        let session = self.register_session(device_id, stream, addr).await?;

        tracing::debug!(
            device_id = %device_id,
            "Session registered - awaiting VERSION_CHECK"
        );

        // Run message loop
        drop(_enter);
        let result = self.message_loop(device_id, session).await;

        // Cleanup
        let _enter = span.enter();
        self.cleanup_device(device_id).await;

        result
    }

    /// Register a session without creating a device
    /// Device will be created when DEVICE_CONNECTED packet is received
    async fn register_session(
        &self,
        device_id: DeviceId,
        stream: tokio::net::TcpStream,
        addr: SocketAddr,
    ) -> Result<Arc<DeviceSession>> {
        let session = Arc::new(DeviceSession::new(stream, device_id, addr));
        self.session_manager.add_session(device_id, session.clone());
        Ok(session)
    }

    async fn message_loop(
        &self,
        device_id: DeviceId,
        session: Arc<DeviceSession>,
    ) -> Result<()> {
        let span = tracing::debug_span!("message_loop", device_id = %device_id);
        let _enter = span.enter();

        tracing::debug!(
            timeout_secs = self.heartbeat_timeout.as_secs(),
            "Starting message loop"
        );

        loop {
            let packet_result = timeout(self.heartbeat_timeout, session.receive_packet()).await;

            match packet_result {
                Ok(Ok(Some(packet))) => {
                    // Update device last_seen timestamp
                    self.update_last_seen(device_id).await;

                    // Handle the packet
                    if let Err(e) = self.handle_packet(device_id, &session, packet).await {
                        tracing::error!(
                            device_id = %device_id,
                            error = %e,
                            "Error handling packet"
                        );
                    }
                }
                Ok(Ok(None)) => {
                    tracing::debug!(device_id = %device_id, "Connection closed by device");
                    break;
                }
                Ok(Err(e)) => {
                    tracing::error!(
                        device_id = %device_id,
                        error = %e,
                        "Error receiving packet"
                    );
                    break;
                }
                Err(_) => {
                    tracing::warn!(
                        device_id = %device_id,
                        timeout_secs = self.heartbeat_timeout.as_secs(),
                        "Heartbeat timeout"
                    );
                    break;
                }
            }
        }

        Ok(())
    }

    async fn update_last_seen(&self, device_id: DeviceId) {
        if let Ok(Some(device)) = self.device_repo.find_by_id(device_id).await {
            let updated = device.as_ref().clone().update_last_seen();
            let _ = self.device_repo.save(updated).await;
        }
    }

    async fn handle_packet(
        &self,
        device_id: DeviceId,
        _session: &DeviceSession,
        packet: RawPacket,
    ) -> Result<()> {
        self.packet_handler
            .handle(device_id, packet)
            .await
    }

    async fn cleanup_device(&self, device_id: DeviceId) {
        let device_info = self.device_repo.find_by_id(device_id).await.ok().flatten();

        self.session_manager.remove_session(&device_id);

        let _ = self.device_repo.remove(device_id).await;

        if let Some(device) = device_info {
            tracing::info!(
                device_id = %device_id,
                serial = %device.serial().as_str(),
                "Device disconnected"
            );
            self.event_bus.emit(crate::app::events::ArceusEvent::DeviceDisconnected {
                device_id: device_id.as_uuid().clone(),
                serial: device.serial().as_str().to_string(),
            });
        }

        tracing::debug!(device_id = %device_id, "Device removed");
    }
}
