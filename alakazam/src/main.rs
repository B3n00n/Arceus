mod api;
mod config;
mod db;
mod error;
mod models;
mod repositories;
mod routes;
mod services;

use config::Config;
use repositories::{ArcadeRepository, GameRepository};
use services::ArcadeService;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "alakazam=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;

    // Create database pool
    let pool = db::create_pool(&config.database.url).await?;

    // Initialize repositories
    let arcade_repo = ArcadeRepository::new(pool.clone());
    let game_repo = GameRepository::new(pool.clone());

    // Initialize services
    let arcade_service = Arc::new(ArcadeService::new(arcade_repo, game_repo));

    // Build application router
    let app = axum::Router::new()
        .merge(routes::create_router())
        .nest("/api", api::create_api_router(arcade_service))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Alakazam server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
