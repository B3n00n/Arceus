use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::app::EventBus;
use crate::application::dto::{CachedGameEntry, LocalGameMetadata};
use crate::domain::repositories::{GameVersionError, GameVersionRepository};
use crate::infrastructure::repositories::SledGameCacheRepository;

/// Game status information for the dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStatus {
    pub game_id: i32,
    pub game_name: String,
    pub installed_version: Option<String>,
    pub assigned_version: String,
    pub assigned_version_id: i32,
    pub update_available: bool,
    pub download_progress: Option<DownloadProgress>,
    pub online: bool,
    pub last_synced: Option<DateTime<Utc>>,
    pub background_image_path: Option<String>,
}

/// Download progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub total_files: usize,
    pub downloaded_files: usize,
    pub current_file: String,
    pub percentage: f32,
}

/// Service for managing game versions
pub struct GameVersionService {
    repository: Arc<dyn GameVersionRepository>,
    cache_repository: Arc<SledGameCacheRepository>,
    event_bus: Arc<EventBus>,
    /// Track download progress for each game
    download_progress: Arc<RwLock<std::collections::HashMap<i32, DownloadProgress>>>,
    /// Base directory for game installations (C:/Combatica)
    games_directory: std::path::PathBuf,
}

impl GameVersionService {
    pub fn new(
        repository: Arc<dyn GameVersionRepository>,
        cache_repository: Arc<SledGameCacheRepository>,
        event_bus: Arc<EventBus>,
        games_directory: std::path::PathBuf,
    ) -> Self {
        Self {
            repository,
            cache_repository,
            event_bus,
            download_progress: Arc::new(RwLock::new(std::collections::HashMap::new())),
            games_directory,
        }
    }

    /// Check if a background image exists locally and return it as base64 data URL
    /// Background images are stored at: C:/Combatica/<GameName>/<GameName>BG.jpg
    fn get_background_image_path(&self, game_name: &str) -> Option<String> {
        let bg_path = self.games_directory
            .join(game_name)
            .join(format!("{}BG.jpg", game_name));

        tracing::debug!("Checking for background image at: {:?}", bg_path);

        if bg_path.exists() {
            // Read file and convert to base64 data URL
            match std::fs::read(&bg_path) {
                Ok(bytes) => {
                    let base64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
                    let data_url = format!("data:image/jpeg;base64,{}", base64);
                    tracing::info!("Loaded background image for {} ({} bytes)", game_name, bytes.len());
                    Some(data_url)
                }
                Err(e) => {
                    tracing::warn!("Failed to read background image for {}: {}", game_name, e);
                    None
                }
            }
        } else {
            tracing::debug!("No background image found for {} at {:?}", game_name, bg_path);
            None
        }
    }

    /// Download and save a background image to local storage
    /// Saves to: C:/Combatica/<GameName>/<GameName>BG.jpg
    async fn download_background_image(
        &self,
        game_name: &str,
        download_url: &str,
    ) -> Result<(), GameVersionError> {
        use tokio::io::AsyncWriteExt;

        // Download the image
        let response = reqwest::get(download_url)
            .await
            .map_err(|e| GameVersionError::Network(format!("Failed to download background image: {}", e)))?;

        if !response.status().is_success() {
            return Err(GameVersionError::Network(format!(
                "Failed to download background image: HTTP {}",
                response.status()
            )));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| GameVersionError::Network(format!("Failed to read background image bytes: {}", e)))?;

        // Prepare local path
        let bg_path = self.games_directory
            .join(game_name)
            .join(format!("{}BG.jpg", game_name));

        // Ensure parent directory exists
        if let Some(parent) = bg_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Write the image file
        let mut file = tokio::fs::File::create(&bg_path).await?;
        file.write_all(&bytes).await?;

        Ok(())
    }

