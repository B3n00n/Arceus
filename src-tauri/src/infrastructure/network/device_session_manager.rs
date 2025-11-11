/// Session Manager
/// Manages active device sessions for command execution.

use crate::domain::models::DeviceId;
use crate::domain::services::SessionManager as SessionManagerTrait;
use crate::infrastructure::network::device_session::DeviceSession;
use crate::infrastructure::protocol::RawPacket;
use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;

/// Manages active device sessions
/// Provides O(1) lookup of sessions by device ID for command execution.
pub struct DeviceSessionManager {
    sessions: Arc<DashMap<DeviceId, Arc<DeviceSession>>>,
}

impl DeviceSessionManager {
    /// Create a new SessionManager
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
        }
    }

    /// Add a session
    pub fn add_session(&self, device_id: DeviceId, session: Arc<DeviceSession>) {
        self.sessions.insert(device_id, session);
        tracing::debug!(device_id = %device_id, "Session added to manager");
    }

    /// Remove a session
    pub fn remove_session(&self, device_id: &DeviceId) {
        self.sessions.remove(device_id);
        tracing::debug!(device_id = %device_id, "Session removed from manager");
    }

    /// Get a session by device ID
    pub fn get_session(&self, device_id: &DeviceId) -> Option<Arc<DeviceSession>> {
        self.sessions.get(device_id).map(|entry| Arc::clone(&entry.value()))
    }
}

impl Default for DeviceSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

// Implement the domain trait for infrastructure SessionManager
#[async_trait]
impl SessionManagerTrait for DeviceSessionManager {
    async fn send_packet(&self, device_id: DeviceId, packet: RawPacket) -> Result<(), crate::domain::services::SessionError> {
        let session = self.get_session(&device_id)
            .ok_or(crate::domain::services::SessionError::SessionNotFound(device_id))?;

        session.send_packet(packet)
            .await
            .map_err(|e| crate::domain::services::SessionError::SendError(e.to_string()))
    }

    fn has_session(&self, device_id: &DeviceId) -> bool {
        self.sessions.contains_key(device_id)
    }
}
