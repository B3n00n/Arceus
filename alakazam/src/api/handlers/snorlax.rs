use crate::{
    api::MachineId,
    error::Result,
    models::SnorlaxApkResponse,
    services::{ArcadeService, SnorlaxService},
};
use axum::{extract::State, Json};
use std::sync::Arc;

/// GET /api/arcade/snorlax/latest
/// Returns download URL for the latest Snorlax APK
pub async fn get_snorlax_latest(
    State((arcade_service, snorlax_service)): State<(Arc<ArcadeService>, Arc<SnorlaxService>)>,
    MachineId(machine_id): MachineId,
) -> Result<Json<SnorlaxApkResponse>> {
    // Authenticate the arcade
    let _arcade = arcade_service.get_arcade_config(&machine_id).await?;

    // Get the latest Snorlax APK download info from database
    let response = snorlax_service.get_latest_apk_response().await?;

    Ok(Json(response))
}
