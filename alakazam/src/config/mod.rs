use std::env;

use crate::error::AppError;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// Database connection URL
    pub database_url: String,

    /// Secret for signing client tokens
    pub token_secret: String,

    /// Server host (default: 0.0.0.0)
    pub server_host: String,

    /// Server port (default: 8080)
    pub server_port: u16,

    /// GCS project ID
    pub gcs_project_id: Option<String>,

    /// Signed URL expiration in seconds (default: 3600 = 1 hour)
    pub signed_url_expiration_secs: u64,

    /// Environment (development, staging, production)
    pub environment: Environment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Staging => "staging",
            Self::Production => "production",
        }
    }
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, AppError> {
        // Load .env file if it exists (for local development)
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| AppError::Config("DATABASE_URL must be set".into()))?;

        let token_secret = env::var("TOKEN_SECRET")
            .map_err(|_| AppError::Config("TOKEN_SECRET must be set".into()))?;

        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into());

        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .map_err(|_| AppError::Config("SERVER_PORT must be a valid port number".into()))?;

        let gcs_project_id = env::var("GCS_PROJECT_ID").ok();

        let signed_url_expiration_secs = env::var("SIGNED_URL_EXPIRATION_SECS")
            .unwrap_or_else(|_| "3600".into())
            .parse()
            .map_err(|_| {
                AppError::Config("SIGNED_URL_EXPIRATION_SECS must be a valid number".into())
            })?;

        let environment = match env::var("ENVIRONMENT")
            .unwrap_or_else(|_| "development".into())
            .to_lowercase()
            .as_str()
        {
            "production" | "prod" => Environment::Production,
            "staging" | "stage" => Environment::Staging,
            _ => Environment::Development,
        };

        Ok(Self {
            database_url,
            token_secret,
            server_host,
            server_port,
            gcs_project_id,
            signed_url_expiration_secs,
            environment,
        })
    }

    /// Get the server address as a string
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }

    /// Check if running in production
    pub fn is_production(&self) -> bool {
        self.environment == Environment::Production
    }
}
