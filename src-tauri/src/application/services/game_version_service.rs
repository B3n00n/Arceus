use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::application::dto::LocalGameMetadata;
use crate::domain::repositories::{GameVersionError, GameVersionRepository};

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
    /// Track download progress for each game
    download_progress: Arc<RwLock<std::collections::HashMap<i32, DownloadProgress>>>,
}

impl GameVersionService {
    pub fn new(repository: Arc<dyn GameVersionRepository>) -> Self {
        Self {
            repository,
            download_progress: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Get all games with their status
    pub async fn get_game_statuses(&self) -> Result<Vec<GameStatus>, GameVersionError> {
        // Fetch game assignments from Alakazam
        let assignments = self.repository.fetch_game_assignments().await?;

        let mut statuses = Vec::new();

        for assignment in assignments {
            // Get local metadata to check installed version
            let local_metadata = self
                .repository
                .get_local_metadata(&assignment.game_name)
                .await?;

            let installed_version = local_metadata
                .as_ref()
                .map(|m| m.installed_version.clone());

            // Check if update is available
            let update_available = if let Some(ref metadata) = local_metadata {
                // Compare version IDs to determine if update needed
                metadata.installed_version_id != assignment.assigned_version.version_id
            } else {
                // No version installed, so update is "available" (first install)
                true
            };

            // Get download progress if downloading
            let download_progress = {
                let progress_map = self.download_progress.read().await;
                progress_map.get(&assignment.game_id).cloned()
            };

            statuses.push(GameStatus {
                game_id: assignment.game_id,
                game_name: assignment.game_name,
                installed_version,
                assigned_version: assignment.assigned_version.version.clone(),
                assigned_version_id: assignment.assigned_version.version_id,
                update_available,
                download_progress,
            });
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
        let progress_callback = Box::new(move |downloaded: usize, total: usize, current_file: String| {
            let progress_map_clone = Arc::clone(&progress_map);
            tauri::async_runtime::spawn(async move {
                let mut map = progress_map_clone.write().await;
                if let Some(progress) = map.get_mut(&game_id) {
                    progress.downloaded_files = downloaded;
                    progress.total_files = total;
                    progress.current_file = current_file;
                    progress.percentage = if total > 0 {
                        (downloaded as f32 / total as f32) * 100.0
                    } else {
                        0.0
                    };
                }
            });
        });

        self.repository
            .download_game_files(&game_name, &download_response.files, progress_callback)
            .await?;

        // Save local metadata
        let metadata = LocalGameMetadata::new(game_id, game_name.clone(), version, version_id);
        self.repository
            .save_local_metadata(&game_name, &metadata)
            .await?;

        // Report version status to Alakazam
        self.repository
            .report_version_status(game_id, Some(version_id))
            .await?;

        // Set progress to complete
        {
            let mut progress_map = self.download_progress.write().await;
            if let Some(progress) = progress_map.get_mut(&game_id) {
                progress.downloaded_files = total_files;
                progress.percentage = 100.0;
                progress.current_file = "Complete".to_string();
            }
        }

        // Clear progress after a short delay to let user see completion
        let progress_map_clear = Arc::clone(&self.download_progress);
        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            let mut map = progress_map_clear.write().await;
            map.remove(&game_id);
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
        let mut progress_map = self.download_progress.write().await;
        progress_map.remove(&game_id);
        tracing::info!("Cancelled download for game {}", game_id);
    }

    /// Get download progress for a specific game
    pub async fn get_download_progress(&self, game_id: i32) -> Option<DownloadProgress> {
        let progress_map = self.download_progress.read().await;
        progress_map.get(&game_id).cloned()
    }
}
