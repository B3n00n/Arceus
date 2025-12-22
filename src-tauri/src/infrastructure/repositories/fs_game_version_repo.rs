/// Filesystem-based Game Version Repository Implementation
///
/// Manages game installations and version tracking on the filesystem.
/// Downloads games from GCS via Alakazam signed URLs with smart updates (only changed files).

use async_trait::async_trait;
use reqwest::Client;
use std::collections::HashSet;
use std::path::PathBuf;
use tokio::fs;

use crate::app::config::{get_mac_address};
use crate::app::models::AlakazamConfig;
use crate::application::dto::{GameAssignment, GameDownloadResponse, GameFile, LocalGameMetadata};
use crate::domain::repositories::{GameVersionError, GameVersionRepository};

const GAME_METADATA_FILENAME: &str = "game_metadata.json";

pub struct FsGameVersionRepository {
    /// Base directory for game installations (e.g., C:/Combatica)
    games_directory: PathBuf,
    /// HTTP client for downloading game files (configured with long timeout)
    http_client: Client,
    /// Alakazam server configuration
    alakazam_config: AlakazamConfig,
}

impl FsGameVersionRepository {
    pub fn new(games_directory: PathBuf, alakazam_config: AlakazamConfig) -> Self {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(3600))
            .build()
            .expect("Failed to create HTTP client - TLS initialization may have failed");

        Self {
            games_directory,
            http_client,
            alakazam_config,
        }
    }

    fn metadata_path(&self, game_name: &str) -> PathBuf {
        self.games_directory
            .join(game_name)
            .join(GAME_METADATA_FILENAME)
    }

    /// Recursively collect all files in a directory (excluding metadata file)
    async fn collect_local_files(&self, dir: &PathBuf) -> Result<HashSet<String>, GameVersionError> {
        let mut files = HashSet::new();
        let mut stack = vec![dir.clone()];

        while let Some(current_dir) = stack.pop() {
            let mut entries = fs::read_dir(&current_dir).await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();

                // Skip metadata file
                if path.file_name().and_then(|n| n.to_str()) == Some(GAME_METADATA_FILENAME) {
                    continue;
                }

                if path.is_dir() {
                    stack.push(path);
                } else if path.is_file() {
                    // Get relative path from game directory
                    if let Ok(rel_path) = path.strip_prefix(dir) {
                        if let Some(path_str) = rel_path.to_str() {
                            files.insert(path_str.replace('\\', "/"));
                        }
                    }
                }
            }
        }

        Ok(files)
    }
}

#[async_trait]
impl GameVersionRepository for FsGameVersionRepository {
    async fn fetch_game_assignments(&self) -> Result<Vec<GameAssignment>, GameVersionError> {
        let url = format!("{}/api/arcade/games", self.alakazam_config.base_url);

        // Get MAC address for authentication
        let mac_address = get_mac_address().map_err(|e| {
            GameVersionError::Network(format!("Failed to get MAC address: {}", e))
        })?;

        let response = self
            .http_client
            .get(&url)
            .header("X-MAC-Address", mac_address)
            .send()
            .await
            .map_err(|e| GameVersionError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(GameVersionError::Network(format!(
                "HTTP {}: Failed to fetch game assignments from Alakazam",
                response.status()
            )));
        }

        let assignments: Vec<GameAssignment> = response
            .json()
            .await
            .map_err(|e| GameVersionError::InvalidMetadata(e.to_string()))?;

        tracing::debug!("Fetched {} game assignments", assignments.len());
        Ok(assignments)
    }

    async fn fetch_download_urls(
        &self,
        game_id: i32,
    ) -> Result<GameDownloadResponse, GameVersionError> {
        let url = format!(
            "{}/api/arcade/games/{}/download",
            self.alakazam_config.base_url, game_id
        );
        tracing::info!("Fetching download URLs for game {}", game_id);

        // Get MAC address for authentication
        let mac_address = get_mac_address().map_err(|e| {
            GameVersionError::Network(format!("Failed to get MAC address: {}", e))
        })?;

        let response = self
            .http_client
            .get(&url)
            .header("X-MAC-Address", mac_address)
            .send()
            .await
            .map_err(|e| GameVersionError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(GameVersionError::Network(format!(
                "HTTP {}: Failed to fetch download URLs",
                response.status()
            )));
        }

        let download_response: GameDownloadResponse = response
            .json()
            .await
            .map_err(|e| GameVersionError::InvalidMetadata(e.to_string()))?;

        tracing::info!(
            "Fetched {} files for {} v{}",
            download_response.files.len(),
            download_response.game_name,
            download_response.version
        );
        Ok(download_response)
    }

