use crate::application::dto::{ClientApkMetadata, RemoteApkMetadata};
use async_trait::async_trait;
use std::path::PathBuf;

/// Repository for managing client APK files and metadata
/// This trait abstracts the storage and retrieval of client APK files,
/// allowing different implementations (filesystem, cloud storage, etc.).
#[async_trait]
pub trait ClientApkRepository: Send + Sync {
    /// Fetch metadata from remote source
    /// Returns the version information of the latest available APK.
    async fn fetch_remote_metadata(&self) -> Result<RemoteApkMetadata, ClientApkError>;

    /// Download APK from remote source
    /// Returns the raw bytes of the APK file.
    async fn download_apk(&self) -> Result<Vec<u8>, ClientApkError>;

    /// Save APK to local storage
    /// Persists the APK data to disk.
    async fn save_apk(&self, data: &[u8]) -> Result<(), ClientApkError>;

    /// Get cached metadata (None if not exists)
    /// Returns the metadata of the locally cached APK, if available.
    async fn get_cached_metadata(&self) -> Result<Option<ClientApkMetadata>, ClientApkError>;

    /// Save metadata to local storage
    /// Persists metadata about the cached APK.
    async fn save_metadata(&self, metadata: &ClientApkMetadata) -> Result<(), ClientApkError>;

    /// Get the local path where APK is stored
    /// Returns the absolute path to the APK file.
    fn get_apk_path(&self) -> PathBuf;
}

/// Errors that can occur during client APK operations
#[derive(Debug, thiserror::Error)]
pub enum ClientApkError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),

    #[error("Invalid metadata: {0}")]
    InvalidMetadata(String),

    #[error("Version parse error: {0}")]
    VersionParse(#[from] semver::Error),
}
