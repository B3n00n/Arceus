use crate::domain::models::Serial;
use async_trait::async_trait;

use super::error::RepositoryError;

pub type Result<T> = std::result::Result<T, RepositoryError>;

/// Repository for managing custom device names
/// Device names are persisted separately from device state to ensure
/// they survive across device disconnections and application restarts.
#[async_trait]
pub trait DeviceNameRepository: Send + Sync {
    /// Get the custom name for a device by serial number
    /// Returns `None` if no custom name is set for this device.
    async fn get_name(&self, serial: &Serial) -> Result<Option<String>>;

    /// Set a custom name for a device
    /// If `name` is `None`, the custom name will be cleared.
    async fn set_name(&self, serial: &Serial, name: Option<String>) -> Result<()>;
}
