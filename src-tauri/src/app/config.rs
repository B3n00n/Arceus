use crate::app::{error::Result, models::{ServerConfig, AlakazamConfig}};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub alakazam: AlakazamConfig,
    pub apk_directory: PathBuf,
    pub database_path: PathBuf,
}

impl AppConfig {
    pub fn with_paths(apk_directory: PathBuf, database_path: PathBuf) -> Self {
        Self {
            server: ServerConfig::default(),
            alakazam: AlakazamConfig::default(),
            apk_directory,
            database_path,
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.server.tcp_port == 0 {
            return Err(crate::app::error::ArceusError::Config(
                "TCP port must be greater than 0".to_string(),
            ));
        }

        if self.server.http_port == 0 {
            return Err(crate::app::error::ArceusError::Config(
                "HTTP port must be greater than 0".to_string(),
            ));
        }

        if self.server.tcp_port == self.server.http_port {
            return Err(crate::app::error::ArceusError::Config(
                "TCP and HTTP ports must be different".to_string(),
            ));
        }

        if self.server.max_connections == 0 {
            return Err(crate::app::error::ArceusError::Config(
                "Max connections must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            alakazam: AlakazamConfig::default(),
            apk_directory: PathBuf::from("apks"),
            database_path: PathBuf::from("arceus.db"),
        }
    }
}

/// Get the system's MAC address for authentication with Alakazam
/// Returns the first valid non-loopback MAC address found
pub fn get_mac_address() -> Result<String> {
    use mac_address::get_mac_address;

    let mac = get_mac_address()
        .map_err(|e| crate::app::error::ArceusError::Config(format!("Failed to get MAC address: {}", e)))?
        .ok_or_else(|| crate::app::error::ArceusError::Config("No MAC address found".to_string()))?;

    Ok(mac.to_string())
}

pub const CLIENT_APK_FILENAME: &str = "Snorlax.apk";
pub const CLIENT_METADATA_FILENAME: &str = "client_metadata.json";

