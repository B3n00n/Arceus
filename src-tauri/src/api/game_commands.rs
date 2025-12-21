use crate::application::services::{GameApplicationService, GameVersionService, GameStatus};
use crate::domain::models::{GameConfig, PackageName};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;

/// DTO for game configuration from frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameConfigDto {
    pub name: String,
    pub exe_path: String,
    pub content_path: String,
    pub package_name: String,
}

/// DTO for game state to frontend - frontend only needs game name
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStateDto {
    pub game_name: String,
}

impl From<crate::domain::models::GameState> for GameStateDto {
    fn from(state: crate::domain::models::GameState) -> Self {
        Self {
            game_name: state.config.name,
        }
    }
}

#[tauri::command]
pub async fn start_game(
    config_dto: GameConfigDto,
    game_service: State<'_, Arc<GameApplicationService>>,
) -> Result<GameStateDto, String> {
    tracing::info!(
        game = %config_dto.name,
        "Received start_game command"
    );

    let package_name = PackageName::new(config_dto.package_name)
        .map_err(|e| format!("Invalid package name: {}", e))?;

    let config = GameConfig::new(
        config_dto.name,
        PathBuf::from(config_dto.exe_path),
        PathBuf::from(config_dto.content_path),
        package_name,
    );

    let game_state = game_service
        .start_game(config)
        .await
        .map_err(|e| format!("Failed to start game: {}", e))?;

    Ok(game_state.into())
}

#[tauri::command]
pub async fn get_current_game(
    game_service: State<'_, Arc<GameApplicationService>>,
) -> Result<Option<GameStateDto>, String> {
    Ok(game_service.get_current_game().map(|state| state.into()))
}

#[tauri::command]
pub async fn stop_game(
    game_service: State<'_, Arc<GameApplicationService>>,
) -> Result<(), String> {
    game_service
        .stop_game()
        .await
        .map_err(|e| format!("Failed to stop game: {}", e))
}

/// Get list of all games with their version status
#[tauri::command]
pub async fn get_game_list(
    game_version_service: State<'_, Arc<GameVersionService>>,
) -> Result<Vec<GameStatus>, String> {
    tracing::debug!("Fetching game list");
    game_version_service
        .get_game_statuses()
        .await
        .map_err(|e| format!("Failed to fetch game list: {}", e))
}

/// Download and install a game (or update it)
#[tauri::command]
pub async fn download_game(
    game_id: i32,
    game_version_service: State<'_, Arc<GameVersionService>>,
) -> Result<(), String> {
    tracing::info!("Starting download for game {}", game_id);
    game_version_service
        .download_and_install_game(game_id)
        .await
        .map_err(|e| format!("Failed to download game: {}", e))
}

/// Cancel an ongoing download
#[tauri::command]
pub async fn cancel_download(
    game_id: i32,
    game_version_service: State<'_, Arc<GameVersionService>>,
) -> Result<(), String> {
    tracing::info!("Cancelling download for game {}", game_id);
    game_version_service.cancel_download(game_id).await;
    Ok(())
}
