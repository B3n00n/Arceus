/// Filesystem-based Client APK Repository Implementation
///
/// Manages client APK files and metadata on the filesystem.
/// Downloads APKs from remote storage (GCS) and caches them locally
/// for distribution to connecting clients.

use async_trait::async_trait;
use reqwest::Client;
use std::path::PathBuf;

use crate::app::config::{
    CLIENT_APK_DOWNLOAD_URL, CLIENT_APK_FILENAME, CLIENT_APK_METADATA_URL,
    CLIENT_METADATA_FILENAME,
};
use crate::application::dto::{ClientApkMetadata, RemoteApkMetadata};
use crate::domain::repositories::{ClientApkError, ClientApkRepository};

pub struct FsClientApkRepository {
    /// Directory where APK files and metadata are stored
    apk_directory: PathBuf,
    /// HTTP client for downloading APKs (configured with 5-minute timeout)
    http_client: Client,
}

impl FsClientApkRepository {
    pub fn new(apk_directory: PathBuf) -> Self {
        Self {
            apk_directory,
            http_client: Client::builder()
                .timeout(std::time::Duration::from_secs(300)) // 5 min timeout for large APK downloads
                .build()
                .expect("Failed to create HTTP client"),
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
        tracing::debug!("Fetching remote APK metadata from {}", CLIENT_APK_METADATA_URL);

        let response = self
            .http_client
            .get(CLIENT_APK_METADATA_URL)
            .header("Cache-Control", "no-cache, no-store, must-revalidate")
            .header("Pragma", "no-cache")
            .header("Expires", "0")
            .send()
            .await
            .map_err(|e| ClientApkError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ClientApkError::Network(format!(
                "HTTP {}: Failed to fetch metadata",
                response.status()
            )));
        }

        let metadata: RemoteApkMetadata = response
            .json()
            .await
            .map_err(|e| ClientApkError::InvalidMetadata(e.to_string()))?;

        tracing::debug!("Remote APK version: {}", metadata.version);
        Ok(metadata)
    }

    async fn download_apk(&self) -> Result<Vec<u8>, ClientApkError> {
        tracing::info!("Downloading APK from {}", CLIENT_APK_DOWNLOAD_URL);

        let response = self
            .http_client
            .get(CLIENT_APK_DOWNLOAD_URL)
            .send()
            .await
            .map_err(|e| ClientApkError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(ClientApkError::Network(format!(
                "HTTP {}: Failed to download APK",
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
