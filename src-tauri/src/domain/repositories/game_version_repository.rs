use crate::application::dto::{GameAssignment, GameDownloadResponse, LocalGameMetadata};
use async_trait::async_trait;
use std::path::PathBuf;

/// Repository for managing game versions
/// Handles fetching game assignments, downloading game files, and tracking installed versions
#[async_trait]
pub trait GameVersionRepository: Send + Sync {
    /// Fetch all game assignments for this arcade from Alakazam
    /// Returns list of games assigned to this arcade with version information
    async fn fetch_game_assignments(&self) -> Result<Vec<GameAssignment>, GameVersionError>;

    /// Fetch download URLs for a specific game from Alakazam
    /// Returns signed URLs for all files in the game version
    async fn fetch_download_urls(&self, game_id: i32) -> Result<GameDownloadResponse, GameVersionError>;

    /// Download all files for a game version
    /// Downloads each file and saves it to the appropriate location
    /// Calls progress_callback after each file with (downloaded_count, total_count, current_file)
    async fn download_game_files(
        &self,
        game_name: &str,
        files: &[crate::application::dto::GameFile],
        progress_callback: Box<dyn Fn(usize, usize, String) + Send + Sync>,
    ) -> Result<(), GameVersionError>;

    /// Get local metadata for an installed game
    /// Returns None if the game is not installed
    async fn get_local_metadata(&self, game_name: &str) -> Result<Option<LocalGameMetadata>, GameVersionError>;

    /// Save local metadata after installing a game
    async fn save_local_metadata(
        &self,
        game_name: &str,
        metadata: &LocalGameMetadata,
    ) -> Result<(), GameVersionError>;

    /// Report current version status to Alakazam
    /// Updates the server with the currently installed version
    async fn report_version_status(
        &self,
        game_id: i32,
        version_id: Option<i32>,
    ) -> Result<(), GameVersionError>;

    /// Get the installation directory for a game
    fn get_game_directory(&self, game_name: &str) -> PathBuf;

    /// Scan the games directory and discover all installed games
    /// Returns a list of LocalGameMetadata for all games found
    async fn scan_installed_games(&self) -> Result<Vec<LocalGameMetadata>, GameVersionError>;
}

/// Errors that can occur during game version operations
#[derive(Debug, thiserror::Error)]
pub enum GameVersionError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),

    #[error("Invalid metadata: {0}")]
    InvalidMetadata(String),

    #[error("Version parse error: {0}")]
    VersionParse(#[from] semver::Error),

    #[error("Game not found")]
    GameNotFound,

    #[error("Download failed for file {file}: {error}")]
    DownloadFailed { file: String, error: String },
}
