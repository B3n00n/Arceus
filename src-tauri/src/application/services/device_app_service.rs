/// Device Application Service
///
/// Orchestrates device operations using domain services and repositories.

use crate::domain::commands::{BatchResult, Command, CommandResponse};
use crate::domain::models::{Device, DeviceId, Serial};
use crate::domain::repositories::{DeviceNameRepository, DeviceRepository, RepositoryError};
use crate::domain::services::{CommandError, CommandExecutor};
use std::sync::Arc;

/// Result type for application service operations
pub type Result<T> = std::result::Result<T, ApplicationError>;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),

    #[error("Command error: {0}")]
    Command(#[from] CommandError),

    #[error("Device with serial {serial} not found")]
    DeviceNotFoundBySerial { serial: String },

    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

/// Application service for device operations
/// This service orchestrates device-related use cases by coordinating
/// between repositories, domain services, and command execution.
pub struct DeviceApplicationService {
    device_repo: Arc<dyn DeviceRepository>,
    device_name_repo: Arc<dyn DeviceNameRepository>,
    command_executor: Arc<CommandExecutor>,
}

impl DeviceApplicationService {
    pub fn new(
        device_repo: Arc<dyn DeviceRepository>,
        device_name_repo: Arc<dyn DeviceNameRepository>,
        command_executor: Arc<CommandExecutor>,
    ) -> Self {
        Self {
            device_repo,
            device_name_repo,
            command_executor,
        }
    }

    /// Get all devices
    pub async fn get_all_devices(&self) -> Result<Vec<Device>> {
        Ok(self.device_repo.find_all().await?)
    }

    /// Get a single device by ID
    pub async fn get_device(&self, id: DeviceId) -> Result<Option<Device>> {
        Ok(self.device_repo.find_by_id(id).await?)
    }

    /// Set a custom name for a device
    pub async fn set_device_name(&self, serial: Serial, name: Option<String>) -> Result<()> {
        if let Some(device) = self.device_repo.find_by_serial(&serial).await? {
            let updated_device = device.with_custom_name(name.clone());
            self.device_repo.save(updated_device).await?;
        }

        self.device_name_repo.set_name(&serial, name.clone()).await?;

        tracing::info!(
            serial = %serial,
            name = ?name,
            "Device name updated"
        );

        Ok(())
    }

    /// Execute a command on multiple devices (batch operation)
    pub async fn execute_command_batch(
        &self,
        device_ids: Vec<DeviceId>,
        command: Arc<dyn Command>,
    ) -> BatchResult<CommandResponse> {
        self.command_executor.execute_batch(device_ids, command).await
    }
}
