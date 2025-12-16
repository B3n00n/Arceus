use crate::{
    api::MacKey,
    error::Result,
    models::SnorlaxApkResponse,
    services::{ArcadeService, GcsService},
};
use axum::{extract::State, Json};
use chrono::Utc;
use std::sync::Arc;

const SNORLAX_APK_PATH: &str = "Snorlax.apk";

/// GET /api/arcade/snorlax/latest
/// Returns download URL for the latest Snorlax APK
pub async fn get_snorlax_latest(
    State((arcade_service, gcs_service)): State<(Arc<ArcadeService>, Arc<GcsService>)>,
    MacKey(mac_key): MacKey,
) -> Result<Json<SnorlaxApkResponse>> {
    // Authenticate the arcade
    let _arcade = arcade_service.get_arcade_config(&mac_key).await?;

    // Generate signed URL for Snorlax.apk
    let download_url = gcs_service
        .generate_signed_download_url(SNORLAX_APK_PATH)
        .await?;

    // Calculate expiration time (current time + duration from config)
    let duration_secs = gcs_service.get_url_duration_secs();
    let expires_at = Utc::now() + chrono::Duration::seconds(duration_secs as i64);

    Ok(Json(SnorlaxApkResponse {
        download_url,
        expires_at,
        version: gcs_service.get_snorlax_version().to_string(),
    }))
}
