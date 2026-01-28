use crate::{
    api::MachineId,
    error::{AppError, Result},
    services::{ArcadeService, GcsService},
};
use axum::{extract::{Path, State}, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
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
    pub background_image_url: Option<String>,
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
    MachineId(machine_id): MachineId,
) -> Result<Json<GameDownloadResponse>> {
    // Authenticate the arcade
    let _arcade = arcade_service.get_arcade_config(&machine_id).await?;

    // Get the arcade's game assignments
    let games = arcade_service.get_arcade_games(&machine_id).await?;

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

    // Background image path: <GameName>/<GameName>BG.jpg
    let background_image_url = {
        let bg_path = format!("{}/{}BG.jpg", game_assignment.game_name, game_assignment.game_name);
        gcs_service
            .generate_signed_download_url(&bg_path)
            .await
            .ok()
    };

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
        background_image_url,
        expires_at,
    }))
}

/// Request for reporting installed games
#[derive(Debug, Deserialize)]
pub struct ReportInstallationsRequest {
    pub installed_games: serde_json::Value,
}

/// POST /api/arcade/games/status
/// Arcade reports all its installed games and versions
pub async fn report_installations(
    State(arcade_service): State<Arc<ArcadeService>>,
    MachineId(machine_id): MachineId,
    Json(payload): Json<ReportInstallationsRequest>,
) -> Result<Json<serde_json::Value>> {
    // Authenticate and update installations
    arcade_service
        .update_installed_games(&machine_id, payload.installed_games)
        .await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Installations updated successfully"
    })))
}
