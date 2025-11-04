/// TCP Server for device connections
/// Accepts incoming TCP connections from devices and manages their lifecycle.

use crate::core::{error::NetworkError, EventBus, Result, ServerConfig};
use crate::domain::models::{Device, DeviceId, IpAddress, Serial};
use crate::domain::repositories::{DeviceNameRepository, DeviceRepository};
use crate::infrastructure::network::device_session::DeviceSession;
use crate::infrastructure::network::packet_handler::PacketHandlerRegistry;
use crate::infrastructure::network::session_manager::SessionManager;
use crate::protocol::RawPacket;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, RwLock};
use tokio::time::timeout;

/// TCP server that accepts and manages device connections
pub struct TcpServer {
    config: ServerConfig,
    device_repo: Arc<dyn DeviceRepository>,
    device_name_repo: Arc<dyn DeviceNameRepository>,
    event_bus: Arc<EventBus>,
    packet_handler: Arc<PacketHandlerRegistry>,
    session_manager: Arc<SessionManager>,
    running: Arc<RwLock<bool>>,
    shutdown_tx: broadcast::Sender<()>,
}

impl TcpServer {
    pub fn new(
        config: ServerConfig,
        device_repo: Arc<dyn DeviceRepository>,
        device_name_repo: Arc<dyn DeviceNameRepository>,
        event_bus: Arc<EventBus>,
    ) -> (Self, broadcast::Receiver<()>, Arc<SessionManager>) {
        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);

        // Create session manager (needed for command execution)
        let session_manager = Arc::new(SessionManager::new());

        // Create packet handler registry
        let packet_handler = Arc::new(PacketHandlerRegistry::new(
            device_repo.clone(),
            device_name_repo.clone(),
            event_bus.clone(),
            session_manager.clone(),
        ));

        let server = Self {
            config,
            device_repo,
            device_name_repo,
            event_bus,
            packet_handler,
            session_manager: session_manager.clone(),
            running: Arc::new(RwLock::new(false)),
            shutdown_tx,
        };

