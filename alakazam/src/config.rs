use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub gcs: GcsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GcsConfig {
    pub bucket_name: String,
    pub service_account_path: String,
    pub signed_url_duration_secs: u32,
    pub snorlax_version: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Config {
            server: ServerConfig {
                host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: std::env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "43571".to_string())
                    .parse()?,
            },
            database: DatabaseConfig {
                url: std::env::var("DATABASE_URL")?,
            },
            gcs: GcsConfig {
                bucket_name: std::env::var("GCS_BUCKET_NAME")?,
                service_account_path: std::env::var("GCS_SERVICE_ACCOUNT_PATH")?,
                signed_url_duration_secs: std::env::var("GCS_SIGNED_URL_DURATION_SECS")
                    .unwrap_or_else(|_| "3600".to_string())
                    .parse()?,
                snorlax_version: std::env::var("SNORLAX_VERSION")
                    .unwrap_or_else(|_| "1.0.0".to_string()),
            },
        })
    }
}
