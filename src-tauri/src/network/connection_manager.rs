use super::device::DeviceConnection;
use crate::core::{error::NetworkError, DeviceState, Result};
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct ConnectionManager {
    devices: Arc<DashMap<Uuid, Arc<DeviceConnection>>>,
    serials: Arc<DashMap<String, Uuid>>,
    max_connections: usize,
}

impl ConnectionManager {
    pub fn new(max_connections: usize) -> Self {
        Self {
            devices: Arc::new(DashMap::new()),
            serials: Arc::new(DashMap::new()),
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
        let serial = device.serial();

        self.devices.insert(id, Arc::clone(&device));
        self.serials.insert(serial.clone(), id);

        tracing::info!(
            "Registered device {} ({}), total connections: {}",
            serial,
            id,
            self.connection_count()
        );

        Ok(())
    }

    pub fn unregister(&self, id: Uuid) {
        if let Some((_, device)) = self.devices.remove(&id) {
            let serial = device.serial();
            self.serials.remove(&serial);

            device.mark_disconnected();

            tracing::info!(
                "Unregistered device {} ({}), remaining connections: {}",
                serial,
                id,
                self.connection_count()
            );
        }
    }

    pub fn get(&self, id: Uuid) -> Option<Arc<DeviceConnection>> {
        self.devices.get(&id).map(|entry| Arc::clone(entry.value()))
    }

    pub fn get_by_serial(&self, serial: &str) -> Option<Arc<DeviceConnection>> {
        self.serials
            .get(serial)
            .and_then(|entry| self.get(*entry.value()))
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
}

