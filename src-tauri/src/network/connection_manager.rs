use super::device::DeviceConnection;
use crate::core::{error::NetworkError, DeviceState, Result};
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

/// Manages active TCP connections to devices
///
/// Responsibilities:
/// - Track active connections by UUID
/// - Enforce connection limits
/// - Provide connection lookup by UUID
pub struct ConnectionManager {
    devices: Arc<DashMap<Uuid, Arc<DeviceConnection>>>,
    max_connections: usize,
}

impl ConnectionManager {
    pub fn new(max_connections: usize) -> Self {
        Self {
            devices: Arc::new(DashMap::new()),
            max_connections,
        }
    }

    pub fn connection_count(&self) -> usize {
        self.devices.len()
    }

    pub fn is_full(&self) -> bool {
        self.connection_count() >= self.max_connections
    }

    pub fn register(&self, device: Arc<DeviceConnection>) -> Result<()> {
        if self.is_full() {
            return Err(NetworkError::MaxConnectionsReached(self.max_connections).into());
        }

        let id = device.id();
        self.devices.insert(id, Arc::clone(&device));

        tracing::info!(
            "Registered connection {}, total connections: {}",
            id,
            self.connection_count()
        );

        Ok(())
    }

    pub fn unregister(&self, id: Uuid) {
        if let Some((_, device)) = self.devices.remove(&id) {
            device.mark_disconnected();

            tracing::info!(
                "Unregistered connection {}, remaining connections: {}",
                id,
                self.connection_count()
            );
        }
    }

    pub fn get(&self, id: Uuid) -> Option<Arc<DeviceConnection>> {
        self.devices.get(&id).map(|entry| Arc::clone(entry.value()))
    }

    pub fn get_all(&self) -> Vec<Arc<DeviceConnection>> {
        self.devices
            .iter()
            .map(|entry| Arc::clone(entry.value()))
            .collect()
    }

    pub fn get_all_states(&self) -> Vec<DeviceState> {
        self.devices
            .iter()
            .map(|entry| entry.value().get_state())
            .collect()
    }

    /// Finds a device by its serial (MAC address)
    /// Note: This performs a linear search through all connections
    pub fn find_by_serial(&self, serial: &str) -> Option<Arc<DeviceConnection>> {
        self.devices
            .iter()
            .find(|entry| entry.value().serial() == serial)
            .map(|entry| Arc::clone(entry.value()))
    }
}

