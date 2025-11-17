use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{GameId, GameVersionId, VersionStatus};
use crate::domain::entities::{GameManifest, GameVersion};

/// Request to create a new game version
#[derive(Debug, Clone, Deserialize)]
pub struct CreateGameVersionRequest {
    pub game_id: GameId,
    pub version: String,
    pub gcs_path: String,
    pub manifest: GameManifest,
}

/// Game version response DTO
#[derive(Debug, Clone, Serialize)]
pub struct GameVersionResponse {
    pub id: GameVersionId,
    pub game_id: GameId,
    pub version: String,
    pub gcs_path: String,
    pub is_latest: bool,
    pub status: VersionStatus,
    pub file_count: usize,
    pub total_size: u64,
    pub created_at: DateTime<Utc>,
}

impl From<GameVersion> for GameVersionResponse {
    fn from(gv: GameVersion) -> Self {
        Self {
            id: gv.id,
            game_id: gv.game_id,
            version: gv.version,
            gcs_path: gv.gcs_path,
            is_latest: gv.is_latest,
            status: gv.status,
            file_count: gv.manifest.file_count(),
            total_size: gv.manifest.total_size(),
            created_at: gv.created_at,
        }
    }
}

/// Full game version response with manifest
#[derive(Debug, Clone, Serialize)]
pub struct GameVersionWithManifestResponse {
    pub id: GameVersionId,
    pub game_id: GameId,
    pub version: String,
    pub gcs_path: String,
    pub manifest: GameManifest,
    pub is_latest: bool,
    pub status: VersionStatus,
    pub created_at: DateTime<Utc>,
}

impl From<GameVersion> for GameVersionWithManifestResponse {
    fn from(gv: GameVersion) -> Self {
        Self {
            id: gv.id,
            game_id: gv.game_id,
            version: gv.version,
            gcs_path: gv.gcs_path,
            manifest: gv.manifest,
            is_latest: gv.is_latest,
            status: gv.status,
            created_at: gv.created_at,
        }
    }
}
