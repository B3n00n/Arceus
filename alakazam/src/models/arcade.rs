use serde::Serialize;
use sqlx::types::chrono::{DateTime, Utc};

/// Arcade entity from database
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Arcade {
    pub id: i32,
    pub name: String,
    pub machine_id: String,
    pub status: String,
    pub channel_id: i32,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Response DTO for arcade configuration
#[derive(Debug, Serialize)]
pub struct ArcadeConfigResponse {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub channel_id: i32,
}

impl From<Arcade> for ArcadeConfigResponse {
    fn from(arcade: Arcade) -> Self {
        Self {
            id: arcade.id,
            name: arcade.name,
            status: arcade.status,
            channel_id: arcade.channel_id,
        }
    }
}
