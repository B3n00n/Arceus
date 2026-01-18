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
}

impl ClientApkMetadata {
    pub fn new(version: String) -> Self {
        Self { version }
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
