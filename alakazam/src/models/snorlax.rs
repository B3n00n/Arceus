use serde::Serialize;
use sqlx::types::chrono::{DateTime, Utc};

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
