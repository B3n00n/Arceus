use super::{ConnectionManager, DeviceConnection};
use crate::core::{error::NetworkError, EventBus, Result, ServerConfig};
use crate::handlers::HandlerRegistry;
use crate::protocol::{Message, MessageType, PacketReader};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio::time::{timeout};

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

    pub async fn stop(&self) {
        *self.running.write().await = false;
        tracing::info!("Stopping TCP server...");
    }

    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    async fn handle_connection(
        &self,
        stream: tokio::net::TcpStream,
        addr: SocketAddr,
    ) -> Result<()> {
        tracing::info!("New connection from {}", addr);

        let device = Arc::new(DeviceConnection::new(
            stream,
            addr,
            Arc::clone(&self.event_bus),
        ));

        let device_id = device.id();
        self.connection_manager.register(Arc::clone(&device))?;
        let result = self.message_loop(Arc::clone(&device)).await;
        self.connection_manager.unregister(device_id);

        result
    }

    async fn message_loop(&self, device: Arc<DeviceConnection>) -> Result<()> {
        let heartbeat_timeout = Duration::from_secs(self.config.heartbeat_timeout);
        let mut device_connected = false;

        loop {
            let message_result = timeout(heartbeat_timeout, device.receive_message()).await;

            match message_result {
                Ok(Ok(Some(message))) => {
                    device.update_last_seen();

                    if let Err(e) = self.handle_message(&device, message, &mut device_connected).await {
                        tracing::error!("Error handling message from device {}: {}", device.id(), e);
                    }
                }
                Ok(Ok(None)) => {
                    tracing::info!("Device {} closed connection", device.id());
                    break;
                }
                Ok(Err(e)) => {
                    tracing::error!("Error receiving message from device {}: {}", device.id(), e);
                    break;
                }
                Err(_) => {
                    tracing::warn!("Heartbeat timeout for device {}", device.id());
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_message(
        &self,
        device: &Arc<DeviceConnection>,
        message: Message,
        device_connected: &mut bool,
    ) -> Result<()> {
        tracing::debug!(
            "Received {} message from device {}",
            message.msg_type,
            device.id()
        );

        if message.msg_type == MessageType::DeviceConnected && !*device_connected {
            let mut reader = PacketReader::new(message.payload);
            let model = reader.read_string()?;
            let serial = reader.read_string()?;

            device.update_device_info(model, serial);
            *device_connected = true;

            self.event_bus.device_connected(device.get_state());

            return Ok(());
        }

        if let Err(e) = self
            .handler_registry
            .handle(message.msg_type, device, message.payload)
            .await
        {
            tracing::error!(
                "Handler error for {} from device {}: {}",
                message.msg_type,
                device.id(),
                e
            );
            return Err(e);
        }

        Ok(())
    }
}

