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
