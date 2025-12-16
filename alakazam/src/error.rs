use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Arcade not found")]
    ArcadeNotFound,

    #[error("Invalid MAC address")]
    InvalidMacAddress,

    #[error("Game not found")]
    GameNotFound,

    #[error("Game version not found")]
    GameVersionNotFound,

    #[error("No assignment found for this arcade and game")]
    NoAssignment,

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::ArcadeNotFound => (StatusCode::NOT_FOUND, "Arcade not found"),
            AppError::InvalidMacAddress => (StatusCode::UNAUTHORIZED, "Invalid MAC address"),
            AppError::GameNotFound => (StatusCode::NOT_FOUND, "Game not found"),
            AppError::GameVersionNotFound => (StatusCode::NOT_FOUND, "Game version not found"),
            AppError::NoAssignment => (StatusCode::NOT_FOUND, "No assignment found"),
            AppError::Internal(ref msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        let body = Json(json!({
            "error": message,
            "details": self.to_string()
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