        (server, shutdown_rx, session_manager)
    }

    /// Start the TCP server
    pub async fn start(self: Arc<Self>) -> Result<()> {
        let addr = format!("{}:{}", self.config.tcp_host, self.config.tcp_port);
        let listener = TcpListener::bind(&addr).await.map_err(|e| {
            NetworkError::BindError(format!("Failed to bind to {}: {}", addr, e))
        })?;

        tracing::info!(
            tcp_host = %self.config.tcp_host,
            tcp_port = self.config.tcp_port,
            "TCP server listening"
        );

        *self.running.write().await = true;
        self.event_bus
            .server_started(self.config.tcp_port, self.config.http_port);

        let mut shutdown_rx = self.shutdown_tx.subscribe();

        loop {
            tokio::select! {
                accept_result = listener.accept() => {
                    match accept_result {
                        Ok((stream, addr)) => {
                            // Check capacity
                            let current_count = self.device_repo.count().await.unwrap_or(0);
                            if current_count >= self.config.max_connections {
                                tracing::warn!(
                                    addr = %addr,
                                    current = current_count,
                                    max = self.config.max_connections,
                                    "Connection limit reached, rejecting connection"
                                );
                                drop(stream);
                                continue;
                            }

                            let server = Arc::clone(&self);
                            tokio::spawn(async move {
                                if let Err(e) = server.handle_connection(stream, addr).await {
                                    tracing::error!(
                                        addr = %addr,
                                        error = %e,
                                        "Error handling connection"
                                    );
                                }
                            });
                        }
                        Err(e) => {
                            tracing::error!(error = %e, "Failed to accept connection");
                            continue;
                        }
                    }
                }
                _ = shutdown_rx.recv() => {
                    tracing::info!("Shutdown signal received, stopping TCP server");
                    break;
                }
            }
        }

        *self.running.write().await = false;
        tracing::info!("TCP server stopped");
        self.event_bus.server_stopped();
        Ok(())
    }

    /// Handle a single device connection
    async fn handle_connection(&self, stream: tokio::net::TcpStream, addr: SocketAddr) -> Result<()> {
        let span = tracing::info_span!("connection", %addr);
        let _enter = span.enter();

        tracing::info!("New connection established");

        // Wait for initial device connection packet
        // For now, we'll create a basic device entry
        // TODO: This should be updated when we receive the actual device info packet

        let device_id = DeviceId::new();
        let serial = Serial::new(format!(
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            addr.ip().to_string().bytes().nth(0).unwrap_or(0),
            addr.ip().to_string().bytes().nth(1).unwrap_or(0),
            addr.ip().to_string().bytes().nth(2).unwrap_or(0),
            addr.ip().to_string().bytes().nth(3).unwrap_or(0),
            (addr.port() >> 8) as u8,
            (addr.port() & 0xFF) as u8,
        )).map_err(|e| NetworkError::ConnectionFailed(format!("Invalid serial: {}", e)))?;

        let ip = IpAddress::new(addr.ip().to_string())
            .map_err(|e| NetworkError::ConnectionFailed(format!("Invalid IP: {}", e)))?;

        // Create device session
        let session = Arc::new(DeviceSession::new(stream, device_id, addr));

        // Add session to manager
        self.session_manager.add_session(device_id, session.clone());

        // Create device in repository
        let device = Device::new(
            device_id,
            serial.clone(),
            "Unknown".to_string(), // Model will be updated when we receive device info
            ip,
        );

        // Load custom name if exists
        let custom_name = self.device_name_repo.get_name(&serial).await.ok().flatten();
        let device = if let Some(name) = custom_name {
            device.with_custom_name(Some(name))
        } else {
            device
        };

        self.device_repo.save(device.clone()).await?;

        tracing::debug!(
            device_id = %device_id,
            serial = %serial.as_str(),
            "Device registered"
        );

        // Run message loop
        drop(_enter);
        let result = self.message_loop(device_id, session).await;

        // Cleanup
        let _enter = span.enter();

        // Get device info before removal for the disconnect event
        let device_info = self.device_repo.find_by_id(device_id).await.ok().flatten();

        // Remove session from manager
        self.session_manager.remove_session(&device_id);

        // Remove device from repository
        let _ = self.device_repo.remove(device_id).await;

        // Emit DeviceDisconnected event
        if let Some(device) = device_info {
            tracing::info!(
                device_id = %device_id,
                serial = %device.serial().as_str(),
                "Device disconnected"
            );
            self.event_bus.emit(crate::core::events::ArceusEvent::DeviceDisconnected {
                device_id: device_id.as_uuid().clone(),
                serial: device.serial().as_str().to_string(),
            });
        }

        tracing::debug!(
            device_id = %device_id,
            "Device removed"
        );

        result
    }

    /// Message loop for a device connection
    async fn message_loop(&self, device_id: DeviceId, session: Arc<DeviceSession>) -> Result<()> {
        let heartbeat_timeout = Duration::from_secs(self.config.heartbeat_timeout);

        let span = tracing::debug_span!("message_loop", device_id = %device_id);
        let _enter = span.enter();

        tracing::debug!(
            timeout_secs = self.config.heartbeat_timeout,
            "Starting message loop"
        );

        loop {
            let packet_result = timeout(heartbeat_timeout, session.receive_packet()).await;

            match packet_result {
                Ok(Ok(Some(packet))) => {
                    // Update device last_seen
                    if let Ok(Some(device)) = self.device_repo.find_by_id(device_id).await {
                        let updated = device.update_last_seen();
                        let _ = self.device_repo.save(updated).await;
                    }

                    if let Err(e) = self.handle_packet(device_id, &session, packet).await {
                        tracing::error!(
                            device_id = %device_id,
                            error = %e,
                            "Error handling packet"
                        );
                    }
                }
                Ok(Ok(None)) => {
                    tracing::info!(device_id = %device_id, "Connection closed gracefully");
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
                        timeout_secs = self.config.heartbeat_timeout,
                        "Heartbeat timeout"
                    );
                    break;
                }
            }
        }

        tracing::debug!(device_id = %device_id, "Message loop ended");
        Ok(())
    }

    /// Handle a received packet
    async fn handle_packet(
        &self,
        device_id: DeviceId,
        _session: &DeviceSession,
        packet: RawPacket,
    ) -> Result<()> {
        tracing::debug!(
            device_id = %device_id,
            opcode = packet.opcode,
            size = packet.payload.len(),
            "Received packet"
        );

        // Route to packet handler
        self.packet_handler.handle(device_id, packet).await?;

        Ok(())
    }
}
