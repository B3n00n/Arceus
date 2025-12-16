use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Metadata about the locally cached client APK
///
/// This is persisted to disk as `client_metadata.json` to track
/// which version of the client APK is currently cached.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientApkMetadata {
    /// Semantic version of the cached APK (e.g., "1.2.0")
    pub version: String,
    /// Timestamp when the APK was downloaded
    pub downloaded_at: DateTime<Utc>,
    /// Source URL the APK was downloaded from
    pub source_url: String,
}

impl ClientApkMetadata {
    pub fn new(version: String, source_url: String) -> Self {
        Self {
            version,
            downloaded_at: Utc::now(),
            source_url,
        }
    }
}

/// Response from Alakazam server for Snorlax APK download
///
/// This represents the signed download URL, version, and expiration
/// received from the Alakazam central server.
#[derive(Debug, Deserialize)]
pub struct RemoteApkMetadata {
    /// Signed download URL for the APK (from GCS via Alakazam)
    pub download_url: String,
    /// Expiration time of the signed URL
    pub expires_at: DateTime<Utc>,
    /// Current version of the Snorlax APK
    pub version: String,
}
