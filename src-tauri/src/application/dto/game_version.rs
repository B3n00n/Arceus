use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Game assignment information from Alakazam server
#[derive(Debug, Clone, Deserialize)]
pub struct GameAssignment {
    pub game_id: i32,
    pub game_name: String,
    pub assigned_version: VersionInfo,
    pub current_version: Option<VersionInfo>,
    pub background_image_url: Option<String>,
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

/// Cached game entry combining Alakazam assignment data with local metadata
/// Stored in Sled database for offline access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedGameEntry {
    // From Alakazam assignment
    pub game_id: i32,
    pub game_name: String,
    pub assigned_version: VersionInfo,
    pub current_version: Option<VersionInfo>,
    pub background_image_url: Option<String>,

    // From local filesystem
    pub local_metadata: Option<LocalGameMetadata>,

    // Cache metadata
    pub cached_at: DateTime<Utc>,
    pub last_synced: Option<DateTime<Utc>>,
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
            assigned_version: assignment.assigned_version,
            current_version: assignment.current_version,
            background_image_url: assignment.background_image_url,
            local_metadata,
            cached_at: Utc::now(),
            last_synced: Some(Utc::now()),
        }
    }

    /// Create entry from local metadata only (when offline and no server data available)
    pub fn from_local_only(local_metadata: LocalGameMetadata) -> Self {
        Self {
            game_id: local_metadata.game_id,
            game_name: local_metadata.game_name.clone(),
            assigned_version: VersionInfo {
                version_id: local_metadata.installed_version_id,
                version: local_metadata.installed_version.clone(),
                gcs_path: String::new(), // Unknown without online data
                release_date: local_metadata.installed_at,
            },
            current_version: None,
            background_image_url: None, // Unknown without server data
            local_metadata: Some(local_metadata),
            cached_at: Utc::now(),
            last_synced: None, // Never synced with server
        }
    }

    /// Update the local metadata portion of this entry
    pub fn update_local_metadata(&mut self, metadata: LocalGameMetadata) {
        self.local_metadata = Some(metadata);
        self.cached_at = Utc::now();
    }
}
