use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Game assignment information from Alakazam server
#[derive(Debug, Clone, Deserialize)]
pub struct GameAssignment {
    pub game_id: i32,
    pub game_name: String,
    pub assigned_version: VersionInfo,
    #[serde(rename = "current_version")]
    pub _current_version: Option<VersionInfo>,
    #[serde(rename = "background_image_url")]
    pub _background_image_url: Option<String>,
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
    #[serde(rename = "game_id")]
    pub _game_id: i32,
    pub game_name: String,
    pub version: String,
    pub version_id: i32,
    #[serde(rename = "gcs_path")]
    pub _gcs_path: String,
    pub files: Vec<GameFile>,
    pub background_image_url: Option<String>,
    #[serde(rename = "expires_at")]
    pub _expires_at: DateTime<Utc>,
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

/// Cached game entry - minimal structure for offline access
/// Stores only what's needed to display games and detect updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedGameEntry {
    pub game_id: i32,
    pub game_name: String,

    // Assigned version (from Alakazam server)
    pub assigned_version_id: i32,
    pub assigned_version: String,

    // Installed version (from local filesystem)
    pub installed_version_id: Option<i32>,
    pub installed_version: Option<String>,
    pub installed_at: Option<DateTime<Utc>>,
}

impl CachedGameEntry {
    /// Create entry from Alakazam assignment and optional local metadata
    pub fn from_assignment_and_metadata(
        assignment: GameAssignment,
        local_metadata: Option<LocalGameMetadata>,
    ) -> Self {
        Self {
            game_id: assignment.game_id,
            game_name: assignment.game_name,
            assigned_version_id: assignment.assigned_version.version_id,
            assigned_version: assignment.assigned_version.version,
            installed_version_id: local_metadata.as_ref().map(|m| m.installed_version_id),
            installed_version: local_metadata.as_ref().map(|m| m.installed_version.clone()),
            installed_at: local_metadata.map(|m| m.installed_at),
        }
    }

    /// Create entry from local metadata only (when offline and no server data available)
    pub fn from_local_only(local_metadata: LocalGameMetadata) -> Self {
        Self {
            game_id: local_metadata.game_id,
            game_name: local_metadata.game_name.clone(),
            assigned_version_id: local_metadata.installed_version_id,
            assigned_version: local_metadata.installed_version.clone(),
            installed_version_id: Some(local_metadata.installed_version_id),
            installed_version: Some(local_metadata.installed_version),
            installed_at: Some(local_metadata.installed_at),
        }
    }
}
