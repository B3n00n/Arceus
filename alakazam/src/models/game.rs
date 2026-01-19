use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

/// Game entity from database
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

/// Game version entity from database
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct GameVersion {
    pub id: i32,
    pub game_id: i32,
    pub version: String,
    pub gcs_path: String,
    pub release_date: DateTime<Utc>,
}

/// Game version with channel information
#[derive(Debug, Serialize)]
pub struct GameVersionWithChannels {
    pub id: i32,
    pub game_id: i32,
    pub version: String,
    pub gcs_path: String,
    pub release_date: DateTime<Utc>,
    pub channels: Vec<ChannelInfo>,
}

/// Channel information for a version
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ChannelInfo {
    pub id: i32,
    pub name: String,
}

/// Response DTO for game assignment with full details
#[derive(Debug, Serialize)]
pub struct GameAssignmentResponse {
    pub game_id: i32,
    pub game_name: String,
    pub assigned_version: VersionInfo,
    pub background_image_url: Option<String>,
}

/// Version information in response
#[derive(Debug, Serialize)]
pub struct VersionInfo {
    pub version_id: i32,
    pub version: String,
    pub gcs_path: String,
    pub release_date: DateTime<Utc>,
}

impl From<GameVersion> for VersionInfo {
    fn from(gv: GameVersion) -> Self {
        Self {
            version_id: gv.id,
            version: gv.version,
            gcs_path: gv.gcs_path,
            release_date: gv.release_date,
        }
    }
}

/// Request to publish/unpublish version to channels
#[derive(Debug, Deserialize)]
pub struct PublishVersionRequest {
    pub channel_ids: Vec<i32>,
}

/// Request to update arcade's release channel
#[derive(Debug, Deserialize)]
pub struct UpdateArcadeChannelRequest {
    pub channel_id: i32,
}
