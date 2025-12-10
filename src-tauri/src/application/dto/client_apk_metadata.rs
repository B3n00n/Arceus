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

/// Metadata fetched from remote storage (e.g., GCS)
///
/// This represents the version information of the latest
/// available client APK stored in cloud storage.
#[derive(Debug, Deserialize)]
pub struct RemoteApkMetadata {
    /// Semantic version of the latest available 
    pub version: String,
}
