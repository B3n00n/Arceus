use crate::{
    api::IapUser,
    error::{AppError, Result},
    models::{Arcade, ArcadeGameAssignment, Game, GameVersion, SnorlaxVersion},
    services::{AdminService, GcsService, SnorlaxService},
};
use axum::{
    extract::{Multipart, Path, State},
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

/// POST /api/admin/snorlax/upload
/// Upload Snorlax APK and create new version
pub async fn upload_snorlax_apk(
    State((snorlax_service, gcs_service)): State<(Arc<SnorlaxService>, Arc<GcsService>)>,
    _user: IapUser,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<SnorlaxVersion>)> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut version: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::BadRequest(format!("Failed to read multipart field: {}", e))
    })? {
        match field.name() {
            Some("file") => {
                file_data = Some(field.bytes().await.map_err(|e| {
                    AppError::BadRequest(format!("Failed to read file data: {}", e))
                })?.to_vec());
            }
            Some("version") => {
                version = Some(field.text().await.map_err(|e| {
                    AppError::BadRequest(format!("Failed to read version: {}", e))
                })?);
            }
            _ => {}
        }
    }

    let file_data = file_data.ok_or_else(|| {
        AppError::BadRequest("No file provided".to_string())
    })?;

    let version = version.ok_or_else(|| {
        AppError::BadRequest("Version is required".to_string())
    })?;

    if file_data.len() < 100 {
        return Err(AppError::BadRequest("File too small".to_string()));
    }

    let gcs_path = format!("Snorlax/{}", version);

    gcs_service
        .upload_file(
            &format!("{}/Snorlax.apk", gcs_path),
            "application/vnd.android.package-archive",
            file_data,
        )
        .await?;

    let snorlax_version = snorlax_service.create_version(&version, &gcs_path).await?;

    Ok((StatusCode::CREATED, Json(snorlax_version)))
}

/// POST /api/admin/games/{id}/background
/// Upload background image for a game (JPEG only)
pub async fn upload_game_background(
    State((admin_service, gcs_service)): State<(Arc<AdminService>, Arc<GcsService>)>,
    _user: IapUser,
    Path(game_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<Json<AdminActionResponse>> {
    let game = admin_service.get_game(game_id).await?;

    let mut file_data: Option<Vec<u8>> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::BadRequest(format!("Failed to read multipart field: {}", e))
    })? {
        if field.name() == Some("file") {
            file_data = Some(field.bytes().await.map_err(|e| {
                AppError::BadRequest(format!("Failed to read file data: {}", e))
            })?.to_vec());
            break;
        }
    }

    let file_data = file_data.ok_or_else(|| {
        AppError::BadRequest("No file provided".to_string())
    })?;

    if file_data.len() < 100 {
        return Err(AppError::BadRequest("File too small".to_string()));
    }

    let gcs_path = format!("{}/{}BG.jpg", game.name, game.name);

    gcs_service
        .upload_file(&gcs_path, "image/jpeg", file_data)
        .await?;

    Ok(Json(AdminActionResponse {
        message: format!("Background uploaded: {}", gcs_path),
    }))
}

/// POST /api/admin/games/{game_id}/versions/upload
/// Upload game version ZIP file and create database record
pub async fn upload_game_version(
    State((admin_service, gcs_service)): State<(Arc<AdminService>, Arc<GcsService>)>,
    _user: IapUser,
    Path(game_id): Path<i32>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<GameVersion>)> {
    let game = admin_service.get_game(game_id).await?;

    let mut file_data: Option<Vec<u8>> = None;
    let mut version: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::BadRequest(format!("Failed to read multipart field: {}", e))
    })? {
        match field.name() {
            Some("file") => {
                file_data = Some(field.bytes().await.map_err(|e| {
                    AppError::BadRequest(format!("Failed to read file data: {}", e))
                })?.to_vec());
            }
            Some("version") => {
                version = Some(field.text().await.map_err(|e| {
                    AppError::BadRequest(format!("Failed to read version: {}", e))
                })?);
            }
            _ => {}
        }
    }

    let file_data = file_data.ok_or_else(|| {
        AppError::BadRequest("No file provided".to_string())
    })?;

    let version_str = version.ok_or_else(|| {
        AppError::BadRequest("Version is required".to_string())
    })?;

    if file_data.len() < 100 {
        return Err(AppError::BadRequest("File too small".to_string()));
    }

    // Validate version format (X.Y.Z)
    if !version_str.split('.').all(|part| part.parse::<u32>().is_ok()) {
        return Err(AppError::BadRequest("Version must be in format X.Y.Z (e.g., 1.0.0)".to_string()));
    }

    // GCS path: {GameName}/{version}/game.zip
    let gcs_folder = format!("{}/{}", game.name, version_str);
    let gcs_path = format!("{}/game.zip", gcs_folder);

    // Upload using resumable upload for large files
    gcs_service
        .upload_file_resumable(
            &gcs_path,
            "application/zip",
            file_data,
        )
        .await?;

    // Create database record with folder path (not the zip path)
    // Arceus will list files in this folder after cloud function extracts the ZIP
    let game_version = admin_service
        .create_game_version(game_id, &version_str, &gcs_folder)
        .await?;

    Ok((StatusCode::CREATED, Json(game_version)))
}
