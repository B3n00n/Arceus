use crate::{api::handlers, services::{ArcadeService, GcsService}};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn create_api_router(
    arcade_service: Arc<ArcadeService>,
    gcs_service: Arc<GcsService>,
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
        .with_state((arcade_service, gcs_service));

    // Merge routers
    arcade_router
        .merge(game_download_router)
        .merge(snorlax_router)
}
