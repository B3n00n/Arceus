use super::{ConnectionManager, DeviceConnection};
use crate::core::{error::NetworkError, BatteryInfo, EventBus, Result, ServerConfig, VolumeInfo};
use crate::handlers::HandlerRegistry;
use crate::protocol::ClientPacket;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio::time::timeout;

pub struct TcpServer {
    config: ServerConfig,
    connection_manager: Arc<ConnectionManager>,
    handler_registry: Arc<HandlerRegistry>,
    event_bus: Arc<EventBus>,
    running: Arc<RwLock<bool>>,
}

impl TcpServer {
    pub fn new(
        config: ServerConfig,
        connection_manager: Arc<ConnectionManager>,
        handler_registry: Arc<HandlerRegistry>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self {
            config,
            connection_manager,
            handler_registry,
            event_bus,
            running: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn start(self: Arc<Self>) -> Result<()> {
        let addr = format!("{}:{}", self.config.tcp_host, self.config.tcp_port);
        let listener = TcpListener::bind(&addr).await.map_err(|e| {
            NetworkError::BindError(format!("Failed to bind to {}: {}", addr, e))
        })?;

        tracing::info!("TCP server listening on {}", addr);

        *self.running.write().await = true;
        self.event_bus
            .server_started(self.config.tcp_port, self.config.http_port);

        while *self.running.read().await {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    if self.connection_manager.is_full() {
                        tracing::warn!(
                            "Connection limit reached, rejecting connection from {}",
                            addr
                        );
                        drop(stream);
                        continue;
                    }

                    let server = Arc::clone(&self);
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_connection(stream, addr).await {
                            tracing::error!("Error handling connection from {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    tracing::error!("Failed to accept connection: {}", e);
                    continue;
                }
            }
        }

        tracing::info!("TCP server stopped");
        self.event_bus.server_stopped();
        Ok(())
    }

    async fn handle_connection(
        &self,
        stream: tokio::net::TcpStream,
        addr: SocketAddr,
    ) -> Result<()> {
        let span = tracing::info_span!("connection", %addr);
        let _enter = span.enter();

        tracing::info!("New connection established");

        let device = Arc::new(DeviceConnection::new(
            stream,
            addr,
            Arc::clone(&self.event_bus),
        ));

        let device_id = device.id();
        self.connection_manager.register(Arc::clone(&device))?;

        tracing::debug!(
            device_id = %device_id,
            total_connections = self.connection_manager.connection_count(),
            "Device registered"
        );

        drop(_enter);
        let result = self.message_loop(Arc::clone(&device)).await;

        let _enter = span.enter();
        self.connection_manager.unregister(device_id);

        tracing::debug!(
            device_id = %device_id,
            remaining_connections = self.connection_manager.connection_count(),
            "Device unregistered"
        );

        result
    }

    async fn message_loop(&self, device: Arc<DeviceConnection>) -> Result<()> {
        let heartbeat_timeout = Duration::from_secs(self.config.heartbeat_timeout);
        let mut device_connected = false;
        let device_id = device.id();

        let span = tracing::debug_span!("message_loop", device_id = %device_id);
        let _enter = span.enter();

        tracing::debug!(
            timeout_secs = self.config.heartbeat_timeout,
            "Starting message loop"
        );

        loop {
            let packet_result = timeout(heartbeat_timeout, device.receive_packet()).await;

            match packet_result {
                Ok(Ok(Some(packet))) => {
                    device.update_last_seen();

                    if let Err(e) = self
                        .handle_packet(&device, packet, &mut device_connected)
                        .await
                    {
                        tracing::error!(error = %e, "Error handling packet");
                    }
                }
                Ok(Ok(None)) => {
                    tracing::info!("Connection closed gracefully");
                    break;
                }
                Ok(Err(e)) => {
                    tracing::error!(error = %e, "Error receiving packet");
                    break;
                }
                Err(_) => {
                    tracing::warn!(
                        timeout_secs = self.config.heartbeat_timeout,
                        "Heartbeat timeout"
                    );
                    break;
                }
            }
        }

        tracing::debug!("Message loop ended");
        Ok(())
    }

    async fn handle_packet(
        &self,
        device: &Arc<DeviceConnection>,
        packet: ClientPacket,
        device_connected: &mut bool,
    ) -> Result<()> {
        use crate::protocol::client_packet as cp;

        tracing::debug!(opcode = packet.opcode(), "Received packet");

        match packet {
            // Handle DeviceConnected first
            ClientPacket::DeviceConnected(cp::DeviceConnected { model, serial }) => {
                if !*device_connected {
                    tracing::info!(model = %model, serial = %serial, "Device connected");

                    device.update_device_info(model, serial);
                    *device_connected = true;

                    self.event_bus.device_connected(device.get_state());
                }
                Ok(())
            }

            // Heartbeat - just update last seen (already done above)
            ClientPacket::Heartbeat(_) => {
                tracing::trace!("Heartbeat received");
                Ok(())
            }

            // Battery status update
            ClientPacket::BatteryStatus(cp::BatteryStatus { level, is_charging }) => {
                let battery = BatteryInfo::new(level, is_charging);
                device.update_battery(battery);
                Ok(())
            }

            // Volume status update
            ClientPacket::VolumeStatus(cp::VolumeStatus {
                percentage,
                current,
                max,
            }) => {
                let volume = VolumeInfo::new(percentage, current, max);
                device.update_volume(volume);
                Ok(())
            }

            // Error from client
            ClientPacket::Error(cp::Error { message }) => {
                tracing::warn!(device = %device.serial(), error = %message, "Client error");
                Ok(())
            }

            // All other packets are handled by the handler registry
            _ => {
                self.handler_registry
                    .handle_packet(device, packet)
                    .await
            }
        }
    }
}
