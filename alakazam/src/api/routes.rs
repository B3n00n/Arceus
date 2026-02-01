use crate::{api::handlers, services::{AdminService, ArcadeService, GcsService, GyrosService, SnorlaxService}};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

pub fn create_api_router(
    arcade_service: Arc<ArcadeService>,
    gcs_service: Arc<GcsService>,
    snorlax_service: Arc<SnorlaxService>,
    gyros_service: Arc<GyrosService>,
    admin_service: Arc<AdminService>,
) -> Router {
    // Arcade endpoints
    let arcade_router = Router::new()
        .route("/arcade/config", get(handlers::get_arcade_config))
        .route("/arcade/games", get(handlers::get_arcade_games))
        .with_state(arcade_service.clone());

    // Game download and status endpoints
    let game_download_router = Router::new()
        .route(
            "/arcade/games/{game_id}/download",
            get(handlers::get_game_download_urls),
        )
        .with_state((arcade_service.clone(), gcs_service.clone()));

    let game_status_router = Router::new()
        .route("/arcade/games/status", post(handlers::report_installations))
        .with_state(arcade_service.clone());

    // Snorlax endpoint
    let snorlax_router = Router::new()
        .route(
            "/arcade/snorlax/latest",
            get(handlers::get_snorlax_latest),
        )
        .with_state((arcade_service, snorlax_service.clone()));

    let admin_router = Router::new()
        // Arcade management
        .route("/admin/arcades",
            post(handlers::create_arcade)
                .get(handlers::list_arcades))
        .route("/admin/arcades/{id}",
            get(handlers::get_arcade)
                .put(handlers::update_arcade)
                .delete(handlers::delete_arcade))
        .route("/admin/arcades/{id}/channel", put(handlers::update_arcade_channel))
        // Release channel management
        .route("/admin/channels",
            post(handlers::create_channel)
                .get(handlers::list_channels))
        .route("/admin/channels/{id}",
            get(handlers::get_channel)
                .put(handlers::update_channel)
                .delete(handlers::delete_channel))
        // Game management
        .route("/admin/games",
            post(handlers::create_game))
        .route("/admin/games/{id}",
            get(handlers::get_game)
                .put(handlers::update_game)
                .delete(handlers::delete_game))
        // Game version management
        .route("/admin/games/{game_id}/versions",
            post(handlers::create_game_version)
                .get(handlers::list_game_versions))
        .route("/admin/games/{game_id}/versions/{version_id}",
            get(handlers::get_game_version)
                .put(handlers::update_game_version))
        .route("/admin/games/{game_id}/versions/{version_id}/publish",
            post(handlers::publish_version)
                .delete(handlers::unpublish_version))
        .with_state(admin_service.clone());

    // Game endpoints that require GCS service
    let game_gcs_router = Router::new()
        .route("/admin/games", get(handlers::list_games))
        .route("/admin/games/{game_id}/versions/{version_id}", delete(handlers::delete_game_version))
        .route("/admin/games/{game_id}/versions/generate-upload-url", post(handlers::generate_game_version_upload_url))
        .route("/admin/games/{game_id}/versions/generate-batch-upload-urls", post(handlers::generate_batch_upload_urls))
        .route("/admin/games/{game_id}/background/generate-upload-url", post(handlers::generate_background_upload_url))
        .with_state((admin_service.clone(), gcs_service.clone()));

    // Game version confirmation endpoint
    let game_confirm_router = Router::new()
        .route("/admin/games/{game_id}/versions/confirm-upload", post(handlers::confirm_game_version_upload))
        .with_state(admin_service);

    // Snorlax admin endpoints
    let snorlax_admin_router = Router::new()
        .route("/admin/snorlax/versions",
            get(handlers::list_snorlax_versions)
                .post(handlers::create_snorlax_version))
        .route("/admin/snorlax/versions/{id}/set-current", put(handlers::set_current_snorlax_version))
        .route("/admin/snorlax/versions/{id}", delete(handlers::delete_snorlax_version))
        .with_state(snorlax_service.clone());

    // Snorlax direct upload endpoints
    let snorlax_upload_router = Router::new()
        .route("/admin/snorlax/generate-upload-url", post(handlers::generate_snorlax_upload_url))
        .with_state(gcs_service.clone());

    let snorlax_confirm_router = Router::new()
        .route("/admin/snorlax/confirm-upload", post(handlers::confirm_snorlax_upload))
        .with_state(snorlax_service);

    // Gyros admin endpoints
    let gyros_admin_router = Router::new()
        .route("/admin/gyros/versions",
            get(handlers::list_gyros_versions)
                .post(handlers::create_gyros_version))
        .route("/admin/gyros/versions/{id}/set-current", put(handlers::set_current_gyros_version))
        .route("/admin/gyros/versions/{id}", delete(handlers::delete_gyros_version))
        .with_state(gyros_service.clone());

    // Gyros direct upload endpoints
    let gyros_upload_router = Router::new()
        .route("/admin/gyros/generate-upload-url", post(handlers::generate_gyros_upload_url))
        .with_state(gcs_service.clone());

    let gyros_confirm_router = Router::new()
        .route("/admin/gyros/confirm-upload", post(handlers::confirm_gyros_upload))
        .with_state(gyros_service);

    // Merge routers
    arcade_router
        .merge(game_download_router)
        .merge(game_status_router)
        .merge(snorlax_router)
        .merge(admin_router)
        .merge(game_gcs_router)
        .merge(game_confirm_router)
        .merge(snorlax_admin_router)
        .merge(snorlax_upload_router)
        .merge(snorlax_confirm_router)
        .merge(gyros_admin_router)
        .merge(gyros_upload_router)
        .merge(gyros_confirm_router)
}
