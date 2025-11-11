/// Command Executor
/// Executes commands on devices.

use crate::domain::commands::{BatchResult, Command, CommandResponse};
use crate::domain::models::DeviceId;
use crate::domain::repositories::{DeviceRepository, RepositoryError};
use crate::domain::services::SessionManager;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, CommandError>;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Device {device_id} not found")]
    DeviceNotFound { device_id: DeviceId },

    #[error("Session not found for device {device_id}")]
    SessionNotFound { device_id: DeviceId },

    #[error("Command validation failed: {0}")]
    ValidationFailed(String),

    #[error("Command '{command}' failed on device {device_id}: {reason}")]
    ExecutionFailed {
        device_id: DeviceId,
        command: String,
        reason: String,
    },

    #[error("Command '{command}' timed out after {timeout_ms}ms on device {device_id}")]
    Timeout {
        device_id: DeviceId,
        command: String,
        timeout_ms: u64,
    },

    #[error("Repository error: {0}")]
    RepositoryError(#[from] RepositoryError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] std::io::Error),

    #[error("Network error: {reason}")]
    NetworkError { reason: String },

    #[error("Batch command failed: {succeeded} succeeded, {failed} failed out of {total} devices")]
    BatchPartialFailure {
        total: usize,
        succeeded: usize,
        failed: usize,
        command: String,
    },
}

/// Executes commands on devices
pub struct CommandExecutor {
    device_repo: Arc<dyn DeviceRepository>,
    session_manager: Arc<dyn SessionManager>,
}

impl CommandExecutor {
    pub fn new(
        device_repo: Arc<dyn DeviceRepository>,
        session_manager: Arc<dyn SessionManager>,
    ) -> Self {
        Self {
            device_repo,
            session_manager,
        }
    }

    /// Execute a command on a single device
    pub async fn execute_single(
        &self,
        device_id: DeviceId,
        cmd: Arc<dyn Command>,
    ) -> Result<CommandResponse> {
        if let Err(e) = cmd.validate() {
            return Err(CommandError::ValidationFailed(e));
        }

        // Execute command
        self.execute_internal(device_id, cmd).await
    }

    /// Execute a command on multiple devices in parallel
    pub async fn execute_batch(
        &self,
        device_ids: Vec<DeviceId>,
        cmd: Arc<dyn Command>,
    ) -> BatchResult<CommandResponse> {
        let mut result = BatchResult::new();

        if device_ids.is_empty() {
            return result;
        }

        // Execute in parallel (FuturesUnordered)
        let mut tasks = FuturesUnordered::new();

        for device_id in device_ids {
            let executor = self.clone_for_task();
            let cmd = Arc::clone(&cmd);

            tasks.push(async move {
                (device_id, executor.execute_single(device_id, cmd).await)
            });
        }

        // Collect results as they complete
        while let Some((device_id, res)) = tasks.next().await {
            match res {
                Ok(response) => result.add_success(device_id, response),
                Err(e) => result.add_failure(device_id, e.to_string()),
            }
        }

        result
    }

    /// Internal execution logic (override this in tests)
    async fn execute_internal(
        &self,
        device_id: DeviceId,
        cmd: Arc<dyn Command>,
    ) -> Result<CommandResponse> {
        // Verify device exists
        let _device = self
            .device_repo
            .find_by_id(device_id)
            .await?
            .ok_or(CommandError::DeviceNotFound { device_id })?;

        // Check if session exists (device is connected if session exists)
        if !self.session_manager.has_session(&device_id) {
            return Err(CommandError::SessionNotFound { device_id });
        }

        // Serialize command to packet
        let payload = cmd.serialize()?;
        let packet = crate::infrastructure::protocol::RawPacket {
            opcode: cmd.opcode(),
            payload,
        };

        // Send packet to device via session manager
        self.session_manager
            .send_packet(device_id, packet)
            .await
            .map_err(|e| CommandError::ExecutionFailed {
                device_id,
                command: cmd.name().to_string(),
                reason: e.to_string(),
            })?;

        tracing::debug!(
            device_id = %device_id,
            command = cmd.name(),
            "Command sent successfully"
        );

        // Return success - actual response will come via packet handler
        Ok(CommandResponse::Success)
    }

    /// Clone for parallel task execution
    fn clone_for_task(&self) -> Self {
        Self {
            device_repo: Arc::clone(&self.device_repo),
            session_manager: Arc::clone(&self.session_manager),
        }
    }
}
