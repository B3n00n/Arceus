use crate::{
    api::MacKey,
    error::{AppError, Result},
    services::{ArcadeService, GcsService},
};
use axum::{extract::{Path, State}, Json};
use chrono::Utc;
use serde::Serialize;
use std::sync::Arc;

/// Response for game download request
#[derive(Debug, Serialize)]
pub struct GameDownloadResponse {
    pub game_id: i32,
    pub game_name: String,
    pub version: String,
    pub version_id: i32,
    pub gcs_path: String,
    pub files: Vec<GameFile>,
    pub expires_at: chrono::DateTime<Utc>,
}

/// Individual file to download
#[derive(Debug, Serialize)]
pub struct GameFile {
    pub path: String,
    pub download_url: String,
}

/// GET /api/arcade/games/{game_id}/download
/// Returns signed download URLs for all files in the game version
pub async fn get_game_download_urls(
    State((arcade_service, gcs_service)): State<(Arc<ArcadeService>, Arc<GcsService>)>,
    Path(game_id): Path<i32>,
    MacKey(mac_key): MacKey,
) -> Result<Json<GameDownloadResponse>> {
    // Authenticate the arcade
    let _arcade = arcade_service.get_arcade_config(&mac_key).await?;

    // Get the arcade's game assignments
    let games = arcade_service.get_arcade_games(&mac_key).await?;

    // Find the requested game
    let game_assignment = games
        .iter()
        .find(|g| g.game_id == game_id)
        .ok_or(AppError::GameNotFound)?;

    // Use the assigned version
    let version = &game_assignment.assigned_version;

    // List all files in the GCS path and generate signed URLs
    let files = gcs_service
        .list_and_sign_folder(&version.gcs_path)
        .await?;

    // Calculate expiration time
    let duration_secs = gcs_service.get_url_duration_secs();
    let expires_at = Utc::now() + chrono::Duration::seconds(duration_secs as i64);

    Ok(Json(GameDownloadResponse {
        game_id: game_assignment.game_id,
        game_name: game_assignment.game_name.clone(),
        version: version.version.clone(),
        version_id: version.version_id,
        gcs_path: version.gcs_path.clone(),
        files,
        expires_at,
    }))
}
