use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

/// Release channel entity from database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReleaseChannel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Request to create a new release channel
#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    pub description: Option<String>,
}

/// Request to update a release channel (only description can be changed)
#[derive(Debug, Deserialize)]
pub struct UpdateChannelRequest {
    pub description: Option<String>,
}
