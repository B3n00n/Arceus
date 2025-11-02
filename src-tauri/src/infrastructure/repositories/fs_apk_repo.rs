/// Filesystem-based APK Repository Implementation
///
/// Stores APK files in a directory and provides access via HTTP URLs.

use crate::domain::repositories::{ApkInfo, ApkRepository, RepositoryError};
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

/// Filesystem APK repository
///
/// Stores APK files in a directory on disk.
pub struct FsApkRepository {
    storage_dir: PathBuf,
    base_url: String,
}

impl FsApkRepository {
    /// Create a new FsApkRepository
    ///
    /// # Arguments
    /// * `storage_dir` - Directory where APK files are stored
    /// * `base_url` - Base URL for serving APK files (e.g., "http://localhost:8080")
    pub fn new<P: Into<PathBuf>>(storage_dir: P, base_url: String) -> Self {
        Self {
            storage_dir: storage_dir.into(),
            base_url,
        }
    }

    /// Get the full URL for an APK file
    fn get_apk_url(&self, filename: &str) -> String {
        format!("{}/{}", self.base_url, filename)
    }

    /// Get the full path for an APK file
    fn get_apk_path(&self, filename: &str) -> PathBuf {
        self.storage_dir.join(filename)
    }
}

#[async_trait]
impl ApkRepository for FsApkRepository {
    async fn list_apks(&self) -> Result<Vec<ApkInfo>, RepositoryError> {
        let mut apks = Vec::new();

        let mut entries = fs::read_dir(&self.storage_dir)
            .await
            .map_err(|e| RepositoryError::IoError(format!("Failed to read APK directory: {}", e)))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| RepositoryError::IoError(format!("Failed to read directory entry: {}", e)))?
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("apk") {
                let filename = entry
                    .file_name()
                    .to_string_lossy()
                    .to_string();

                let metadata = entry
                    .metadata()
                    .await
                    .map_err(|e| RepositoryError::IoError(format!("Failed to read file metadata: {}", e)))?;

                let size_bytes = metadata.len();
                let url = self.get_apk_url(&filename);

                apks.push(ApkInfo {
                    filename,
                    size_bytes,
                    url,
                });
            }
        }

        apks.sort_by(|a, b| a.filename.cmp(&b.filename));

        Ok(apks)
    }

    async fn add_apk(&self, source_path: PathBuf) -> Result<String, RepositoryError> {
        let filename = source_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| RepositoryError::IoError("Invalid source path".to_string()))?;

        let dest_path = self.get_apk_path(filename);

        fs::copy(&source_path, &dest_path)
            .await
            .map_err(|e| RepositoryError::IoError(format!("Failed to copy APK file: {}", e)))?;

        tracing::info!("Added APK: {}", filename);

        Ok(filename.to_string())
    }

    async fn remove_apk(&self, filename: &str) -> Result<(), RepositoryError> {
        let path = self.get_apk_path(filename);

        if !path.exists() {
            // Idempotent - return Ok if file doesn't exist
            return Ok(());
        }

        fs::remove_file(&path)
            .await
            .map_err(|e| RepositoryError::IoError(format!("Failed to remove APK file: {}", e)))?;

        tracing::info!("Removed APK: {}", filename);

        Ok(())
    }

    fn get_storage_directory(&self) -> PathBuf {
        self.storage_dir.clone()
    }
}
