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

    #[error("Invalid machine ID")]
    InvalidMachineId,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Game not found")]
    GameNotFound,

    #[error("Game version not found")]
    GameVersionNotFound,

    #[error("Release channel not found")]
    ChannelNotFound,

    #[error("Customer not found")]
    CustomerNotFound,

    #[error("Cannot delete customer with assigned arcades")]
    CustomerHasArcades,

    #[error("Snorlax version not found")]
    SnorlaxVersionNotFound,

    #[error("No current Snorlax version set")]
    NoCurrentSnorlaxVersion,

    #[error("Gyros version not found")]
    GyrosVersionNotFound,

    #[error("No current Gyros version set")]
    NoCurrentGyrosVersion,

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::ArcadeNotFound => (StatusCode::NOT_FOUND, "Arcade not found".to_string()),
            AppError::InvalidMachineId => (StatusCode::UNAUTHORIZED, "Invalid machine ID".to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::GameNotFound => (StatusCode::NOT_FOUND, "Game not found".to_string()),
            AppError::GameVersionNotFound => (StatusCode::NOT_FOUND, "Game version not found".to_string()),
            AppError::ChannelNotFound => (StatusCode::NOT_FOUND, "Release channel not found".to_string()),
            AppError::CustomerNotFound => (StatusCode::NOT_FOUND, "Customer not found".to_string()),
            AppError::CustomerHasArcades => (StatusCode::CONFLICT, "Cannot delete customer with assigned arcades".to_string()),
            AppError::SnorlaxVersionNotFound => (StatusCode::NOT_FOUND, "Snorlax version not found".to_string()),
            AppError::NoCurrentSnorlaxVersion => (StatusCode::NOT_FOUND, "No current Snorlax version set".to_string()),
            AppError::GyrosVersionNotFound => (StatusCode::NOT_FOUND, "Gyros version not found".to_string()),
            AppError::NoCurrentGyrosVersion => (StatusCode::NOT_FOUND, "No current Gyros version set".to_string()),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let body = Json(json!({
            "error": message
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
