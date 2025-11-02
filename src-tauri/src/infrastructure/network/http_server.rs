use crate::core::{error::NetworkError, EventBus, Result};
use axum::{
    Router,
    routing::{get, get_service},
    response::{Html, IntoResponse},
    extract::State as AxumState,
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
            .route("/", get(directory_listing))
            .fallback_service(get_service(serve_dir))
            .layer(TraceLayer::new_for_http())
            .with_state(Arc::clone(&self));

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        let listener = TcpListener::bind(addr).await.map_err(|e| {
            NetworkError::BindError(format!(
                "Failed to bind HTTP server to port {}: {}",
                self.port, e
            ))
        })?;

        let url = self.get_base_url();
        tracing::info!("HTTP server listening on {}", url);
        tracing::info!("APK directory: {:?}", self.apk_directory);

        self.event_bus.http_server_started(self.port, url.clone());

        axum::serve(listener, app).await.map_err(|e| {
            NetworkError::ConnectionFailed(format!("HTTP server error: {}", e)).into()
        })
    }

    pub fn get_base_url(&self) -> String {
        format!("http://{}:{}", self.local_ip, self.port)
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

async fn directory_listing(AxumState(server): AxumState<Arc<HttpServer>>) -> impl IntoResponse {
    let mut files = Vec::new();

    match tokio::fs::read_dir(&server.apk_directory).await {
        Ok(mut entries) => {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(metadata) = entry.metadata().await {
                    if metadata.is_file() {
                        if let Some(filename) = entry.file_name().to_str() {
                            if filename.ends_with(".apk") {
                                let size = format_size(metadata.len());
                                files.push((filename.to_string(), size));
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to read APK directory: {}", e);
        }
    }

    files.sort_by(|a, b| a.0.cmp(&b.0));

    let mut html = String::from("<html><head><title>Index of /</title></head><body><h1>Index of /</h1><hr><pre>\n");

    if files.is_empty() {
        html.push_str("No APK files found\n");
    } else {
        for (filename, size) in files {
            html.push_str(&format!("<a href=\"/{}\">{}</a>  {}\n", filename, filename, size));
        }
    }

    html.push_str("</pre><hr></body></html>");

    Html(html)
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

