use crate::infrastructure::network::TcpServer;
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::process::Child;

pub struct AppState {
    tcp_server: Arc<TcpServer>,
    tcp_server_handle: RwLock<Option<tauri::async_runtime::JoinHandle<()>>>,
    http_server: RwLock<Option<Child>>,
    battery_monitor_handle: RwLock<Option<tauri::async_runtime::JoinHandle<()>>>,
}

impl AppState {
    pub fn new(tcp_server: Arc<TcpServer>) -> Self {
        Self {
            tcp_server,
            tcp_server_handle: RwLock::new(None),
            http_server: RwLock::new(None),
            battery_monitor_handle: RwLock::new(None),
        }
    }

    pub fn set_tcp_server_handle(&self, handle: tauri::async_runtime::JoinHandle<()>) {
        *self.tcp_server_handle.write() = Some(handle);
    }

    pub fn set_http_server(&self, child: Child) {
        *self.http_server.write() = Some(child);
    }

    pub fn set_battery_monitor(&self, handle: tauri::async_runtime::JoinHandle<()>) {
        *self.battery_monitor_handle.write() = Some(handle);
    }

    pub fn shutdown(&self) {
        tracing::info!("Shutting down services");

        self.tcp_server.shutdown();

        if let Some(handle) = self.battery_monitor_handle.write().take() {
            handle.abort();
        }

        if let Some(handle) = self.tcp_server_handle.write().take() {
            let _ = tauri::async_runtime::block_on(handle);
        }

        if let Some(mut child) = self.http_server.write().take() {
            let _ = tauri::async_runtime::block_on(child.kill());
        }

        tracing::info!("Shutdown complete");
    }
}
