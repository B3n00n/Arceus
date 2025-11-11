use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::GameId;
use crate::domain::models::PackageName;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub id: GameId,
    pub name: String,
    pub exe_path: PathBuf,
    pub content_path: PathBuf,
    pub package_name: PackageName,
}

impl GameConfig {
    pub fn new(
        name: String,
        exe_path: PathBuf,
        content_path: PathBuf,
        package_name: PackageName,
    ) -> Self {
        Self {
            id: GameId::new(),
            name,
            exe_path,
            content_path,
            package_name,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Game name cannot be empty".to_string());
        }

        if !self.exe_path.exists() {
            return Err(format!("Game executable not found"));
        }

        if !self.exe_path.is_file() {
            return Err(format!("Game executable path is not a file"));
        }

        if !self.content_path.exists() {
            return Err(format!("Game content directory not found"));
        }

        if !self.content_path.is_dir() {
            return Err(format!("Game content path is not a directory"));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub config: GameConfig,
    pub process_id: Option<u32>,
    pub http_server_url: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl GameState {
    pub fn new(config: GameConfig, process_id: Option<u32>, http_server_url: String) -> Self {
        Self {
            config,
            process_id,
            http_server_url,
            started_at: chrono::Utc::now(),
        }
    }
}
