use std::io;
use thiserror::Error;
use crate::domain::services::CommandError;
use crate::domain::repositories::RepositoryError;
use crate::application::services::ApplicationError;

pub type Result<T> = std::result::Result<T, ArceusError>;

#[derive(Error, Debug)]
pub enum ArceusError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error(transparent)]
    Protocol(#[from] ProtocolError),

    #[error(transparent)]
    Network(#[from] NetworkError),

    #[error(transparent)]
    Storage(#[from] StorageError),

    #[error(transparent)]
    Handler(#[from] HandlerError),

    #[error(transparent)]
    Service(#[from] ServiceError),

    #[error(transparent)]
    Command(#[from] CommandError),

    #[error(transparent)]
    Repository(#[from] RepositoryError),

    #[error(transparent)]
    Application(#[from] ApplicationError),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("{0}")]
    Other(String),
}

impl ArceusError {
    /// Get a user friendly error message suitable for display
    pub fn user_message(&self) -> String {
        match self {
            Self::Io(_) => "A system I/O error occurred. Please try again.".to_string(),
            Self::Protocol(e) => e.user_message(),
            Self::Network(e) => e.user_message(),
            Self::Storage(e) => e.user_message(),
            Self::Handler(e) => e.user_message(),
            Self::Service(e) => e.user_message(),
            Self::Command(e) => format!("Command error: {}", e),
            Self::Repository(e) => format!("Repository error: {}", e),
            Self::Application(e) => format!("Application error: {}", e),
            Self::Config(msg) => format!("Configuration error: {}", msg),
            Self::Other(msg) => msg.clone(),
        }
    }

    /// Determine if this error is retriable
    pub fn is_retriable(&self) -> bool {
        match self {
            Self::Network(NetworkError::Timeout) => true,
            Self::Network(NetworkError::ConnectionClosed) => true,
            Self::Network(NetworkError::SendFailed(_)) => true,
            Self::Network(NetworkError::ReceiveFailed(_)) => true,
            Self::Command(CommandError::DeviceDisconnected { .. }) => false,
            Self::Command(CommandError::DeviceNotFound { .. }) => false,
            Self::Command(CommandError::SessionNotFound { .. }) => false,
            Self::Repository(RepositoryError::NotFound { .. }) => false,
            Self::Repository(RepositoryError::CapacityExceeded { .. }) => false,
            Self::Storage(StorageError::Database(_)) => true,
            Self::Handler(HandlerError::ResponseTimeout) => true,
            _ => false,
        }
    }

    /// Get a machine readable error code
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::Io(_) => "IO_ERROR",
            Self::Protocol(_) => "PROTOCOL_ERROR",
            Self::Network(NetworkError::Timeout) => "NETWORK_TIMEOUT",
            Self::Network(NetworkError::ConnectionClosed) => "CONNECTION_CLOSED",
            Self::Network(NetworkError::MaxConnectionsReached(_)) => "MAX_CONNECTIONS",
            Self::Network(_) => "NETWORK_ERROR",
            Self::Storage(_) => "STORAGE_ERROR",
            Self::Handler(_) => "HANDLER_ERROR",
            Self::Service(_) => "SERVICE_ERROR",
            Self::Command(CommandError::DeviceNotFound { .. }) => "DEVICE_NOT_FOUND",
            Self::Command(CommandError::DeviceDisconnected { .. }) => "DEVICE_DISCONNECTED",
            Self::Command(CommandError::SessionNotFound { .. }) => "SESSION_NOT_FOUND",
            Self::Command(CommandError::ValidationFailed(_)) => "VALIDATION_FAILED",
            Self::Command(_) => "COMMAND_ERROR",
            Self::Repository(RepositoryError::NotFound { .. }) => "NOT_FOUND",
            Self::Repository(RepositoryError::CapacityExceeded { .. }) => "CAPACITY_EXCEEDED",
            Self::Repository(_) => "REPOSITORY_ERROR",
            Self::Application(_) => "APPLICATION_ERROR",
            Self::Config(_) => "CONFIG_ERROR",
            Self::Other(_) => "UNKNOWN_ERROR",
        }
    }

    /// Get the severity level for logging
    pub fn severity(&self) -> tracing::Level {
        match self {
            Self::Command(CommandError::DeviceNotFound { .. }) => tracing::Level::WARN,
            Self::Command(CommandError::DeviceDisconnected { .. }) => tracing::Level::WARN,
            Self::Network(NetworkError::ConnectionClosed) => tracing::Level::INFO,
            Self::Repository(RepositoryError::NotFound { .. }) => tracing::Level::WARN,
            Self::Config(_) => tracing::Level::ERROR,
            _ => tracing::Level::ERROR,
        }
    }
}

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Invalid message type: {0:#x}")]
    InvalidMessageType(u8),

    #[error("Insufficient data: expected {expected} bytes, got {actual}")]
    InsufficientData { expected: usize, actual: usize },

    #[error("Invalid string encoding: {0}")]
    InvalidEncoding(String),

    #[error("Malformed packet: {0}")]
    MalformedPacket(String),

    #[error("Unsupported protocol version: {0}")]
    UnsupportedVersion(u8),
}

impl ProtocolError {
    pub fn user_message(&self) -> String {
        match self {
            Self::InvalidMessageType(_) => {
                "Received an invalid message from the device.".to_string()
            }
            Self::InsufficientData { .. } => {
                "Incomplete data received from the device.".to_string()
            }
            Self::InvalidEncoding(_) => {
                "Device sent data in an unreadable format.".to_string()
            }
            Self::MalformedPacket(_) => {
                "Received a malformed packet from the device.".to_string()
            }
            Self::UnsupportedVersion(v) => {
                format!("Device is using an unsupported protocol version ({}). Please update the device software.", v)
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Connection timeout")]
    Timeout,

    #[error("Connection closed")]
    ConnectionClosed,

    #[error("Device not found: {0}")]
    DeviceNotFound(String),

    #[error("Maximum connections reached: {0}")]
    MaxConnectionsReached(usize),

    #[error("Send failed: {0}")]
    SendFailed(String),

    #[error("Receive failed: {0}")]
    ReceiveFailed(String),

    #[error("Bind error: {0}")]
    BindError(String),
}

impl NetworkError {
    pub fn user_message(&self) -> String {
        match self {
            Self::ConnectionFailed(reason) => {
                format!("Failed to connect to device: {}", reason)
            }
            Self::Timeout => {
                "Connection to device timed out. Please check the device's network connection.".to_string()
            }
            Self::ConnectionClosed => {
                "Connection to device was closed.".to_string()
            }
            Self::DeviceNotFound(id) => {
                format!("Device '{}' not found. It may have disconnected.", id)
            }
            Self::MaxConnectionsReached(max) => {
                format!("Maximum number of device connections ({}) has been reached. Please disconnect some devices first.", max)
            }
            Self::SendFailed(reason) => {
                format!("Failed to send data to device: {}", reason)
            }
            Self::ReceiveFailed(reason) => {
                format!("Failed to receive data from device: {}", reason)
            }
            Self::BindError(reason) => {
                format!("Failed to start server: {}. The port may already be in use.", reason)
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Write failed: {0}")]
    WriteFailed(String),
}

impl StorageError {
    pub fn user_message(&self) -> String {
        match self {
            Self::Database(reason) => {
                format!("Database error: {}. Please restart the application.", reason)
            }
            Self::Serialization(reason) => {
                format!("Failed to save data: {}", reason)
            }
            Self::Deserialization(reason) => {
                format!("Failed to load data: {}", reason)
            }
            Self::KeyNotFound(key) => {
                format!("Data '{}' not found in storage.", key)
            }
            Self::WriteFailed(reason) => {
                format!("Failed to write to storage: {}", reason)
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("Unknown message type: {0:#x}")]
    UnknownMessageType(u8),

    #[error("Handler not registered for message type: {0:#x}")]
    HandlerNotRegistered(u8),

    #[error("Invalid payload: {0}")]
    InvalidPayload(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Timeout waiting for response")]
    ResponseTimeout,
}

impl HandlerError {
    pub fn user_message(&self) -> String {
        match self {
            Self::UnknownMessageType(opcode) => {
                format!("Received unknown message type (0x{:02X}) from device.", opcode)
            }
            Self::HandlerNotRegistered(opcode) => {
                format!("No handler registered for message type 0x{:02X}.", opcode)
            }
            Self::InvalidPayload(reason) => {
                format!("Device sent invalid data: {}", reason)
            }
            Self::ExecutionFailed(reason) => {
                format!("Failed to process device message: {}", reason)
            }
            Self::ResponseTimeout => {
                "Device did not respond within the expected time.".to_string()
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Device service error: {0}")]
    Device(String),

    #[error("APK service error: {0}")]
    Apk(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

impl ServiceError {
    pub fn user_message(&self) -> String {
        match self {
            Self::Device(reason) => {
                format!("Device operation failed: {}", reason)
            }
            Self::Apk(reason) => {
                format!("APK operation failed: {}", reason)
            }
            Self::InvalidOperation(reason) => {
                format!("Invalid operation: {}", reason)
            }
            Self::ResourceNotFound(resource) => {
                format!("'{}' was not found.", resource)
            }
            Self::PermissionDenied(reason) => {
                format!("Permission denied: {}", reason)
            }
        }
    }
}

impl From<ArceusError> for String {
    fn from(err: ArceusError) -> Self {
        err.to_string()
    }
}

impl serde::Serialize for ArceusError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

