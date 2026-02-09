use crate::{
    api::{IapUser, MachineId},
    error::{AppError, Result},
    models::SensorWithArcade,
    services::SensorService,
};
use axum::{
    extract::State,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct ReportSensorRequest {
    pub serial_number: String,
    pub mac_address: Option<String>,
    pub firmware_version: Option<String>,
}

/// GET /api/admin/sensors — list all tracked sensors (for Giratina)
pub async fn list_sensors(
    State(service): State<Arc<SensorService>>,
    _user: IapUser,
) -> Result<Json<Vec<SensorWithArcade>>> {
    let sensors = service.get_all_sensors().await?;
    Ok(Json(sensors))
}

/// POST /api/arcade/sensors/report — report sensor from an arcade (called by Arceus)
pub async fn report_sensor(
    State(service): State<Arc<SensorService>>,
    MachineId(machine_id): MachineId,
    Json(payload): Json<ReportSensorRequest>,
) -> Result<Json<serde_json::Value>> {
    if payload.serial_number.trim().is_empty() {
        return Err(AppError::BadRequest("serial_number is required".to_string()));
    }

    service
        .report_sensor(
            &machine_id,
            &payload.serial_number,
            payload.mac_address.as_deref(),
            payload.firmware_version.as_deref(),
        )
        .await?;

    Ok(Json(serde_json::json!({ "status": "ok" })))
}
