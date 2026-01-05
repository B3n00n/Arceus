use crate::{api::handlers, services::ArcadeService};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn create_api_router(arcade_service: Arc<ArcadeService>) -> Router {
    Router::new()
        .route("/arcade/config", get(handlers::get_arcade_config))
        .route("/arcade/games", get(handlers::get_arcade_games))
        .route(
            "/arcade/games/{game_id}/status",
            post(handlers::update_game_status),
        )
        .with_state(arcade_service)
}
