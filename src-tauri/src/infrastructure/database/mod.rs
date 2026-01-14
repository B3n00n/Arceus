use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions, SqliteSynchronous};
use std::path::Path;
use std::str::FromStr;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal); 

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        // Create tables if they don't exist
        Self::initialize_schema(&pool).await?;

        Ok(Self { pool })
    }

    async fn initialize_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        // Create device_names table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS device_names (
                serial TEXT PRIMARY KEY,
                custom_name TEXT NOT NULL
            )
            "#,
        )
        .execute(pool)
        .await?;

        // Create game_cache table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS game_cache (
                game_id INTEGER PRIMARY KEY,
                game_name TEXT NOT NULL,
                assigned_version_id INTEGER NOT NULL,
                assigned_version TEXT NOT NULL,
                installed_version_id INTEGER,
                installed_version TEXT,
                installed_at TEXT
            )
            "#,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
