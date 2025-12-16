use crate::{
    api::ApiKey,
    error::Result,
    models::{ArcadeConfigResponse, GameAssignmentResponse, UpdateStatusRequest},
    services::ArcadeService,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;

/// GET /api/arcade/config
/// Returns arcade configuration (authenticated by API key header)
pub async fn get_arcade_config(
    State(service): State<Arc<ArcadeService>>,
    ApiKey(api_key): ApiKey,
) -> Result<Json<ArcadeConfigResponse>> {
    let config = service.get_arcade_config(&api_key).await?;
    Ok(Json(config))
}

/// GET /api/arcade/games
/// Returns all game assignments for the arcade
pub async fn get_arcade_games(
    State(service): State<Arc<ArcadeService>>,
    ApiKey(api_key): ApiKey,
) -> Result<Json<Vec<GameAssignmentResponse>>> {
    let games = service.get_arcade_games(&api_key).await?;
    Ok(Json(games))
}

/// POST /api/arcade/games/{game_id}/status
/// Update the current version status for a game
pub async fn update_game_status(
    State(service): State<Arc<ArcadeService>>,
    Path(game_id): Path<i32>,
    ApiKey(api_key): ApiKey,
    Json(payload): Json<UpdateStatusRequest>,
) -> Result<StatusCode> {
    service
        .update_game_status(&api_key, game_id, payload.current_version_id)
        .await?;

    Ok(StatusCode::OK)
}
