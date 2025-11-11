use crate::core::{error::ArceusError, Result};
use std::path::PathBuf;
use tokio::process::{Child, Command};

pub struct HttpServerService;

impl HttpServerService {
    pub async fn start_server(
        port: u16,
        directory: PathBuf,
        server_name: &str,
    ) -> Result<(Child, String)> {
        tracing::info!(
            port = port,
            directory = ?directory,
            server = %server_name,
            "Starting Python HTTP server"
        );

        if !directory.exists() {
            return Err(ArceusError::Config(format!(
                "Directory not found: {:?}",
                directory
            )));
        }

        let child = Command::new("python")
            .args(["-m", "http.server", &port.to_string()])
            .current_dir(&directory)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .stdin(std::process::Stdio::null())
            .spawn()
            .map_err(|e| {
                ArceusError::Config(format!(
                    "Failed to spawn Python HTTP server: {} (Make sure Python is installed)",
                    e
                ))
            })?;

        let pid = child.id();

        let local_ip = match local_ip_address::local_ip() {
            Ok(ip) => ip.to_string(),
            Err(_) => "127.0.0.1".to_string(),
        };
        let url = format!("http://{}:{}", local_ip, port);

        tracing::info!(
            port = port,
            pid = ?pid,
            url = %url,
            server = %server_name,
            "Python HTTP server started"
        );

        Ok((child, url))
    }
}
