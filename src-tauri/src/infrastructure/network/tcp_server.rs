/// TCP Server for device connections
/// Accepts incoming TCP connections and delegates to ConnectionHandler.
/// Focuses solely on TCP transport concerns.

use crate::core::{error::NetworkError, EventBus, Result, ServerConfig};
use crate::domain::repositories::{DeviceNameRepository, DeviceRepository};
use crate::infrastructure::network::connection_handler::ConnectionHandler;
use crate::infrastructure::network::device_session_manager::DeviceSessionManager;
use crate::infrastructure::network::packet_handler::PacketHandlerRegistry;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, RwLock};

/// TCP server that accepts device connections
/// Delegates connection handling to ConnectionHandler
pub struct TcpServer {
    config: ServerConfig,
    connection_handler: Arc<ConnectionHandler>,
    device_repo: Arc<dyn DeviceRepository>,
    event_bus: Arc<EventBus>,
    running: Arc<RwLock<bool>>,
    shutdown_tx: broadcast::Sender<()>,
}

impl TcpServer {
    pub fn new(
        config: ServerConfig,
        device_repo: Arc<dyn DeviceRepository>,
        device_name_repo: Arc<dyn DeviceNameRepository>,
        event_bus: Arc<EventBus>,
    ) -> (Self, broadcast::Receiver<()>, Arc<DeviceSessionManager>) {
        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);

        // Create session manager (needed for command execution)
        let session_manager = Arc::new(DeviceSessionManager::new());

        // Create packet handler registry
        let packet_handler = Arc::new(PacketHandlerRegistry::new(
            device_repo.clone(),
            device_name_repo.clone(),
            event_bus.clone(),
            session_manager.clone(),
        ));

        // Create connection handler (business logic)
        let connection_handler = Arc::new(ConnectionHandler::new(
            device_repo.clone(),
            device_name_repo.clone(),
            event_bus.clone(),
            packet_handler,
            session_manager.clone(),
            Duration::from_secs(config.heartbeat_timeout),
        ));

        let server = Self {
            config,
            connection_handler,
            device_repo,
            event_bus: event_bus.clone(),
            running: Arc::new(RwLock::new(false)),
            shutdown_tx,
        };

        (server, shutdown_rx, session_manager)
    }

    /// Start the TCP server
    /// Listens for incoming connections and spawns handlers
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
                            // Check capacity before accepting
                            if !self.check_capacity() {
                                let current_count = self.device_repo.count().unwrap_or(0);
                                tracing::warn!(
                                    addr = %addr,
                                    current = current_count,
                                    max = self.config.max_connections,
                                    "Connection limit reached, rejecting connection"
                                );
                                drop(stream);
                                continue;
                            }

                            // Delegate to connection handler
                            let handler = Arc::clone(&self.connection_handler);
                            tokio::spawn(async move {
                                if let Err(e) = handler.handle_connection(stream, addr).await {
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

    /// Check if we can accept more connections
    fn check_capacity(&self) -> bool {
        let current_count = self.device_repo.count().unwrap_or(0);
        current_count < self.config.max_connections
    }

    /// Shutdown the server
    #[allow(dead_code)]
    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(());
    }

    /// Check if server is running
    #[allow(dead_code)]
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}
