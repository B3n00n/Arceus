use crate::{
    error::{AppError, Result},
    models::GyrosVersion,
    repositories::GyrosRepository,
    services::GcsService,
};
use std::sync::Arc;

pub struct GyrosService {
    repository: Arc<GyrosRepository>,
    gcs_service: Arc<GcsService>,
}

impl GyrosService {
    pub fn new(repository: Arc<GyrosRepository>, gcs_service: Arc<GcsService>) -> Self {
        Self {
            repository,
            gcs_service,
        }
    }

    /// Get all Gyros versions (for admin)
    pub async fn get_all_versions(&self) -> Result<Vec<GyrosVersion>> {
        self.repository.get_all_versions().await
    }

    /// Get a specific version by ID (for admin)
    pub async fn get_version_by_id(&self, id: i32) -> Result<GyrosVersion> {
        self.repository
            .get_version_by_id(id)
            .await?
            .ok_or(AppError::GyrosVersionNotFound)
    }

    /// Create a new Gyros version (for admin)
    pub async fn create_version(&self, version: &str, gcs_path: &str) -> Result<GyrosVersion> {
        self.repository.create_version(version, gcs_path).await
    }

    /// Set a version as current (for admin)
    pub async fn set_current_version(&self, id: i32) -> Result<GyrosVersion> {
        // Verify the version exists
        let version = self.get_version_by_id(id).await?;

        // Set it as current
        self.repository.set_current_version(id).await?;

        Ok(version)
    }

    /// Delete a Gyros version (for admin, only if not current)
    pub async fn delete_version(&self, id: i32) -> Result<()> {
        let version = self.get_version_by_id(id).await?;

        if version.is_current {
            return Err(AppError::BadRequest(
                "Cannot delete the current version".to_string(),
            ));
        }

        let firmware_path = format!("{}/Gyros.bin", version.gcs_path);
        self.gcs_service.delete_file(&firmware_path).await?;

        self.repository.delete_version(id).await
    }
}
