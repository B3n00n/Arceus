use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Snorlax APK version stored in database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SnorlaxVersion {
    pub id: i32,
    pub version: String,
    pub gcs_path: String,
    pub release_date: DateTime<Utc>,
    pub is_current: bool,
    pub created_at: DateTime<Utc>,
}

/// Response for Snorlax APK download
#[derive(Debug, Serialize)]
pub struct SnorlaxApkResponse {
    /// Signed download URL for the Snorlax APK
    pub download_url: String,
    /// When the signed URL expires
    pub expires_at: DateTime<Utc>,
    /// Current version of the Snorlax APK
    pub version: String,
}
