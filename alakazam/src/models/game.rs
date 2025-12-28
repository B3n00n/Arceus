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

/// Arcade game assignment entity from database
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ArcadeGameAssignment {
    pub id: i32,
    pub arcade_id: i32,
    pub game_id: i32,
    pub assigned_version_id: i32,
    pub current_version_id: Option<i32>,
    pub updated_at: DateTime<Utc>,
}

/// Response DTO for game assignment with full details
#[derive(Debug, Serialize)]
pub struct GameAssignmentResponse {
    pub game_id: i32,
    pub game_name: String,
    pub assigned_version: VersionInfo,
    pub current_version: Option<VersionInfo>,
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

/// Request to update arcade status
#[derive(Debug, Deserialize)]
pub struct UpdateStatusRequest {
    pub current_version_id: Option<i32>,
}
