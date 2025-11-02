use crate::domain::repositories::{ApkInfo, ApkRepository, RepositoryError};
use std::path::PathBuf;
use std::sync::Arc;

/// Result type for APK service operations
pub type Result<T> = std::result::Result<T, ApkServiceError>;

/// Errors that can occur in the APK service
#[derive(Debug, thiserror::Error)]
pub enum ApkServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Invalid file path: {0}")]
    InvalidPath(String),

    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

/// Application service for APK management
/// This service orchestrates APK related use cases.
pub struct ApkApplicationService {
    apk_repo: Arc<dyn ApkRepository>,
}

impl ApkApplicationService {
    /// Create a new ApkApplicationService
    pub fn new(apk_repo: Arc<dyn ApkRepository>) -> Self {
        Self { apk_repo }
    }

    /// List all available APK files
    pub async fn list_apks(&self) -> Result<Vec<ApkInfo>> {
        Ok(self.apk_repo.list_apks().await?)
    }

    /// Add a new APK file from a source path
    /// Copies the file into the APK repository.
    pub async fn add_apk(&self, source_path: PathBuf) -> Result<String> {
        if !source_path.exists() {
            return Err(ApkServiceError::InvalidPath(format!(
                "Source file does not exist: {}",
                source_path.display()
            )));
        }

        if !source_path.is_file() {
            return Err(ApkServiceError::InvalidPath(format!(
                "Path is not a file: {}",
                source_path.display()
            )));
        }

        if source_path.extension().and_then(|s| s.to_str()) != Some("apk") {
            return Err(ApkServiceError::InvalidPath(format!(
                "File must have .apk extension: {}",
                source_path.display()
            )));
        }

        let filename = self.apk_repo.add_apk(source_path.clone()).await?;

        tracing::info!(
            filename = %filename,
            source = %source_path.display(),
            "APK added successfully"
        );

        Ok(filename)
    }

    pub async fn remove_apk(&self, filename: &str) -> Result<()> {
        self.apk_repo.remove_apk(filename).await?;

        tracing::info!(
            filename = %filename,
            "APK removed successfully"
        );

        Ok(())
    }

    pub fn open_apk_folder(&self) -> Result<()> {
        let path = self.apk_repo.get_storage_directory();

        std::process::Command::new("explorer")
            .arg(path)
            .spawn()
            .map_err(|e| ApkServiceError::OperationFailed(format!("Failed to open folder: {}", e)))?;

        Ok(())
    }
}
