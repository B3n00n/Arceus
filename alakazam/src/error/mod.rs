use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    // Authentication errors
    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Invalid or expired token")]
    InvalidToken,

    #[error("Missing authorization header")]
    MissingAuth,

    // Not found errors
    #[error("Client not found")]
    ClientNotFound,

    #[error("Game not found")]
    GameNotFound,

    #[error("Game version not found")]
    GameVersionNotFound,

    #[error("Assignment not found")]
    AssignmentNotFound,

    // Conflict errors
    #[error("Client with this name already exists")]
    ClientAlreadyExists,

    #[error("Game with this name already exists")]
    GameAlreadyExists,

    #[error("Game version already exists")]
    GameVersionAlreadyExists,

    #[error("Assignment already exists")]
    AssignmentAlreadyExists,

    // Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    // External service errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    // Internal errors
    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),

    #[error("Configuration error: {0}")]
    Config(String),
}

/// Error code for API responses
impl AppError {
    fn error_code(&self) -> &'static str {
        match self {
            Self::InvalidApiKey => "INVALID_API_KEY",
            Self::InvalidToken => "INVALID_TOKEN",
            Self::MissingAuth => "MISSING_AUTH",
            Self::ClientNotFound => "CLIENT_NOT_FOUND",
            Self::GameNotFound => "GAME_NOT_FOUND",
            Self::GameVersionNotFound => "GAME_VERSION_NOT_FOUND",
            Self::AssignmentNotFound => "ASSIGNMENT_NOT_FOUND",
            Self::ClientAlreadyExists => "CLIENT_ALREADY_EXISTS",
            Self::GameAlreadyExists => "GAME_ALREADY_EXISTS",
            Self::GameVersionAlreadyExists => "GAME_VERSION_ALREADY_EXISTS",
            Self::AssignmentAlreadyExists => "ASSIGNMENT_ALREADY_EXISTS",
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::InvalidInput(_) => "INVALID_INPUT",
            Self::Database(_) => "DATABASE_ERROR",
            Self::Storage(_) => "STORAGE_ERROR",
            Self::Jwt(_) => "JWT_ERROR",
            Self::Internal(_) => "INTERNAL_ERROR",
            Self::Config(_) => "CONFIG_ERROR",
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidApiKey | Self::InvalidToken | Self::MissingAuth => {
                StatusCode::UNAUTHORIZED
            }
            Self::ClientNotFound
            | Self::GameNotFound
            | Self::GameVersionNotFound
            | Self::AssignmentNotFound => StatusCode::NOT_FOUND,
            Self::ClientAlreadyExists
            | Self::GameAlreadyExists
            | Self::GameVersionAlreadyExists
            | Self::AssignmentAlreadyExists => StatusCode::CONFLICT,
            Self::Validation(_) | Self::InvalidInput(_) => StatusCode::BAD_REQUEST,
            Self::Database(_) | Self::Storage(_) | Self::Internal(_) | Self::Config(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Jwt(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

/// Convert AppError into HTTP Response
/// This is the SINGLE place where errors should be mapped to responses
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let code = self.error_code();
        let message = self.to_string();

        // Log internal errors with more detail
        match &self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
            }
            AppError::Internal(e) => {
                tracing::error!("Internal error: {:?}", e);
            }
            AppError::Storage(e) => {
                tracing::error!("Storage error: {}", e);
            }
            _ => {
                tracing::debug!("Request error: {} - {}", code, message);
            }
        }

        let body = Json(json!({
            "error": {
                "code": code,
                "message": message
            }
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
