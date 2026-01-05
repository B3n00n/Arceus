/// Repository error types
///
/// Common errors that can occur in repository operations.

use crate::domain::models::DeviceId;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Device with ID {device_id} not found")]
    DeviceNotFound { device_id: DeviceId },

    #[error("Device with serial {serial} not found")]
    DeviceNotFoundBySerial { serial: String },

    #[error("Item not found: {item}")]
    NotFound { item: String },

    #[error("Repository capacity exceeded: current={current}, max={max}")]
    CapacityExceeded { current: usize, max: usize },

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("I/O error: {0}")]
    IoError(String),

    #[error("Repository operation failed: {0}")]
    OperationFailed(String),
}

impl From<std::io::Error> for RepositoryError {
    fn from(err: std::io::Error) -> Self {
        RepositoryError::IoError(err.to_string())
    }
}

impl From<sqlx::Error> for RepositoryError {
    fn from(err: sqlx::Error) -> Self {
        RepositoryError::DatabaseError(err.to_string())
    }
}

impl From<serde_json::Error> for RepositoryError {
    fn from(err: serde_json::Error) -> Self {
        RepositoryError::SerializationError(err.to_string())
    }
}
