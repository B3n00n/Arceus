use crate::domain::models::DeviceId;
use crate::protocol::RawPacket;
use async_trait::async_trait;

/// Error type for session operations
#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Session not found for device {0}")]
    SessionNotFound(DeviceId),

    #[error("Failed to send packet: {0}")]
    SendError(String),
}

/// Trait for managing device sessions
/// This abstraction allows the domain layer to send packets to devices
/// without depending on infrastructure implementation details.
#[async_trait]
pub trait SessionManager: Send + Sync {
    /// Get a session handle for sending packets to a device
    async fn send_packet(&self, device_id: DeviceId, packet: RawPacket) -> Result<(), SessionError>;

    /// Check if a session exists for a device
    fn has_session(&self, device_id: &DeviceId) -> bool;
}
