use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::GameId;

/// Represents a game in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: GameId,
    pub name: String,
    pub gcs_bucket: String,
    pub gcs_base_path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Game {
    pub fn new(name: String, gcs_bucket: String, gcs_base_path: String) -> Self {
        let now = Utc::now();
        Self {
            id: GameId::new(),
            name,
            gcs_bucket,
            gcs_base_path,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get the full GCS path for a version
    pub fn version_path(&self, version: &str) -> String {
        format!("{}/{}", self.gcs_base_path, version)
    }
}
