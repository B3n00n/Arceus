use semver::Version;
use std::sync::Arc;

use crate::app::config::{CLIENT_APK_DOWNLOAD_URL, CLIENT_APK_FILENAME};
use crate::application::dto::ClientApkMetadata;
use crate::domain::repositories::{ClientApkError, ClientApkRepository};

/// Service for managing client APK updates
///
/// Handles checking for updates, downloading APKs from remote storage,
/// and determining whether connected clients need to be updated.
/// Uses semantic versioning to compare versions.
pub struct ClientApkService {
    /// Repository for APK storage operations
    repository: Arc<dyn ClientApkRepository>,
    /// Server's IP address or hostname (for generating download URLs)
    server_host: String,
    /// HTTP server port (for generating download URLs)
    http_port: u16,
}

impl ClientApkService {
    pub fn new(repository: Arc<dyn ClientApkRepository>, server_host: String, http_port: u16) -> Self {
        Self {
            repository,
            server_host,
            http_port,
        }
    }

    pub async fn check_and_download_if_needed(&self) -> Result<bool, ClientApkError> {
        tracing::info!("Checking for client APK updates...");

        // Fetch remote version
        let remote_metadata = self.repository.fetch_remote_metadata().await?;
        let remote_version = Version::parse(&remote_metadata.version)?;

        // Get cached version
        let cached_metadata = self.repository.get_cached_metadata().await?;
        let cached_version = match cached_metadata {
            Some(ref metadata) => Version::parse(&metadata.version)?,
            None => {
                tracing::info!("No cached APK found");
                Version::parse("0.0.0").unwrap()
            }
        };

        tracing::info!(
            "Version check: remote={}, cached={}",
            remote_version,
            cached_version
        );

        if remote_version > cached_version {
            tracing::info!(
                "New client version available: {} -> {}",
                cached_version,
                remote_version
            );

            // Download APK
            let apk_data = self.repository.download_apk().await?;

            // Save to disk
            self.repository.save_apk(&apk_data).await?;

            // Update metadata
            let new_metadata =
                ClientApkMetadata::new(remote_metadata.version, CLIENT_APK_DOWNLOAD_URL.to_string());
            self.repository.save_metadata(&new_metadata).await?;

            tracing::info!(
                "Client APK updated successfully to version {}",
                remote_version
            );
            Ok(true)
        } else {
            tracing::info!(
                "Client APK is up to date (version {})",
                cached_version
            );
            Ok(false)
        }
    }

    pub async fn get_cached_version(&self) -> Option<String> {
        match self.repository.get_cached_metadata().await {
            Ok(Some(metadata)) => Some(metadata.version),
            _ => None,
        }
    }

    pub async fn should_update_client(&self, client_version: &str) -> bool {
        let cached_version = match self.get_cached_version().await {
            Some(v) => v,
            None => {
                tracing::warn!("No cached APK available for client updates");
                return false;
            }
        };

        match (
            Version::parse(&cached_version),
            Version::parse(client_version),
        ) {
            (Ok(cached), Ok(client)) => {
                let should_update = cached > client;
                if should_update {
                    tracing::info!(
                        "Client version {} is outdated (latest: {})",
                        client,
                        cached
                    );
                }
                should_update
            }
            (Err(e), _) | (_, Err(e)) => {
                tracing::error!("Version parse error: {}", e);
                false
            }
        }
    }

    pub fn get_download_url(&self) -> String {
        format!(
            "http://{}:{}/{}",
            self.server_host, self.http_port, CLIENT_APK_FILENAME
        )
    }
}