    /// Initialize cache on first run by scanning filesystem for existing games
    pub async fn initialize_cache_if_empty(&self) -> Result<(), GameVersionError> {
        let is_empty = self
            .cache_repository
            .is_empty()
            .await
            .map_err(|e| GameVersionError::InvalidMetadata(format!("Cache error: {}", e)))?;

        if is_empty {
            tracing::info!("Cache is empty, scanning filesystem for installed games...");
            let discovered_games = self.repository.scan_installed_games().await?;

            for metadata in discovered_games {
                let entry = CachedGameEntry::from_local_only(metadata);
                self.cache_repository
                    .set_entry(&entry)
                    .await
                    .map_err(|e| {
                        GameVersionError::InvalidMetadata(format!("Cache error: {}", e))
                    })?;
            }

            tracing::info!("Cache initialized from filesystem scan");
        }

        Ok(())
    }

    /// Sync cache with Alakazam server (best effort - doesn't fail if offline)
    /// Returns true if online and synced successfully, false if offline
    pub async fn sync_cache_with_server(&self) -> Result<bool, GameVersionError> {
        tracing::info!("Attempting to sync cache with Alakazam server...");

        match self.repository.fetch_game_assignments().await {
            Ok(assignments) => {
                // Build a lookup for local metadata
                let mut local_metadata_map = std::collections::HashMap::new();
                for assignment in &assignments {
                    if let Ok(Some(metadata)) = self
                        .repository
                        .get_local_metadata(&assignment.game_name)
                        .await
                    {
                        local_metadata_map.insert(assignment.game_name.clone(), metadata);
                    }
                }

                // Sync to cache
                self.cache_repository
                    .sync_from_assignments(assignments, |game_name| {
                        local_metadata_map.get(game_name).cloned()
                    })
                    .await
                    .map_err(|e| {
                        GameVersionError::InvalidMetadata(format!("Cache sync error: {}", e))
                    })?;

                tracing::info!("Cache synced successfully with server");
                Ok(true)
            }
            Err(e) => {
                tracing::warn!("Failed to sync with server (offline?): {}", e);
                Ok(false)
            }
        }
    }

    /// Get all games with their status (cache-first with background sync)
    pub async fn get_game_statuses(&self) -> Result<Vec<GameStatus>, GameVersionError> {
        // Try to sync with server first (best effort)
        let online = self.sync_cache_with_server().await.unwrap_or(false);

        // Always read from cache
        let entries = self
            .cache_repository
            .get_all_entries()
            .await
            .map_err(|e| GameVersionError::InvalidMetadata(format!("Cache error: {}", e)))?;

        let mut statuses = Vec::new();

        for entry in entries {
            let installed_version = entry
                .local_metadata
                .as_ref()
                .map(|m| m.installed_version.clone());

            // Check if update is available
            let update_available = if let Some(ref metadata) = entry.local_metadata {
                // Compare version IDs to determine if update needed
                metadata.installed_version_id != entry.assigned_version.version_id
            } else {
                // No version installed, so update is "available" (first install)
                true
            };

            // Get download progress if downloading
            let download_progress = {
                let progress_map = self.download_progress.read().await;
                progress_map.get(&entry.game_id).cloned()
            };

            // Check for local background image
            let background_image_path = self.get_background_image_path(&entry.game_name);

            statuses.push(GameStatus {
                game_id: entry.game_id,
                game_name: entry.game_name.clone(),
                installed_version,
                assigned_version: entry.assigned_version.version.clone(),
                assigned_version_id: entry.assigned_version.version_id,
                update_available,
                download_progress,
                online,
                last_synced: entry.last_synced,
                background_image_path,
            });
        }

        if statuses.is_empty() && !online {
            tracing::warn!("No games in cache and unable to reach server");
        }

        Ok(statuses)
    }

