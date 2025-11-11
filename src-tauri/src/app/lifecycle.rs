use crate::infrastructure::network::TcpServer;
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::process::Child;

pub struct AppState {
    tcp_server: Arc<TcpServer>,
    http_server: RwLock<Option<Child>>,
    battery_monitor_handle: RwLock<Option<tauri::async_runtime::JoinHandle<()>>>,
}

impl AppState {
    pub fn new(tcp_server: Arc<TcpServer>) -> Self {
        Self {
            tcp_server,
            http_server: RwLock::new(None),
            battery_monitor_handle: RwLock::new(None),
        }
    }

    pub fn set_http_server(&self, child: Child) {
        *self.http_server.write() = Some(child);
    }

    pub fn set_battery_monitor(&self, handle: tauri::async_runtime::JoinHandle<()>) {
        *self.battery_monitor_handle.write() = Some(handle);
    }

    pub fn shutdown(&self) {
        tracing::info!("Initiating graceful shutdown of all services");

        self.tcp_server.shutdown();

        if let Some(mut child) = self.http_server.write().take() {
            tracing::info!("Stopping HTTP server");
            tauri::async_runtime::spawn(async move {
                if let Err(e) = child.kill().await {
                    tracing::error!("Failed to kill HTTP server: {}", e);
                } else {
                    tracing::info!("HTTP server stopped");
                }
            });
        }

        if let Some(handle) = self.battery_monitor_handle.write().take() {
            tracing::info!("Stopping battery monitor");
            handle.abort();
            tracing::info!("Battery monitor stopped");
        }

        tracing::info!("All services shutdown complete");
    }
}
