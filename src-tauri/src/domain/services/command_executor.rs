/// Command Executor
/// Executes commands on devices with middleware support.

use crate::domain::commands::{BatchResult, Command, CommandResponse};
use crate::domain::models::DeviceId;
use crate::domain::repositories::{DeviceRepository, RepositoryError};
use crate::domain::services::SessionManager;
use async_trait::async_trait;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::sync::Arc;
use std::time::Instant;

pub type Result<T> = std::result::Result<T, CommandError>;

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Device {device_id} not found")]
    DeviceNotFound { device_id: DeviceId },

    #[error("Device {device_id} is not connected (last seen: {last_seen})")]
    DeviceDisconnected {
        device_id: DeviceId,
        last_seen: String,
    },

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

/// Middleware for command execution
/// Middleware can intercept command execution to add cross-cutting concerns
/// like logging, metrics, validation, rate limiting, etc.
#[async_trait]
pub trait CommandMiddleware: Send + Sync {
    /// Called before command execution
    async fn before_execute(&self, device_id: DeviceId, cmd: &dyn Command) -> Result<()>;

    /// Called after command execution
    async fn after_execute(
        &self,
        device_id: DeviceId,
        cmd: &dyn Command,
        result: &Result<CommandResponse>,
        duration: std::time::Duration,
    ) -> Result<()>;
}

/// Executes commands on devices with middleware support
pub struct CommandExecutor {
    device_repo: Arc<dyn DeviceRepository>,
    session_manager: Arc<dyn SessionManager>,
    middleware: Vec<Arc<dyn CommandMiddleware>>,
}

impl CommandExecutor {
    pub fn new(
        device_repo: Arc<dyn DeviceRepository>,
        session_manager: Arc<dyn SessionManager>,
    ) -> Self {
        Self {
            device_repo,
            session_manager,
            middleware: Vec::new(),
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

        // Apply pre-execution middleware
        for mw in &self.middleware {
            mw.before_execute(device_id, cmd.as_ref()).await?;
        }

        let start = Instant::now();

        // Execute command
        let result = self.execute_internal(device_id, Arc::clone(&cmd)).await;

        let duration = start.elapsed();

        // Apply post-execution middleware
        for mw in &self.middleware {
            mw.after_execute(device_id, cmd.as_ref(), &result, duration).await?;
        }

        result
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
        let device = self
            .device_repo
            .find_by_id(device_id)
            .await?
            .ok_or(CommandError::DeviceNotFound { device_id })?;

        // Check if device is connected
        if !device.is_connected() {
            return Err(CommandError::DeviceDisconnected {
                device_id,
                last_seen: device.last_seen().to_rfc3339(),
            });
        }

        // Check if session exists
        if !self.session_manager.has_session(&device_id) {
            return Err(CommandError::SessionNotFound { device_id });
        }

        // Serialize command to packet
        let payload = cmd.serialize()?;
        let packet = crate::protocol::RawPacket {
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
            middleware: self.middleware.clone(),
        }
    }
}
