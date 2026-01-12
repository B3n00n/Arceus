use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Gyros firmware version stored in database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GyrosVersion {
    pub id: i32,
    pub version: String,
    pub gcs_path: String,
    pub release_date: DateTime<Utc>,
    pub is_current: bool,
    pub created_at: DateTime<Utc>,
}

/// Response for Gyros firmware download
#[derive(Debug, Serialize)]
pub struct GyrosResponse {
    pub download_url: String,
    pub expires_at: DateTime<Utc>,
    pub version: String,
}
