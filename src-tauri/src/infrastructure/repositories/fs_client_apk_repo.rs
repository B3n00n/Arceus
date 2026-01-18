/// Filesystem-based Client APK Repository Implementation
///
/// Manages client APK files and metadata on the filesystem.
/// Downloads APKs from remote storage (GCS) and caches them locally
/// for distribution to connecting clients.

use async_trait::async_trait;
use reqwest::Client;
use std::path::PathBuf;

use crate::app::config::{
    get_machine_id, CLIENT_APK_FILENAME, CLIENT_METADATA_FILENAME,
};
use crate::app::models::AlakazamConfig;
use crate::application::dto::{ClientApkMetadata, RemoteApkMetadata};
use crate::domain::repositories::{ClientApkError, ClientApkRepository};

pub struct FsClientApkRepository {
    /// Directory where APK files and metadata are stored
    apk_directory: PathBuf,
    /// HTTP client for downloading APKs (configured with 1h timeout)
    http_client: Client,
    /// Alakazam server configuration
    alakazam_config: AlakazamConfig,
}

impl FsClientApkRepository {
    pub fn new(apk_directory: PathBuf, alakazam_config: AlakazamConfig) -> Self {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(3600)) // 1h timeout for large APK downloads
            .build()
            .expect("Failed to create HTTP client - TLS initialization may have failed");

        Self {
            apk_directory,
            http_client,
            alakazam_config,
        }
    }

    /// Get the path to the metadata file
    fn metadata_path(&self) -> PathBuf {
        self.apk_directory.join(CLIENT_METADATA_FILENAME)
    }
}

#[async_trait]
impl ClientApkRepository for FsClientApkRepository {
    async fn fetch_remote_metadata(&self) -> Result<RemoteApkMetadata, ClientApkError> {
        let url = format!(
            "{}{}",
            self.alakazam_config.base_url, self.alakazam_config.snorlax_endpoint
        );
        tracing::debug!("Fetching Snorlax APK info from Alakazam: {}", url);

        // Get machine ID for authentication
        let machine_id = get_machine_id()
            .map_err(|e| ClientApkError::Network(format!("Failed to get machine ID: {}", e)))?;

        tracing::info!("Authenticating with machine ID: {}", machine_id);

        let response = self
            .http_client
            .get(&url)
            .header("X-Machine-ID", machine_id)
            .header("Cache-Control", "no-cache, no-store, must-revalidate")
            .header("Pragma", "no-cache")
            .header("Expires", "0")
            .send()
            .await
            .map_err(|e| ClientApkError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ClientApkError::Network(format!(
                "HTTP {}: Failed to fetch Snorlax APK info from Alakazam",
                response.status()
            )));
        }

        let metadata: RemoteApkMetadata = response
            .json()
            .await
            .map_err(|e| ClientApkError::InvalidMetadata(e.to_string()))?;

        tracing::debug!(
            "Snorlax APK version from Alakazam: {}, expires at: {}",
            metadata.version,
            metadata.expires_at
        );
        Ok(metadata)
    }

    async fn download_apk(&self, download_url: &str) -> Result<Vec<u8>, ClientApkError> {
        tracing::info!("Downloading APK from signed URL");

        let response = self
            .http_client
            .get(download_url)
            .send()
            .await
            .map_err(|e| ClientApkError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ClientApkError::Network(format!(
                "HTTP {}: Failed to download APK from signed URL",
                response.status()
            )));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| ClientApkError::Network(e.to_string()))?
            .to_vec();

        tracing::info!("Downloaded APK: {} bytes", bytes.len());
        Ok(bytes)
    }

    async fn save_apk(&self, data: &[u8]) -> Result<(), ClientApkError> {
        let path = self.get_apk_path();
        tracing::debug!("Saving APK to {}", path.display());

        tokio::fs::write(&path, data).await?;
        tracing::info!("APK saved successfully");
        Ok(())
    }

    async fn get_cached_metadata(&self) -> Result<Option<ClientApkMetadata>, ClientApkError> {
        let path = self.metadata_path();

        if !path.exists() {
            return Ok(None);
        }

        let contents = tokio::fs::read_to_string(&path).await?;
        let metadata: ClientApkMetadata = serde_json::from_str(&contents)
            .map_err(|e| ClientApkError::InvalidMetadata(e.to_string()))?;

        Ok(Some(metadata))
    }

    async fn save_metadata(&self, metadata: &ClientApkMetadata) -> Result<(), ClientApkError> {
        let path = self.metadata_path();
        let json = serde_json::to_string_pretty(metadata)
            .map_err(|e| ClientApkError::InvalidMetadata(e.to_string()))?;

        tokio::fs::write(&path, json).await?;
        tracing::debug!("Metadata saved to {}", path.display());
        Ok(())
    }

    fn get_apk_path(&self) -> PathBuf {
        self.apk_directory.join(CLIENT_APK_FILENAME)
    }
}
