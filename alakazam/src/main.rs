mod api;
mod config;
mod db;
mod error;
mod models;
mod repositories;
mod routes;
mod services;

use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderValue, Method};
use config::Config;
use repositories::{ArcadeRepository, ChannelRepository, CustomerRepository, GameRepository, GyrosRepository, SnorlaxRepository};
use services::{AdminService, ArcadeService, GcsService, GyrosService, SnorlaxService};
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
    let channel_repo = Arc::new(ChannelRepository::new(pool.clone()));
    let customer_repo = Arc::new(CustomerRepository::new(pool.clone()));
    let game_repo = Arc::new(GameRepository::new(pool.clone()));
    let snorlax_repo = Arc::new(SnorlaxRepository::new(pool.clone()));
    let gyros_repo = Arc::new(GyrosRepository::new(pool.clone()));

    // Initialize GCS service with Application Default Credentials
    let gcs_service = Arc::new(
        GcsService::new(
            config.gcs.bucket_name.clone(),
            config.gcs.signed_url_duration_secs,
        )
        .await?,
    );

    info!("GCS service initialized for bucket: {}", config.gcs.bucket_name);

    // Initialize services
    let arcade_service = Arc::new(ArcadeService::new(arcade_repo.clone(), game_repo.clone(), gcs_service.clone()));
    let snorlax_service = Arc::new(SnorlaxService::new(snorlax_repo.clone(), gcs_service.clone()));
    let gyros_service = Arc::new(GyrosService::new(gyros_repo.clone(), gcs_service.clone()));
    let admin_service = Arc::new(AdminService::new(arcade_repo.clone(), channel_repo.clone(), customer_repo.clone(), game_repo.clone()));

    // Configure CORS
    let allowed_origins: Vec<HeaderValue> = config.cors.allowed_origin
        .split(',')
        .map(|s| s.trim().parse::<HeaderValue>().expect("Invalid CORS origin"))
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
            axum::http::HeaderName::from_static("x-goog-authenticated-user-email"),
        ])
        .allow_credentials(true);

    info!("CORS configured for origins: {}", config.cors.allowed_origin);

    // Build application router
    let app = axum::Router::new()
        .merge(routes::create_router())
        .nest("/api", api::create_api_router(arcade_service, gcs_service, snorlax_service, gyros_service, admin_service))
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024 * 1024)) // 20 GB limit for file uploads
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    info!("Alakazam server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
