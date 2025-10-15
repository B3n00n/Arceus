use crate::core::{error::NetworkError, EventBus, Result};
use axum::{
    Router,
    routing::get_service,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub struct HttpServer {
    port: u16,
    apk_directory: PathBuf,
    event_bus: Arc<EventBus>,
    local_ip: String,
}

impl HttpServer {
    pub fn new(port: u16, apk_directory: PathBuf, event_bus: Arc<EventBus>) -> Self {
        let local_ip = Self::discover_local_ip();

        Self {
            port,
            apk_directory,
            event_bus,
            local_ip,
        }
    }

    pub async fn start(self: Arc<Self>) -> Result<()> {
        if !self.apk_directory.exists() {
            tokio::fs::create_dir_all(&self.apk_directory)
                .await
                .map_err(|e| {
                    NetworkError::BindError(format!(
                        "Failed to create APK directory: {}",
                        e
                    ))
                })?;
        }

        let serve_dir = ServeDir::new(&self.apk_directory);
        let app = Router::new()
            .nest_service("/apks", get_service(serve_dir))
            .layer(TraceLayer::new_for_http());

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        let listener = TcpListener::bind(addr).await.map_err(|e| {
            NetworkError::BindError(format!(
                "Failed to bind HTTP server to port {}: {}",
                self.port, e
            ))
        })?;

        let url = self.get_base_url();
        tracing::info!("HTTP server listening on {}", url);

        self.event_bus.http_server_started(self.port, url);

        axum::serve(listener, app).await.map_err(|e| {
            NetworkError::ConnectionFailed(format!("HTTP server error: {}", e)).into()
        })
    }

    pub fn get_base_url(&self) -> String {
        format!("http://{}:{}/apks", self.local_ip, self.port)
    }

    pub fn get_apk_url(&self, filename: &str) -> String {
        format!("{}/{}", self.get_base_url(), filename)
    }

    pub fn get_local_ip(&self) -> &str {
        &self.local_ip
    }

    fn discover_local_ip() -> String {
        match local_ip_address::local_ip() {
            Ok(ip) => ip.to_string(),
            Err(e) => {
                tracing::warn!("Failed to discover local IP: {}, using 127.0.0.1", e);
                "127.0.0.1".to_string()
            }
        }
    }
}

