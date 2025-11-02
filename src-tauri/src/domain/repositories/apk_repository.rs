/// APK repository trait
/// Abstraction for managing APK files.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::error::RepositoryError;

pub type Result<T> = std::result::Result<T, RepositoryError>;

/// Information about an APK file
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApkInfo {
    /// File name (e.g., "MyGame.apk")
    pub filename: String,
    /// File size in bytes
    pub size_bytes: u64,
    /// Download URL for devices to fetch this APK
    pub url: String,
}

/// Repository for managing APK files
/// This trait abstracts APK file storage, allowing different implementations
/// (filesystem, S3, GCS, etc.).
#[async_trait]
pub trait ApkRepository: Send + Sync {
    /// List all available APK files
    /// Returns information about each APK file.
    async fn list_apks(&self) -> Result<Vec<ApkInfo>>;

    /// Add a new APK file from a source path
    /// Copies the APK file from `source_path` into the repository.
    /// Returns the filename of the added APK.
    async fn add_apk(&self, source_path: PathBuf) -> Result<String>;

    /// Remove an APK file by filename
    /// Returns `Ok(())` even if the file doesn't exist (idempotent).
    async fn remove_apk(&self, filename: &str) -> Result<()>;

    /// Get the directory where APKs are stored
    /// Useful for operations that need direct filesystem access.
    fn get_storage_directory(&self) -> PathBuf;
}
