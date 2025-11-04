/// In-memory device repository implementation
///
/// Uses DashMap for thread-safe, lock-free access with dual indexing.

use crate::domain::models::{Device, DeviceId, Serial};
use crate::domain::repositories::{DeviceRepository, RepositoryError};
use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;

/// In-memory implementation of DeviceRepository
///
/// Features:
/// - O(1) lookup by device ID
/// - O(1) lookup by serial number (secondary index)
/// - Thread-safe with DashMap (lock-free reads)
/// - Configurable capacity limit
pub struct InMemoryDeviceRepository {
    /// Primary index: device_id -> Device
    by_id: Arc<DashMap<DeviceId, Device>>,
    /// Secondary index: serial -> device_id (for O(1) serial lookups)
    by_serial: Arc<DashMap<Serial, DeviceId>>,
    /// Maximum number of devices allowed
    max_capacity: usize,
}

impl InMemoryDeviceRepository {
    /// Create a new repository with default capacity (100 devices)
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    /// Create a new repository with a specific capacity
    pub fn with_capacity(max_capacity: usize) -> Self {
        Self {
            by_id: Arc::new(DashMap::new()),
            by_serial: Arc::new(DashMap::new()),
            max_capacity,
        }
    }

    /// Check if the repository is at capacity
    fn is_at_capacity(&self) -> bool {
        self.by_id.len() >= self.max_capacity
    }
}

impl Default for InMemoryDeviceRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DeviceRepository for InMemoryDeviceRepository {
    async fn find_by_id(&self, id: DeviceId) -> Result<Option<Device>, RepositoryError> {
        Ok(self.by_id.get(&id).map(|entry| entry.value().clone()))
    }

    async fn find_by_serial(&self, serial: &Serial) -> Result<Option<Device>, RepositoryError> {
        // O(1) lookup using secondary index
        if let Some(device_id_ref) = self.by_serial.get(serial) {
            let device_id = *device_id_ref.value();
            drop(device_id_ref); // Release the lock before next lookup
            Ok(self.by_id.get(&device_id).map(|entry| entry.value().clone()))
        } else {
            Ok(None)
        }
    }

    async fn find_all(&self) -> Result<Vec<Device>, RepositoryError> {
        Ok(self
            .by_id
            .iter()
            .map(|entry| entry.value().clone())
            .collect())
    }

    async fn save(&self, device: Device) -> Result<(), RepositoryError> {
        let id = device.id();
        let serial = device.serial().clone();

        // Check capacity only for new devices
        if !self.by_id.contains_key(&id) && self.is_at_capacity() {
            return Err(RepositoryError::CapacityExceeded {
                current: self.by_id.len(),
                max: self.max_capacity,
            });
        }

        // If updating an existing device, clean up old serial index if serial changed
        if let Some(existing) = self.by_id.get(&id) {
            let old_serial = existing.serial();
            if old_serial != &serial {
                self.by_serial.remove(old_serial);
            }
        }

        // Update primary index
        self.by_id.insert(id, device);

        // Update secondary index
        self.by_serial.insert(serial, id);

        Ok(())
    }

    async fn remove(&self, id: DeviceId) -> Result<(), RepositoryError> {
        if let Some((_, device)) = self.by_id.remove(&id) {
            // Clean up secondary index
            self.by_serial.remove(device.serial());
        }

        Ok(())
    }

    async fn count(&self) -> Result<usize, RepositoryError> {
        Ok(self.by_id.len())
    }
}
