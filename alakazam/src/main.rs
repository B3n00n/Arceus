mod api;
mod config;
mod db;
mod error;
mod models;
mod repositories;
mod routes;
mod services;

use axum::extract::DefaultBodyLimit;
use config::Config;
use repositories::{ArcadeRepository, GameRepository, SnorlaxRepository};
use services::{AdminService, ArcadeService, GcsService, SnorlaxService};
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

    // Initialize repositories (Arc-wrapped for sharing between services)
    let arcade_repo = Arc::new(ArcadeRepository::new(pool.clone()));
    let game_repo = Arc::new(GameRepository::new(pool.clone()));
    let snorlax_repo = Arc::new(SnorlaxRepository::new(pool.clone()));

    // Initialize GCS service
    let gcs_service = Arc::new(
        GcsService::new(
            config.gcs.bucket_name.clone(),
            config.gcs.service_account_path.clone(),
            config.gcs.signed_url_duration_secs,
        )
        .await?,
    );

    info!("GCS service initialized for bucket: {}", config.gcs.bucket_name);

    // Initialize services
    let arcade_service = Arc::new(ArcadeService::new(arcade_repo.clone(), game_repo.clone(), gcs_service.clone()));
    let snorlax_service = Arc::new(SnorlaxService::new(snorlax_repo.clone(), gcs_service.clone()));
    let admin_service = Arc::new(AdminService::new(arcade_repo.clone(), game_repo.clone()));

    // Build application router
    let app = axum::Router::new()
        .merge(routes::create_router())
        .nest("/api", api::create_api_router(arcade_service, gcs_service, snorlax_service, admin_service))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024 * 1024)) // 20 GB limit for file uploads
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Alakazam server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
