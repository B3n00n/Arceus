use crate::{
    api::MachineId,
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
/// Returns arcade configuration (authenticated by machine ID header)
pub async fn get_arcade_config(
    State(service): State<Arc<ArcadeService>>,
    MachineId(machine_id): MachineId,
) -> Result<Json<ArcadeConfigResponse>> {
    let config = service.get_arcade_config(&machine_id).await?;
    Ok(Json(config))
}

/// GET /api/arcade/games
/// Returns all game assignments for the arcade
pub async fn get_arcade_games(
    State(service): State<Arc<ArcadeService>>,
    MachineId(machine_id): MachineId,
) -> Result<Json<Vec<GameAssignmentResponse>>> {
    let games = service.get_arcade_games(&machine_id).await?;
    Ok(Json(games))
}

/// POST /api/arcade/games/{game_id}/status
/// Update the current version status for a game
pub async fn update_game_status(
    State(service): State<Arc<ArcadeService>>,
    Path(game_id): Path<i32>,
    MachineId(machine_id): MachineId,
    Json(payload): Json<UpdateStatusRequest>,
) -> Result<StatusCode> {
    service
        .update_game_status(&machine_id, game_id, payload.current_version_id)
        .await?;

    Ok(StatusCode::OK)
}
