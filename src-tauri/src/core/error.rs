use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ArceusError>;

#[derive(Error, Debug)]
pub enum ArceusError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Protocol error: {0}")]
    Protocol(#[from] ProtocolError),
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Handler error: {0}")]
    Handler(#[from] HandlerError),
    #[error("Service error: {0}")]
    Service(#[from] ServiceError),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("{0}")]
    Other(String),
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

