use crate::{
    error::{AppError, Result},
    models::{SnorlaxApkResponse, SnorlaxVersion},
    repositories::SnorlaxRepository,
    services::GcsService,
};
use chrono::Utc;
use std::sync::Arc;

pub struct SnorlaxService {
    repository: Arc<SnorlaxRepository>,
    gcs_service: Arc<GcsService>,
}

impl SnorlaxService {
    pub fn new(repository: Arc<SnorlaxRepository>, gcs_service: Arc<GcsService>) -> Self {
        Self {
            repository,
            gcs_service,
        }
    }

    /// Get the current Snorlax version that arcades should download
    pub async fn get_current_version(&self) -> Result<SnorlaxVersion> {
        self.repository
            .get_current_version()
            .await?
            .ok_or(AppError::NoCurrentSnorlaxVersion)
    }

    /// Get download URL for the current Snorlax APK
    pub async fn get_latest_apk_response(&self) -> Result<SnorlaxApkResponse> {
        let current_version = self.get_current_version().await?;

        // Construct full GCS path: {folder}/Snorlax.apk
        let full_gcs_path = format!("{}/Snorlax.apk", current_version.gcs_path);

        // Generate signed download URL
        let download_url = self
            .gcs_service
            .generate_signed_download_url(&full_gcs_path)
            .await?;

        // Calculate expiration time
        let duration_secs = self.gcs_service.get_url_duration_secs();
        let expires_at = Utc::now() + chrono::Duration::seconds(duration_secs as i64);

        Ok(SnorlaxApkResponse {
            download_url,
            expires_at,
            version: current_version.version,
        })
    }

    /// Get all Snorlax versions (for admin)
    pub async fn get_all_versions(&self) -> Result<Vec<SnorlaxVersion>> {
        self.repository.get_all_versions().await
    }

    /// Get a specific version by ID (for admin)
    pub async fn get_version_by_id(&self, id: i32) -> Result<SnorlaxVersion> {
        self.repository
            .get_version_by_id(id)
            .await?
            .ok_or(AppError::SnorlaxVersionNotFound)
    }

    /// Create a new Snorlax version (for admin)
    pub async fn create_version(&self, version: &str, gcs_path: &str) -> Result<SnorlaxVersion> {
        self.repository.create_version(version, gcs_path).await
    }

    /// Set a version as current (for admin)
    pub async fn set_current_version(&self, id: i32) -> Result<SnorlaxVersion> {
        // Verify the version exists
        let version = self.get_version_by_id(id).await?;

        // Set it as current
        self.repository.set_current_version(id).await?;

        Ok(version)
    }

    /// Delete a Snorlax version (for admin, only if not current)
    pub async fn delete_version(&self, id: i32) -> Result<()> {
        let version = self.get_version_by_id(id).await?;

        if version.is_current {
            return Err(AppError::BadRequest(
                "Cannot delete the current version".to_string()
            ));
        }

        let apk_path = format!("{}/Snorlax.apk", version.gcs_path);
        self.gcs_service.delete_file(&apk_path).await?;

        self.repository.delete_version(id).await
    }
}
