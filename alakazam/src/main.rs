mod config;
mod domain;
mod error;

use config::Config;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "alakazam=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Alakazam Central Server");

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!(
        "Configuration loaded for {} environment",
        config.environment.as_str()
    );

    // TODO: Initialize database connection pool
    // let pool = sqlx::postgres::PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&config.database_url)
    //     .await?;

    // TODO: Run migrations
    // sqlx::migrate!("./migrations").run(&pool).await?;

    // TODO: Build application state
    // let state = AppState { pool, config };

    // TODO: Build router with routes and middleware
    // let app = Router::new()
    //     .route("/health", get(|| async { "OK" }))
    //     .with_state(state);

    let addr = config.server_addr();
    tracing::info!("Server will listen on {}", addr);

    // TODO: Start server
    // let listener = tokio::net::TcpListener::bind(&addr).await?;
    // axum::serve(listener, app).await?;

    tracing::info!("Alakazam foundation ready!");
    tracing::info!("Next steps:");
    tracing::info!("  1. Implement repository layer");
    tracing::info!("  2. Implement service layer");
    tracing::info!("  3. Implement API handlers");
    tracing::info!("  4. Wire up Axum router");

    Ok(())
}
