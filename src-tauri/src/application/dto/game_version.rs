use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Game assignment information from Alakazam server
#[derive(Debug, Clone, Deserialize)]
pub struct GameAssignment {
    pub game_id: i32,
    pub game_name: String,
    pub assigned_version: VersionInfo,
    pub current_version: Option<VersionInfo>,
}

/// Version information for a game
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionInfo {
    pub version_id: i32,
    pub version: String,
    pub gcs_path: String,
    pub release_date: DateTime<Utc>,
}

/// Response from Alakazam server for game download
#[derive(Debug, Deserialize)]
pub struct GameDownloadResponse {
    pub game_id: i32,
    pub game_name: String,
    pub version: String,
    pub version_id: i32,
    pub gcs_path: String,
    pub files: Vec<GameFile>,
    pub expires_at: DateTime<Utc>,
}

/// Individual file to download from GCS
#[derive(Debug, Deserialize)]
pub struct GameFile {
    /// Relative path within the game directory
    pub path: String,
    /// Signed download URL
    pub download_url: String,
}

/// Local metadata about installed game versions
/// Stored in C:/Combatica/<game_name>/game_metadata.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalGameMetadata {
    pub game_id: i32,
    pub game_name: String,
    pub installed_version: String,
    pub installed_version_id: i32,
    pub installed_at: DateTime<Utc>,
}

impl LocalGameMetadata {
    pub fn new(game_id: i32, game_name: String, version: String, version_id: i32) -> Self {
        Self {
            game_id,
            game_name,
            installed_version: version,
            installed_version_id: version_id,
            installed_at: Utc::now(),
        }
    }
}