    /// Download and install a game (or update it)
    pub async fn download_and_install_game(
        &self,
        game_id: i32,
    ) -> Result<(), GameVersionError> {
        // Fetch download URLs from Alakazam
        let download_response = self.repository.fetch_download_urls(game_id).await?;

        let game_name = download_response.game_name.clone();
        let version = download_response.version.clone();
        let version_id = download_response.version_id;
        let total_files = download_response.files.len();

        tracing::info!(
            "Starting download for {} v{} ({} files)",
            game_name,
            version,
            total_files
        );

        // Initialize progress tracking
        {
            let mut progress_map = self.download_progress.write().await;
            progress_map.insert(
                game_id,
                DownloadProgress {
                    total_files,
                    downloaded_files: 0,
                    current_file: String::new(),
                    percentage: 0.0,
                },
            );
        }

        // Download files with progress tracking
        let progress_map = Arc::clone(&self.download_progress);
        let event_bus = Arc::clone(&self.event_bus);
        let game_name_for_callback = game_name.clone();

        let progress_callback = Box::new(move |downloaded: usize, total: usize, current_file: String| {
            let progress_map = Arc::clone(&progress_map);
            let event_bus = Arc::clone(&event_bus);
            let game_name = game_name_for_callback.clone();

            tauri::async_runtime::spawn(async move {
                let percentage = if total > 0 {
                    (downloaded as f32 / total as f32) * 100.0
                } else {
                    0.0
                };

                // Update internal progress tracking
                {
                    let mut map = progress_map.write().await;
                    if let Some(progress) = map.get_mut(&game_id) {
                        progress.downloaded_files = downloaded;
                        progress.total_files = total;
                        progress.current_file = current_file;
                        progress.percentage = percentage;
                    }
                }

                // Emit progress event to frontend
                event_bus.game_download_progress(game_id, game_name, percentage);
            });
        });

        self.repository
            .download_game_files(&game_name, &download_response.files, progress_callback)
            .await?;

        // Download background image if available
        if let Ok(Some(cache_entry)) = self.cache_repository.get_entry(game_id).await {
            if let Some(ref bg_url) = cache_entry.background_image_url {
                tracing::info!("Downloading background image for {}", game_name);
                if let Err(e) = self.download_background_image(&game_name, bg_url).await {
                    // Don't fail the whole download if background image fails
                    tracing::warn!("Failed to download background image for {}: {}", game_name, e);
                }
            }
        }

        // Save local metadata
        let metadata = LocalGameMetadata::new(game_id, game_name.clone(), version, version_id);
        self.repository
            .save_local_metadata(&game_name, &metadata)
            .await?;

        // Update cache with new metadata
        self.cache_repository
            .update_local_metadata(game_id, metadata.clone())
            .await
            .map_err(|e| GameVersionError::InvalidMetadata(format!("Cache update error: {}", e)))?;

        // Report version status to Alakazam
        self.repository
            .report_version_status(game_id, Some(version_id))
            .await?;

        // Emit completion event (100%)
        self.event_bus.game_download_progress(game_id, game_name.clone(), 100.0);

        // Clear progress tracking after delay to allow UI to show completion
        let progress_map = Arc::clone(&self.download_progress);
        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            progress_map.write().await.remove(&game_id);
        });

        tracing::info!("Successfully installed {} v{}", game_name, metadata.installed_version);
        Ok(())
    }

    /// Check for updates on startup (don't auto-download)
    pub async fn check_for_updates(&self) -> Result<Vec<GameStatus>, GameVersionError> {
        tracing::info!("Checking for game updates...");
        let statuses = self.get_game_statuses().await?;

        let updates_available: Vec<_> = statuses
            .iter()
            .filter(|s| s.update_available)
            .collect();

        if updates_available.is_empty() {
            tracing::info!("All games are up to date");
        } else {
            tracing::info!(
                "Updates available for {} game(s): {}",
                updates_available.len(),
                updates_available
                    .iter()
                    .map(|s| format!("{} ({})", s.game_name, s.assigned_version))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }

        Ok(statuses)
    }

    /// Cancel an ongoing download
    pub async fn cancel_download(&self, game_id: i32) {
        self.download_progress.write().await.remove(&game_id);
        tracing::info!("Cancelled download for game {}", game_id);
    }

    /// Force refresh games from server (requires internet connection)
    pub async fn force_refresh(&self) -> Result<Vec<GameStatus>, GameVersionError> {
        let online = self.sync_cache_with_server().await.unwrap_or(false);

        if !online {
            return Err(GameVersionError::Network(
                "Unable to connect to server".to_string(),
            ));
        }

        self.get_game_statuses().await
    }
}