    async fn download_game_files(
        &self,
        game_name: &str,
        files: &[GameFile],
        progress_callback: Box<dyn Fn(usize, usize, String) + Send + Sync>,
    ) -> Result<(), GameVersionError> {
        let game_dir = self.get_game_directory(game_name);

        // Create game directory if it doesn't exist
        fs::create_dir_all(&game_dir).await?;

        // Get list of currently installed files
        let local_files = if game_dir.exists() {
            self.collect_local_files(&game_dir).await?
        } else {
            HashSet::new()
        };

        // Build set of new version files
        let new_files: HashSet<String> = files.iter().map(|f| f.path.clone()).collect();

        // Find files to delete (exist locally but not in new version)
        let files_to_delete: Vec<_> = local_files.difference(&new_files).collect();
        let files_to_delete_count = files_to_delete.len();

        if files_to_delete_count > 0 {
            tracing::info!("Removing {} obsolete files", files_to_delete_count);
            for file_path in files_to_delete {
                let full_path = game_dir.join(file_path);
                if let Err(e) = fs::remove_file(&full_path).await {
                    tracing::warn!("Failed to remove obsolete file {}: {}", file_path, e);
                } else {
                    tracing::debug!("Removed obsolete file: {}", file_path);
                }
            }
        }

        // Download new or changed files
        let mut downloaded = 0;
        let mut skipped = 0;
        let total_files = files.len();

        for (index, file) in files.iter().enumerate() {
            let file_path = game_dir.join(&file.path);
            let should_download = if !file_path.exists() {
                // File doesn't exist, must download
                true
            } else {
                // File exists - in future we could check size/hash here
                // For now, we'll skip existing files to save bandwidth
                // You can enhance this by comparing file sizes from server
                false
            };

            if should_download {
                tracing::info!(
                    "Downloading file {}/{}: {}",
                    index + 1,
                    files.len(),
                    file.path
                );

                // Update progress before downloading
                progress_callback(index, total_files, file.path.clone());

                // Download file
                let response = self
                    .http_client
                    .get(&file.download_url)
                    .send()
                    .await
                    .map_err(|e| GameVersionError::DownloadFailed {
                        file: file.path.clone(),
                        error: e.to_string(),
                    })?;

                if !response.status().is_success() {
                    return Err(GameVersionError::DownloadFailed {
                        file: file.path.clone(),
                        error: format!("HTTP {}", response.status()),
                    });
                }

                let bytes = response
                    .bytes()
                    .await
                    .map_err(|e| GameVersionError::DownloadFailed {
                        file: file.path.clone(),
                        error: e.to_string(),
                    })?;

                // Create parent directories if needed
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent).await?;
                }

                // Write file
                fs::write(&file_path, &bytes).await.map_err(|e| {
                    GameVersionError::DownloadFailed {
                        file: file.path.clone(),
                        error: e.to_string(),
                    }
                })?;

                tracing::debug!("Saved file: {}", file_path.display());
                downloaded += 1;

                // Update progress after downloading
                progress_callback(index + 1, total_files, file.path.clone());
            } else {
                tracing::debug!("Skipping existing file: {}", file.path);
                skipped += 1;
                // Update progress for skipped files too
                progress_callback(index + 1, total_files, format!("Skipped: {}", file.path));
            }
        }

        tracing::info!(
            "Update complete: {} files downloaded, {} files skipped, {} files removed",
            downloaded,
            skipped,
            files_to_delete_count
        );
        Ok(())
    }

    async fn get_local_metadata(
        &self,
        game_name: &str,
    ) -> Result<Option<LocalGameMetadata>, GameVersionError> {
        let metadata_path = self.metadata_path(game_name);

        if !metadata_path.exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&metadata_path).await?;
        let metadata: LocalGameMetadata = serde_json::from_str(&contents)
            .map_err(|e| GameVersionError::InvalidMetadata(e.to_string()))?;

        Ok(Some(metadata))
    }

    async fn save_local_metadata(
        &self,
        game_name: &str,
        metadata: &LocalGameMetadata,
    ) -> Result<(), GameVersionError> {
        let metadata_path = self.metadata_path(game_name);
        let json = serde_json::to_string_pretty(metadata)
            .map_err(|e| GameVersionError::InvalidMetadata(e.to_string()))?;

        // Ensure game directory exists
        if let Some(parent) = metadata_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(&metadata_path, json).await?;
        tracing::info!("Saved metadata to {}", metadata_path.display());
        Ok(())
    }

    async fn report_version_status(
        &self,
        game_id: i32,
        version_id: Option<i32>,
    ) -> Result<(), GameVersionError> {
        let url = format!(
            "{}/api/arcade/games/{}/status",
            self.alakazam_config.base_url, game_id
        );
        tracing::info!("Reporting version status for game {}", game_id);

        // Get MAC address for authentication
        let mac_address = get_mac_address().map_err(|e| {
            GameVersionError::Network(format!("Failed to get MAC address: {}", e))
        })?;

        let response = self
            .http_client
            .post(&url)
            .header("X-MAC-Address", mac_address)
            .json(&serde_json::json!({
                "current_version_id": version_id
            }))
            .send()
            .await
            .map_err(|e| GameVersionError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(GameVersionError::Network(format!(
                "HTTP {}: Failed to report version status",
                response.status()
            )));
        }

        tracing::info!("Successfully reported version status");
        Ok(())
    }

    fn get_game_directory(&self, game_name: &str) -> PathBuf {
        self.games_directory.join(game_name)
    }

    async fn scan_installed_games(&self) -> Result<Vec<LocalGameMetadata>, GameVersionError> {
        let mut discovered_games = Vec::new();

        if !self.games_directory.exists() {
            tracing::warn!("Games directory does not exist: {:?}", self.games_directory);
            return Ok(discovered_games);
        }

        let mut entries = fs::read_dir(&self.games_directory).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let game_name = match path.file_name().and_then(|n| n.to_str()) {
                Some(name) => name.to_string(),
                None => continue,
            };

            let metadata_path = path.join(GAME_METADATA_FILENAME);

            if metadata_path.exists() {
                match fs::read_to_string(&metadata_path).await {
                    Ok(contents) => {
                        match serde_json::from_str::<LocalGameMetadata>(&contents) {
                            Ok(metadata) => {
                                tracing::info!(
                                    "Discovered installed game: {} v{}",
                                    metadata.game_name,
                                    metadata.installed_version
                                );
                                discovered_games.push(metadata);
                            }
                            Err(e) => {
                                tracing::warn!("Invalid metadata in {:?}: {}", metadata_path, e);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to read metadata from {:?}: {}", metadata_path, e);
                    }
                }
            } else {
                tracing::debug!("No metadata found in directory: {}", game_name);
            }
        }

        tracing::info!(
            "Filesystem scan complete: found {} game(s)",
            discovered_games.len()
        );
        Ok(discovered_games)
    }
}
