use crate::application::services::{BatteryMonitor, HttpServerService};
use crate::app::{AppConfig, AppState, EventBus};
use crate::infrastructure::network::TcpServer;
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ServerState {
    NotStarted,
    Running,
}

pub struct ServerManager {
    state: RwLock<ServerState>,
    tcp_server: Arc<TcpServer>,
    config: AppConfig,
    event_bus: Arc<EventBus>,
    battery_monitor: Arc<BatteryMonitor>,
}

impl ServerManager {
    pub fn new(
        tcp_server: Arc<TcpServer>,
        config: AppConfig,
        event_bus: Arc<EventBus>,
        battery_monitor: Arc<BatteryMonitor>,
    ) -> Self {
        Self {
            state: RwLock::new(ServerState::NotStarted),
            tcp_server,
            config,
            event_bus,
            battery_monitor,
        }
    }

    pub fn start(&self, app_state: &Arc<AppState>) {
        let mut state = self.state.write();

        if *state == ServerState::Running {
            tracing::debug!("Servers already running, ignoring start request");
            return;
        }

        tracing::info!("Starting background servers (TCP: 43572, HTTP: 43573)...");

        let tcp_server = self.tcp_server.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(e) = tcp_server.start().await {
                tracing::error!("TCP server error: {}", e);
            }
        });

        let apk_port = self.config.server.http_port;
        let apk_dir = self.config.apk_directory.clone();
        let event_bus = self.event_bus.clone();
        let app_state_clone = app_state.clone();

        tauri::async_runtime::spawn(async move {
            match HttpServerService::start_server(apk_port, apk_dir, "APK Server").await {
                Ok(child) => {
                    let url = format!("http://127.0.0.1:{}", apk_port);
                    event_bus.http_server_started(apk_port, url);

                    app_state_clone.set_http_server(child);
                }
                Err(e) => {
                    tracing::error!("Failed to start APK HTTP server: {}", e);
                }
            }
        });

        let battery_monitor = self.battery_monitor.clone();
        let app_state_for_monitor = app_state.clone();

        let handle = tauri::async_runtime::spawn(async move {
            battery_monitor.start().await;
        });

        app_state_for_monitor.set_battery_monitor(handle);

        *state = ServerState::Running;
        tracing::info!("All background servers started");
    }
}
