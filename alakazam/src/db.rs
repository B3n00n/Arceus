use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;

pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    info!("Connecting to database...");

    let pool = PgPoolOptions::new()
        .connect(database_url)
        .await?;

    info!("Database connection established");

    Ok(pool)
}
