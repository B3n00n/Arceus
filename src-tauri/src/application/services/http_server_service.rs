use crate::app::{error::ArceusError, Result};
use crate::infrastructure::process::HiddenCommand;
use std::path::PathBuf;
use tokio::process::Child;

pub struct HttpServerService;

impl HttpServerService {
    pub async fn start_server(
        port: u16,
        directory: PathBuf,
        server_name: &str,
    ) -> Result<Child> {
        tracing::info!(
            port = port,
            directory = ?directory,
            server = %server_name,
            "Starting Python HTTP server"
        );

        let child = HiddenCommand::new("python")
            .args(["-m", "http.server", &port.to_string()])
            .current_dir(&directory)
            .silence_all()
            .spawn()
            .map_err(|e| ArceusError::Config(format!("Failed to spawn HTTP server: {}", e)))?;

        let pid = child.id();

        tracing::info!(
            port = port,
            pid = ?pid,
            server = %server_name,
            "Python HTTP server started"
        );

        Ok(child)
    }
}
