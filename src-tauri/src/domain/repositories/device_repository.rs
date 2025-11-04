/// Device repository trait
/// Abstraction for device persistence and retrieval.

use crate::domain::models::{Device, DeviceId, Serial};
use async_trait::async_trait;

use super::error::RepositoryError;

pub type Result<T> = std::result::Result<T, RepositoryError>;

/// Repository for managing device entities
/// This trait abstracts the storage mechanism for devices, allowing
/// different implementations (in-memory, Redis, PostgreSQL, etc.).
#[async_trait]
pub trait DeviceRepository: Send + Sync {
    /// Find a device by its unique ID
    /// Returns `None` if no device with the given ID exists.
    async fn find_by_id(&self, id: DeviceId) -> Result<Option<Device>>;

    /// Find a device by its serial number
    /// Returns `None` if no device with the given serial exists.
    async fn find_by_serial(&self, serial: &Serial) -> Result<Option<Device>>;

    /// Find all devices
    /// Returns all devices currently stored in the repository.
    async fn find_all(&self) -> Result<Vec<Device>>;

    /// Save or update a device
    /// If a device with the same ID already exists, it will be updated.
    /// Otherwise, a new device entry will be created.
    async fn save(&self, device: Device) -> Result<()>;

    /// Remove a device by ID
    /// Returns `Ok(())` even if the device doesn't exist (idempotent).
    async fn remove(&self, id: DeviceId) -> Result<()>;

    /// Count the number of devices
    /// Returns the total number of devices in the repository.
    async fn count(&self) -> Result<usize>;

    // Note: find_where is not included in the trait because it would make
    // the trait not object-safe (dyn DeviceRepository). Implementations can
    // provide this as an inherent method if needed.
}
