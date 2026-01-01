use crate::{api::handlers, services::{AdminService, ArcadeService, GcsService, SnorlaxService}};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

pub fn create_api_router(
    arcade_service: Arc<ArcadeService>,
    gcs_service: Arc<GcsService>,
    snorlax_service: Arc<SnorlaxService>,
    admin_service: Arc<AdminService>,
) -> Router {
    // Arcade endpoints
    let arcade_router = Router::new()
        .route("/arcade/config", get(handlers::get_arcade_config))
        .route("/arcade/games", get(handlers::get_arcade_games))
        .route(
            "/arcade/games/{game_id}/status",
            post(handlers::update_game_status),
        )
        .with_state(arcade_service.clone());

    // Game download endpoint
    let game_download_router = Router::new()
        .route(
            "/arcade/games/{game_id}/download",
            get(handlers::get_game_download_urls),
        )
        .with_state((arcade_service.clone(), gcs_service.clone()));

    // Snorlax endpoint
    let snorlax_router = Router::new()
        .route(
            "/arcade/snorlax/latest",
            get(handlers::get_snorlax_latest),
        )
        .with_state((arcade_service, snorlax_service.clone()));

    let admin_router = Router::new()
        .route("/admin/arcades",
            post(handlers::create_arcade)
                .get(handlers::list_arcades))
        .route("/admin/arcades/{id}",
            get(handlers::get_arcade)
                .put(handlers::update_arcade)
                .delete(handlers::delete_arcade))
        .route("/admin/arcades/{id}/assignments", get(handlers::get_arcade_assignments))
        .route("/admin/games",
            post(handlers::create_game))
        .route("/admin/games/{id}",
            get(handlers::get_game)
                .put(handlers::update_game)
                .delete(handlers::delete_game))
        .route("/admin/games/{game_id}/versions",
            post(handlers::create_game_version)
                .get(handlers::list_game_versions))
        .route("/admin/games/{game_id}/versions/{version_id}",
            get(handlers::get_game_version)
                .put(handlers::update_game_version))
        // Assignment management
        .route("/admin/assignments",
            post(handlers::create_assignment)
                .get(handlers::list_assignments))
        .route("/admin/assignments/{id}",
            put(handlers::update_assignment)
                .delete(handlers::delete_assignment))
        .with_state(admin_service.clone());

    // Game endpoints that require GCS service
    let game_gcs_router = Router::new()
        .route("/admin/games", get(handlers::list_games))
        .route("/admin/games/{id}/background", post(handlers::upload_game_background))
        .route("/admin/games/{game_id}/versions/upload", post(handlers::upload_game_version))
        .route("/admin/games/{game_id}/versions/{version_id}", delete(handlers::delete_game_version))
        .with_state((admin_service, gcs_service.clone()));

    // Snorlax admin endpoints
    let snorlax_admin_router = Router::new()
        .route("/admin/snorlax/versions",
            get(handlers::list_snorlax_versions)
                .post(handlers::create_snorlax_version))
        .route("/admin/snorlax/versions/{id}/set-current", put(handlers::set_current_snorlax_version))
        .route("/admin/snorlax/versions/{id}", delete(handlers::delete_snorlax_version))
        .with_state(snorlax_service.clone());

    // Snorlax upload endpoint (requires both snorlax and GCS services)
    let snorlax_upload_router = Router::new()
        .route("/admin/snorlax/upload", post(handlers::upload_snorlax_apk))
        .with_state((snorlax_service, gcs_service));

    // Merge routers
    arcade_router
        .merge(game_download_router)
        .merge(snorlax_router)
        .merge(admin_router)
        .merge(game_gcs_router)
        .merge(snorlax_admin_router)
        .merge(snorlax_upload_router)
}
