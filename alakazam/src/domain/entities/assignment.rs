use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{ClientId, GameId, GameVersionId};

/// Represents a game assignment to a client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientGameAssignment {
    pub client_id: ClientId,
    pub game_id: GameId,
    /// If None, client should use the latest version
    pub target_version_id: Option<GameVersionId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ClientGameAssignment {
    pub fn new(client_id: ClientId, game_id: GameId) -> Self {
        let now = Utc::now();
        Self {
            client_id,
            game_id,
            target_version_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_version(
        client_id: ClientId,
        game_id: GameId,
        version_id: GameVersionId,
    ) -> Self {
        let now = Utc::now();
        Self {
            client_id,
            game_id,
            target_version_id: Some(version_id),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_target_version(&mut self, version_id: Option<GameVersionId>) {
        self.target_version_id = version_id;
        self.updated_at = Utc::now();
    }
}
