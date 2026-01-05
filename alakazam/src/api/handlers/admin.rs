use crate::{
    api::IapUser,
    error::{AppError, Result},
    models::{Arcade, ArcadeGameAssignment, Game, GameVersion, SnorlaxVersion},
    services::{AdminService, GcsService, SnorlaxService},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct CreateArcadeRequest {
    pub name: String,
    pub mac_address: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateArcadeRequest {
    pub name: String,
    pub status: String,
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
pub struct CreateAssignmentRequest {
    pub arcade_id: i32,
    pub game_id: i32,
    pub assigned_version_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAssignmentRequest {
    pub assigned_version_id: i32,
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

/// POST /api/admin/arcades
/// Create a new arcade
pub async fn create_arcade(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Json(payload): Json<CreateArcadeRequest>,
) -> Result<(StatusCode, Json<Arcade>)> {
    let arcade = service.create_arcade(&payload.name, &payload.mac_address).await?;
    Ok((StatusCode::CREATED, Json(arcade)))
}

/// GET /api/admin/arcades
/// List all arcades
pub async fn list_arcades(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
) -> Result<Json<Vec<Arcade>>> {
    let arcades = service.list_arcades().await?;
    Ok(Json(arcades))
}

/// GET /api/admin/arcades/{id}
/// Get arcade by ID
pub async fn get_arcade(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<Json<Arcade>> {
    let arcade = service.get_arcade(id).await?;
    Ok(Json(arcade))
}

/// PUT /api/admin/arcades/{id}
/// Update arcade
pub async fn update_arcade(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateArcadeRequest>,
) -> Result<Json<Arcade>> {
    let arcade = service.update_arcade(id, &payload.name, &payload.status).await?;
    Ok(Json(arcade))
}

/// DELETE /api/admin/arcades/{id}
/// Delete arcade
pub async fn delete_arcade(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    service.delete_arcade(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/admin/arcades/{id}/assignments
/// Get arcade's game assignments
pub async fn get_arcade_assignments(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(arcade_id): Path<i32>,
) -> Result<Json<Vec<ArcadeGameAssignment>>> {
    let assignments = service.get_arcade_assignments(arcade_id).await?;
    Ok(Json(assignments))
}

/// POST /api/admin/games
/// Create a new game
pub async fn create_game(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Json(payload): Json<CreateGameRequest>,
) -> Result<(StatusCode, Json<Game>)> {
    let game = service.create_game(&payload.name).await?;
    Ok((StatusCode::CREATED, Json(game)))
}

/// GET /api/admin/games
/// List all games with background image URLs
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
/// Get game by ID
pub async fn get_game(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<Json<Game>> {
    let game = service.get_game(id).await?;
    Ok(Json(game))
}

/// PUT /api/admin/games/{id}
/// Update game
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
/// Delete game
pub async fn delete_game(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    service.delete_game(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/admin/games/{game_id}/versions
/// Create a new game version
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
/// List all versions for a game
pub async fn list_game_versions(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(game_id): Path<i32>,
) -> Result<Json<Vec<GameVersion>>> {
    let versions = service.list_game_versions(game_id).await?;
    Ok(Json(versions))
}

/// GET /api/admin/games/{game_id}/versions/{version_id}
/// Get specific version
pub async fn get_game_version(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path((_game_id, version_id)): Path<(i32, i32)>,
) -> Result<Json<GameVersion>> {
    let version = service.get_game_version(version_id).await?;
    Ok(Json(version))
}

/// PUT /api/admin/games/{game_id}/versions/{version_id}
/// Update game version
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
/// Delete game version and its files from GCS
pub async fn delete_game_version(
    State((admin_service, gcs_service)): State<(Arc<AdminService>, Arc<GcsService>)>,
    _user: IapUser,
    Path((_game_id, version_id)): Path<(i32, i32)>,
) -> Result<StatusCode> {
    // Get the version to retrieve GCS path before deleting from DB
    let game_version = admin_service.get_game_version(version_id).await?;

    // Delete all files from GCS folder
    gcs_service.delete_folder(&game_version.gcs_path).await?;

    // Delete from database
    admin_service.delete_game_version(version_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/admin/assignments
/// Create new assignment
pub async fn create_assignment(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Json(payload): Json<CreateAssignmentRequest>,
) -> Result<(StatusCode, Json<ArcadeGameAssignment>)> {
    let assignment = service
        .create_assignment(payload.arcade_id, payload.game_id, payload.assigned_version_id)
        .await?;
    Ok((StatusCode::CREATED, Json(assignment)))
}

/// PUT /api/admin/assignments/{id}
/// Update assignment
pub async fn update_assignment(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateAssignmentRequest>,
) -> Result<Json<ArcadeGameAssignment>> {
    let assignment = service
        .update_assignment(id, payload.assigned_version_id)
        .await?;
    Ok(Json(assignment))
}

/// DELETE /api/admin/assignments/{id}
/// Delete assignment
pub async fn delete_assignment(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    service.delete_assignment(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/admin/assignments
/// List all assignments
pub async fn list_assignments(
    State(service): State<Arc<AdminService>>,
    _user: IapUser,
) -> Result<Json<Vec<ArcadeGameAssignment>>> {
    let assignments = service.list_all_assignments().await?;
    Ok(Json(assignments))
}

/// GET /api/admin/snorlax/versions
/// List all Snorlax versions
pub async fn list_snorlax_versions(
    State(service): State<Arc<SnorlaxService>>,
    _user: IapUser,
) -> Result<Json<Vec<SnorlaxVersion>>> {
    let versions = service.get_all_versions().await?;
    Ok(Json(versions))
}

/// POST /api/admin/snorlax/versions
/// Create new Snorlax version
pub async fn create_snorlax_version(
    State(service): State<Arc<SnorlaxService>>,
    _user: IapUser,
    Json(payload): Json<CreateSnorlaxVersionRequest>,
) -> Result<(StatusCode, Json<SnorlaxVersion>)> {
    let version = service.create_version(&payload.version, &payload.gcs_path).await?;
    Ok((StatusCode::CREATED, Json(version)))
}

/// PUT /api/admin/snorlax/versions/{id}/set-current
/// Set version as current
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
/// Delete Snorlax version
pub async fn delete_snorlax_version(
    State(service): State<Arc<SnorlaxService>>,
    _user: IapUser,
    Path(id): Path<i32>,
) -> Result<StatusCode> {
    service.delete_version(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/admin/games/{game_id}/versions/generate-upload-url
/// Generate a signed URL for uploading a game version directly to GCS
pub async fn generate_game_version_upload_url(
    State((admin_service, gcs_service)): State<(Arc<AdminService>, Arc<GcsService>)>,
    _user: IapUser,
    Path(game_id): Path<i32>,
    Json(payload): Json<GenerateUploadUrlRequest>,
) -> Result<Json<GenerateUploadUrlResponse>> {
    let game = admin_service.get_game(game_id).await?;

    // Validate version format (X.Y.Z)
    if !payload.version.split('.').all(|part| part.parse::<u32>().is_ok()) {
        return Err(AppError::BadRequest("Version must be in format X.Y.Z (e.g., 1.0.0)".to_string()));
    }

    // GCS path: {GameName}/{version}/game.zip
    let gcs_folder = format!("{}/{}", game.name, payload.version);
    let gcs_path = format!("{}/game.zip", gcs_folder);

    // Generate signed upload URL valid for 1 hour
    let upload_url = gcs_service.generate_signed_upload_url(&gcs_path, 3600).await?;

    Ok(Json(GenerateUploadUrlResponse {
        upload_url,
        gcs_path: gcs_folder,
    }))
}

/// POST /api/admin/games/{game_id}/versions/confirm-upload
/// Confirm that a game version was uploaded and create database record
pub async fn confirm_game_version_upload(
    State(admin_service): State<Arc<AdminService>>,
    _user: IapUser,
    Path(game_id): Path<i32>,
    Json(payload): Json<ConfirmGameVersionUploadRequest>,
) -> Result<(StatusCode, Json<GameVersion>)> {
    // Validate version format (X.Y.Z)
    if !payload.version.split('.').all(|part| part.parse::<u32>().is_ok()) {
        return Err(AppError::BadRequest("Version must be in format X.Y.Z (e.g., 1.0.0)".to_string()));
    }

    // Create database record with folder path
    let game_version = admin_service
        .create_game_version(game_id, &payload.version, &payload.gcs_path)
        .await?;

    Ok((StatusCode::CREATED, Json(game_version)))
}

/// POST /api/admin/snorlax/generate-upload-url
/// Generate a signed URL for uploading Snorlax APK directly to GCS
pub async fn generate_snorlax_upload_url(
    State(gcs_service): State<Arc<GcsService>>,
    _user: IapUser,
    Json(payload): Json<GenerateUploadUrlRequest>,
) -> Result<Json<GenerateUploadUrlResponse>> {
    let gcs_path = format!("Snorlax/{}", payload.version);
    let apk_path = format!("{}/Snorlax.apk", gcs_path);

    // Generate signed upload URL valid for 1 hour
    let upload_url = gcs_service.generate_signed_upload_url(&apk_path, 3600).await?;

    Ok(Json(GenerateUploadUrlResponse {
        upload_url,
        gcs_path,
    }))
}

/// POST /api/admin/snorlax/confirm-upload
/// Confirm that Snorlax APK was uploaded and create database record
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

/// POST /api/admin/games/{game_id}/background/generate-upload-url
/// Generate a signed URL for uploading game background image directly to GCS
pub async fn generate_background_upload_url(
    State((admin_service, gcs_service)): State<(Arc<AdminService>, Arc<GcsService>)>,
    _user: IapUser,
    Path(game_id): Path<i32>,
) -> Result<Json<GenerateUploadUrlResponse>> {
    let game = admin_service.get_game(game_id).await?;

    let gcs_path = format!("{}/{}BG.jpg", game.name, game.name);

    // Generate signed upload URL valid for 30 minutes
    let upload_url = gcs_service.generate_signed_upload_url(&gcs_path, 1800).await?;

    Ok(Json(GenerateUploadUrlResponse {
        upload_url,
        gcs_path: gcs_path.clone(),
    }))
}
