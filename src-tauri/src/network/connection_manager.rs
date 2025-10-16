use super::device::DeviceConnection;
use crate::core::{error::NetworkError, DeviceState, Result};
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ConnectionStats {
    pub total_connections: usize,
    pub max_connections: usize,
    pub utilization_percent: u32,
    pub is_full: bool,
}

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

    pub fn contains(&self, id: Uuid) -> bool {
        self.devices.contains_key(&id)
    }

    pub fn contains_serial(&self, serial: &str) -> bool {
        self.serials.contains_key(serial)
    }

    pub async fn for_each<F, Fut>(&self, f: F)
    where
        F: Fn(Arc<DeviceConnection>) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = ()> + Send,
    {
        let devices = self.get_all();
        let futures: Vec<_> = devices.into_iter().map(|device| f(device)).collect();
        futures::future::join_all(futures).await;
    }

    pub async fn for_each_by_ids<F, Fut>(&self, ids: &[Uuid], f: F)
    where
        F: Fn(Arc<DeviceConnection>) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = ()> + Send,
    {
        let futures: Vec<_> = ids
            .iter()
            .filter_map(|id| self.get(*id))
            .map(|device| f(device))
            .collect();
        futures::future::join_all(futures).await;
    }

    pub fn get_stats(&self) -> ConnectionStats {
        ConnectionStats {
            total_connections: self.connection_count(),
            max_connections: self.max_connections,
            utilization_percent: (self.connection_count() as f32 / self.max_connections as f32 * 100.0) as u32,
            is_full: self.is_full(),
        }
    }

    pub fn clear(&self) {
        let devices: Vec<_> = self.devices.iter().map(|e| *e.key()).collect();
        for id in devices {
            self.unregister(id);
        }
    }
}

