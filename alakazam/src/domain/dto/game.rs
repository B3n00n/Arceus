use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::GameId;
use crate::domain::entities::Game;

/// Request to create a new game
#[derive(Debug, Clone, Deserialize)]
pub struct CreateGameRequest {
    pub name: String,
    pub gcs_bucket: String,
    pub gcs_base_path: String,
}

/// Request to update an existing game
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateGameRequest {
    pub name: Option<String>,
    pub gcs_bucket: Option<String>,
    pub gcs_base_path: Option<String>,
}

/// Game response DTO
#[derive(Debug, Clone, Serialize)]
pub struct GameResponse {
    pub id: GameId,
    pub name: String,
    pub gcs_bucket: String,
    pub gcs_base_path: String,
    pub created_at: DateTime<Utc>,
}

impl From<Game> for GameResponse {
    fn from(game: Game) -> Self {
        Self {
            id: game.id,
            name: game.name,
            gcs_bucket: game.gcs_bucket,
            gcs_base_path: game.gcs_base_path,
            created_at: game.created_at,
        }
    }
}
