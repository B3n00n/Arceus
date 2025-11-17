use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{GameId, GameVersionId, VersionStatus};
use super::manifest::GameManifest;

/// Represents a specific version of a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameVersion {
    pub id: GameVersionId,
    pub game_id: GameId,
    pub version: String,
    pub gcs_path: String,
    pub manifest: GameManifest,
    pub is_latest: bool,
    pub status: VersionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl GameVersion {
    pub fn new(
        game_id: GameId,
        version: String,
        gcs_path: String,
        manifest: GameManifest,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: GameVersionId::new(),
            game_id,
            version,
            gcs_path,
            manifest,
            is_latest: false,
            status: VersionStatus::Draft,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn publish(&mut self) {
        self.status = VersionStatus::Published;
        self.updated_at = Utc::now();
    }

    pub fn deprecate(&mut self) {
        self.status = VersionStatus::Deprecated;
        self.updated_at = Utc::now();
    }

    pub fn set_as_latest(&mut self) {
        self.is_latest = true;
        self.updated_at = Utc::now();
    }
}
