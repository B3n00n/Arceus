use crate::{
    api::IapUser,
    error::{AppError, Result},
    models::{
        Arcade, CreateChannelRequest, Game, GameVersion, GameVersionWithChannels,
        GyrosVersion, PublishVersionRequest, ReleaseChannel, SnorlaxVersion,
        UpdateArcadeChannelRequest, UpdateChannelRequest,
    },
    services::{AdminService, GcsService, GyrosService, SnorlaxService},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateArcadeRequest {
    pub name: String,
    pub machine_id: String,
    pub channel_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArcadeRequest {
    pub name: String,
    pub status: String,
    pub channel_id: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGameRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGameRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateGameVersionRequest {
    pub version: String,
    pub gcs_path: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGameVersionRequest {
    pub version: String,
    pub gcs_path: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSnorlaxVersionRequest {
    pub version: String,
    pub gcs_path: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateUploadUrlRequest {
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateUploadUrlResponse {
    pub upload_url: String,
    pub gcs_path: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmGameVersionUploadRequest {
    pub version: String,
    pub gcs_path: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmSnorlaxUploadRequest {
    pub version: String,
    pub gcs_path: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateGyrosVersionRequest {
    pub version: String,
    pub gcs_path: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmGyrosUploadRequest {
    pub version: String,
    pub gcs_path: String,
}

#[derive(Debug, Serialize)]
pub struct AdminActionResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct GameWithBackground {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub background_url: Option<String>,
}

// ============================================================================
// ARCADE ENDPOINTS
// ============================================================================

/// POST /api/admin/arcades
pub async fn create_arcade(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Json(payload): Json<CreateArcadeRequest>,
) -> Result<(StatusCode, Json<Arcade>)> {
    let arcade = service.create_arcade(&payload.name, &payload.machine_id, payload.channel_id).await?;
    Ok((StatusCode::CREATED, Json(arcade)))
}

/// GET /api/admin/arcades
pub async fn list_arcades(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
) -> Result<Json<Vec<Arcade>>> {
    let arcades = service.list_arcades().await?;
    Ok(Json(arcades))
}

/// GET /api/admin/arcades/{id}
pub async fn get_arcade(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<Json<Arcade>> {
    let arcade = service.get_arcade(id).await?;
    Ok(Json(arcade))
}

/// PUT /api/admin/arcades/{id}
pub async fn update_arcade(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateArcadeRequest>,
) -> Result<Json<Arcade>> {
    let mut arcade = service.update_arcade(id, &payload.name, &payload.status).await?;

    // Update channel if provided
    if let Some(channel_id) = payload.channel_id {
        arcade = service.update_arcade_channel(id, channel_id).await?;
    }

    Ok(Json(arcade))
}

/// DELETE /api/admin/arcades/{id}
pub async fn delete_arcade(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    service.delete_arcade(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// PUT /api/admin/arcades/{id}/channel
pub async fn update_arcade_channel(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateArcadeChannelRequest>,
) -> Result<Json<Arcade>> {
    let arcade = service.update_arcade_channel(id, payload.channel_id).await?;
    Ok(Json(arcade))
}

// ============================================================================
// RELEASE CHANNEL ENDPOINTS
// ============================================================================

/// GET /api/admin/channels
pub async fn list_channels(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
) -> Result<Json<Vec<ReleaseChannel>>> {
    let channels = service.list_channels().await?;
    Ok(Json(channels))
}

/// GET /api/admin/channels/{id}
pub async fn get_channel(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<Json<ReleaseChannel>> {
    let channel = service.get_channel(id).await?;
    Ok(Json(channel))
}

/// POST /api/admin/channels
pub async fn create_channel(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Json(payload): Json<CreateChannelRequest>,
) -> Result<(StatusCode, Json<ReleaseChannel>)> {
    let channel = service.create_channel(&payload.name, payload.description.as_deref()).await?;
    Ok((StatusCode::CREATED, Json(channel)))
}

/// PUT /api/admin/channels/{id}
pub async fn update_channel(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateChannelRequest>,
) -> Result<Json<ReleaseChannel>> {
    let channel = service.update_channel(id, payload.description.as_deref()).await?;
    Ok(Json(channel))
}

/// DELETE /api/admin/channels/{id}
pub async fn delete_channel(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    service.delete_channel(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// GAME ENDPOINTS
// ============================================================================

/// POST /api/admin/games
pub async fn create_game(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Json(payload): Json<CreateGameRequest>,
) -> Result<(StatusCode, Json<Game>)> {
    let game = service.create_game(&payload.name).await?;
    Ok((StatusCode::CREATED, Json(game)))
}

/// GET /api/admin/games
pub async fn list_games(
    State((admin_service, gcs_service)): State<(Arc<AdminService>, Arc<GcsService>)>,
    _user: IapUser,
) -> Result<Json<Vec<GameWithBackground>>> {
    let games = admin_service.list_games().await?;

    let mut games_with_bg = Vec::new();
    for game in games {
        let bg_path = format!("{}/{}BG.jpg", game.name, game.name);
        let background_url = gcs_service.generate_signed_download_url(&bg_path).await.ok();

        games_with_bg.push(GameWithBackground {
            id: game.id,
            name: game.name,
            created_at: game.created_at,
            background_url,
        });
    }

    Ok(Json(games_with_bg))
}

/// GET /api/admin/games/{id}
pub async fn get_game(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<Json<Game>> {
    let game = service.get_game(id).await?;
    Ok(Json(game))
}

/// PUT /api/admin/games/{id}
pub async fn update_game(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateGameRequest>,
) -> Result<Json<Game>> {
    let game = service.update_game(id, &payload.name).await?;
    Ok(Json(game))
}

/// DELETE /api/admin/games/{id}
pub async fn delete_game(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    service.delete_game(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// GAME VERSION ENDPOINTS
// ============================================================================

/// POST /api/admin/games/{game_id}/versions
pub async fn create_game_version(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(game_id): Path<i32>,
    Json(payload): Json<CreateGameVersionRequest>,
) -> Result<(StatusCode, Json<GameVersion>)> {
    let version = service
        .create_game_version(game_id, &payload.version, &payload.gcs_path)
        .await?;
    Ok((StatusCode::CREATED, Json(version)))
}

/// GET /api/admin/games/{game_id}/versions
pub async fn list_game_versions(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(game_id): Path<i32>,
) -> Result<Json<Vec<GameVersionWithChannels>>> {
    let versions = service.list_game_versions_with_channels(game_id).await?;
    Ok(Json(versions))
}

/// GET /api/admin/games/{game_id}/versions/{version_id}
pub async fn get_game_version(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path((_game_id, version_id)): Path<(i32, i32)>,
) -> Result<Json<GameVersionWithChannels>> {
    let version = service.get_game_version_with_channels(version_id).await?;
    Ok(Json(version))
}

/// PUT /api/admin/games/{game_id}/versions/{version_id}
pub async fn update_game_version(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path((_game_id, version_id)): Path<(i32, i32)>,
    Json(payload): Json<UpdateGameVersionRequest>,
) -> Result<Json<GameVersion>> {
    let version = service
        .update_game_version(version_id, &payload.version, &payload.gcs_path)
        .await?;
    Ok(Json(version))
}

/// DELETE /api/admin/games/{game_id}/versions/{version_id}
pub async fn delete_game_version(
    State((admin_service, gcs_service)): State<(Arc<AdminService>, Arc<GcsService>)>,
    _user: IapUser,
    Path((_game_id, version_id)): Path<(i32, i32)>,
) -> Result<StatusCode> {
    let game_version = admin_service.get_game_version(version_id).await?;
    gcs_service.delete_folder(&game_version.gcs_path).await?;
    admin_service.delete_game_version(version_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/admin/games/{game_id}/versions/{version_id}/publish
pub async fn publish_version(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path((game_id, version_id)): Path<(i32, i32)>,
    Json(payload): Json<PublishVersionRequest>,
) -> Result<Json<GameVersionWithChannels>> {
    let version = service.get_game_version(version_id).await?;
    if version.game_id != game_id {
        return Err(AppError::BadRequest(
            "Version does not belong to the specified game".to_string()
        ));
    }

    let version = service
        .replace_version_channels(version_id, &payload.channel_ids)
        .await?;

    Ok(Json(version))
}

/// DELETE /api/admin/games/{game_id}/versions/{version_id}/publish
pub async fn unpublish_version(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path((game_id, version_id)): Path<(i32, i32)>,
) -> Result<StatusCode> {
    let version = service.get_game_version(version_id).await?;
    if version.game_id != game_id {
        return Err(AppError::BadRequest(
            "Version does not belong to the specified game".to_string()
        ));
    }

    service.unpublish_version_from_all(version_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// FILE UPLOAD ENDPOINTS
// ============================================================================

/// POST /api/admin/games/{game_id}/versions/generate-upload-url
pub async fn generate_game_version_upload_url(
    State((admin_service, gcs_service)): State<(Arc<AdminService>, Arc<GcsService>)>,
    _user: IapUser,
    Path(game_id): Path<i32>,
    Json(payload): Json<GenerateUploadUrlRequest>,
) -> Result<Json<GenerateUploadUrlResponse>> {
    let game = admin_service.get_game(game_id).await?;

    if !payload.version.split('.').all(|part| part.parse::<u32>().is_ok()) {
        return Err(AppError::BadRequest("Version must be in format X.Y.Z (e.g., 1.0.0)".to_string()));
    }

    let gcs_folder = format!("{}/{}", game.name, payload.version);
    let gcs_path = format!("{}/game.zip", gcs_folder);
    let upload_url = gcs_service.generate_signed_upload_url(&gcs_path, 3600).await?;

    Ok(Json(GenerateUploadUrlResponse {
        upload_url,
        gcs_path: gcs_folder,
    }))
}

/// POST /api/admin/games/{game_id}/versions/confirm-upload
pub async fn confirm_game_version_upload(
    State(admin_service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(game_id): Path<i32>,
    Json(payload): Json<ConfirmGameVersionUploadRequest>,
) -> Result<(StatusCode, Json<GameVersion>)> {
    if !payload.version.split('.').all(|part| part.parse::<u32>().is_ok()) {
        return Err(AppError::BadRequest("Version must be in format X.Y.Z (e.g., 1.0.0)".to_string()));
    }

    let game_version = admin_service
        .create_game_version(game_id, &payload.version, &payload.gcs_path)
        .await?;

    Ok((StatusCode::CREATED, Json(game_version)))
}

/// POST /api/admin/games/{game_id}/background/generate-upload-url
pub async fn generate_background_upload_url(
    State((admin_service, gcs_service)): State<(Arc<AdminService>, Arc<GcsService>)>,
    _user: IapUser,
    Path(game_id): Path<i32>,
) -> Result<Json<GenerateUploadUrlResponse>> {
    let game = admin_service.get_game(game_id).await?;
    let gcs_path = format!("{}/{}BG.jpg", game.name, game.name);
    let upload_url = gcs_service.generate_signed_upload_url(&gcs_path, 1800).await?;

    Ok(Json(GenerateUploadUrlResponse {
        upload_url,
        gcs_path: gcs_path.clone(),
    }))
}

// ============================================================================
// SNORLAX ENDPOINTS
// ============================================================================

/// GET /api/admin/snorlax/versions
pub async fn list_snorlax_versions(
    State(service): State<Arc<SnorlaxService>>,
    _user: IapUser,
) -> Result<Json<Vec<SnorlaxVersion>>> {
    let versions = service.get_all_versions().await?;
    Ok(Json(versions))
}

/// POST /api/admin/snorlax/versions
pub async fn create_snorlax_version(
    State(service): State<Arc<SnorlaxService>>,
    _user: IapUser,
    Json(payload): Json<CreateSnorlaxVersionRequest>,
) -> Result<(StatusCode, Json<SnorlaxVersion>)> {
    let version = service.create_version(&payload.version, &payload.gcs_path).await?;
    Ok((StatusCode::CREATED, Json(version)))
}

/// PUT /api/admin/snorlax/versions/{id}/set-current
pub async fn set_current_snorlax_version(
    State(service): State<Arc<SnorlaxService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<Json<AdminActionResponse>> {
    service.set_current_version(id).await?;
    Ok(Json(AdminActionResponse {
        message: format!("Version {} set as current", id),
    }))
}

/// DELETE /api/admin/snorlax/versions/{id}
pub async fn delete_snorlax_version(
    State(service): State<Arc<SnorlaxService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    service.delete_version(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/admin/snorlax/generate-upload-url
pub async fn generate_snorlax_upload_url(
    State(gcs_service): State<Arc<GcsService>>,
    _user: IapUser,
    Json(payload): Json<GenerateUploadUrlRequest>,
) -> Result<Json<GenerateUploadUrlResponse>> {
    let gcs_path = format!("Snorlax/{}", payload.version);
    let apk_path = format!("{}/Snorlax.apk", gcs_path);
    let upload_url = gcs_service.generate_signed_upload_url(&apk_path, 3600).await?;

    Ok(Json(GenerateUploadUrlResponse {
        upload_url,
        gcs_path,
    }))
}

/// POST /api/admin/snorlax/confirm-upload
pub async fn confirm_snorlax_upload(
    State(snorlax_service): State<Arc<SnorlaxService>>,
    _user: IapUser,
    Json(payload): Json<ConfirmSnorlaxUploadRequest>,
) -> Result<(StatusCode, Json<SnorlaxVersion>)> {
    let snorlax_version = snorlax_service
        .create_version(&payload.version, &payload.gcs_path)
        .await?;

    Ok((StatusCode::CREATED, Json(snorlax_version)))
}

// ============================================================================
// GYROS ENDPOINTS
// ============================================================================

/// GET /api/admin/gyros/versions
pub async fn list_gyros_versions(
    State(service): State<Arc<GyrosService>>,
    _user: IapUser,
) -> Result<Json<Vec<GyrosVersion>>> {
    let versions = service.get_all_versions().await?;
    Ok(Json(versions))
}

/// POST /api/admin/gyros/versions
pub async fn create_gyros_version(
    State(service): State<Arc<GyrosService>>,
    _user: IapUser,
    Json(payload): Json<CreateGyrosVersionRequest>,
) -> Result<(StatusCode, Json<GyrosVersion>)> {
    let version = service.create_version(&payload.version, &payload.gcs_path).await?;
    Ok((StatusCode::CREATED, Json(version)))
}

/// PUT /api/admin/gyros/versions/{id}/set-current
pub async fn set_current_gyros_version(
    State(service): State<Arc<GyrosService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<Json<AdminActionResponse>> {
    service.set_current_version(id).await?;
    Ok(Json(AdminActionResponse {
        message: format!("Version {} set as current", id),
    }))
}

/// DELETE /api/admin/gyros/versions/{id}
pub async fn delete_gyros_version(
    State(service): State<Arc<GyrosService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    service.delete_version(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/admin/gyros/generate-upload-url
pub async fn generate_gyros_upload_url(
    State(gcs_service): State<Arc<GcsService>>,
    _user: IapUser,
    Json(payload): Json<GenerateUploadUrlRequest>,
) -> Result<Json<GenerateUploadUrlResponse>> {
    let gcs_path = format!("Gyros/{}", payload.version);
    let firmware_path = format!("{}/Gyros.bin", gcs_path);
    let upload_url = gcs_service.generate_signed_upload_url(&firmware_path, 3600).await?;

    Ok(Json(GenerateUploadUrlResponse {
        upload_url,
        gcs_path,
    }))
}

/// POST /api/admin/gyros/confirm-upload
pub async fn confirm_gyros_upload(
    State(gyros_service): State<Arc<GyrosService>>,
    _user: IapUser,
    Json(payload): Json<ConfirmGyrosUploadRequest>,
) -> Result<(StatusCode, Json<GyrosVersion>)> {
    let gyros_version = gyros_service
        .create_version(&payload.version, &payload.gcs_path)
        .await?;

    Ok((StatusCode::CREATED, Json(gyros_version)))
}
