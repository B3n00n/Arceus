use crate::core::{error::ServiceError, ApkFile, Result};
use crate::network::HttpServer;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;

pub struct ApkService {
    apk_directory: PathBuf,
    http_server: Arc<HttpServer>,
}

impl ApkService {
    pub fn new(apk_directory: PathBuf, http_server: Arc<HttpServer>) -> Self {
        Self {
            apk_directory,
            http_server,
        }
    }

    pub fn get_http_server(&self) -> &Arc<HttpServer> {
        &self.http_server
    }

    pub async fn list_apks(&self) -> Result<Vec<ApkFile>> {
        let mut apks = Vec::new();
        let mut entries = fs::read_dir(&self.apk_directory).await.map_err(|e| {
            ServiceError::Apk(format!("Failed to read APK directory: {}", e))
        })?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            ServiceError::Apk(format!("Failed to read directory entry: {}", e))
        })? {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("apk") {
                let filename = entry.file_name().to_string_lossy().to_string();
                let metadata = entry.metadata().await.map_err(|e| {
                    ServiceError::Apk(format!("Failed to read file metadata: {}", e))
                })?;

                let size_bytes = metadata.len();
                let url = self.http_server.get_apk_url(&filename);

                apks.push(ApkFile::new(filename, size_bytes, url));
            }
        }

        apks.sort_by(|a, b| a.filename.cmp(&b.filename));

        Ok(apks)
    }

    pub async fn add_apk(&self, source_path: PathBuf) -> Result<()> {
        let filename = source_path
            .file_name()
            .ok_or_else(|| ServiceError::Apk("Invalid file path".to_string()))?;

        let dest_path = self.apk_directory.join(filename);

        fs::copy(&source_path, &dest_path).await.map_err(|e| {
            ServiceError::Apk(format!("Failed to copy APK file: {}", e))
        })?;

        tracing::info!("Added APK: {:?}", filename);

        Ok(())
    }

    pub async fn remove_apk(&self, filename: String) -> Result<()> {
        let path = self.apk_directory.join(&filename);

        if !path.exists() {
            return Err(ServiceError::ResourceNotFound(format!(
                "APK file not found: {}",
                filename
            ))
            .into());
        }

        fs::remove_file(&path).await.map_err(|e| {
            ServiceError::Apk(format!("Failed to remove APK file: {}", e))
        })?;

        tracing::info!("Removed APK: {}", filename);

        Ok(())
    }

    pub fn open_apk_folder(&self) -> Result<()> {
        {
            std::process::Command::new("explorer")
                .arg(&self.apk_directory)
                .spawn()
                .map_err(|e| ServiceError::Apk(format!("Failed to open folder: {}", e)))?;
        }
        Ok(())
    }
}
