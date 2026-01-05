use crate::app::{error::Result, models::ServerConfig};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub apk_directory: PathBuf,
    pub database_path: PathBuf,
}

impl AppConfig {
    pub fn with_paths(apk_directory: PathBuf, database_path: PathBuf) -> Self {
        Self {
            server: ServerConfig::default(),
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
            apk_directory: PathBuf::from("apks"),
            database_path: PathBuf::from("arceus.db"),
        }
    }
}

// TODO: later move to server for authorization
pub const CLIENT_APK_METADATA_URL: &str =
    "https://storage.googleapis.com/combatica_test_bucket/Snorlax.json";

pub const CLIENT_APK_DOWNLOAD_URL: &str =
    "https://storage.googleapis.com/combatica_test_bucket/Snorlax.apk";

pub const CLIENT_APK_FILENAME: &str = "Snorlax.apk";
pub const CLIENT_METADATA_FILENAME: &str = "client_metadata.json";

