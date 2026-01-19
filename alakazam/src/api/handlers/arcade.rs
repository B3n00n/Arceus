use crate::{
    api::MachineId,
    error::Result,
    models::{ArcadeConfigResponse, GameAssignmentResponse},
    services::ArcadeService,
};
use axum::{
    extract::State,
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
