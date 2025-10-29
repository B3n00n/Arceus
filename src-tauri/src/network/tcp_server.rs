use super::{ConnectionManager, DeviceConnection, DeviceNameManager};
use crate::core::{error::NetworkError, EventBus, Result, ServerConfig};
use crate::handlers::HandlerRegistry;
use crate::storage::DeviceNamesStore;
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
    device_manager: Arc<DeviceNameManager>,
    running: Arc<RwLock<bool>>,
}

impl TcpServer {
    pub fn new(
        config: ServerConfig,
        connection_manager: Arc<ConnectionManager>,
        handler_registry: Arc<HandlerRegistry>,
        event_bus: Arc<EventBus>,
        device_names_store: Arc<DeviceNamesStore>,
    ) -> Self {
        let device_manager = Arc::new(DeviceNameManager::new(device_names_store));

        Self {
            config,
            connection_manager,
            handler_registry,
            event_bus,
            device_manager,
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
            Arc::clone(&self.device_manager),
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
        let device_id = device.id();

        let span = tracing::debug_span!("message_loop", device_id = %device_id);
        let _enter = span.enter();

        tracing::debug!(
            timeout_secs = self.config.heartbeat_timeout,
            "Starting message loop"
        );

        loop {
            let packet_result = timeout(heartbeat_timeout, device.receive_raw_packet()).await;

            match packet_result {
                Ok(Ok(Some(packet))) => {
                    device.update_last_seen();

                    if let Err(e) = self.handle_packet(&device, packet).await {
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
        packet: crate::protocol::RawPacket,
    ) -> Result<()> {
        tracing::debug!(opcode = packet.opcode, "Received packet");

        // Create a cursor from payload for reading
        let mut src = std::io::Cursor::new(packet.payload);
        // Dummy dst for handlers that might write responses (currently unused)
        let mut dst = Vec::new();

        // Dispatch to handler registry
        self.handler_registry
            .handle(device, packet.opcode, &mut src, &mut dst)
            .await
    }
}
