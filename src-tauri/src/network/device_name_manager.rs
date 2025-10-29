use crate::core::Result;
use crate::storage::DeviceNamesStore;
use std::sync::Arc;

pub struct DeviceNameManager {
    device_names_store: Arc<DeviceNamesStore>,
}

impl DeviceNameManager {
    pub fn new(device_names_store: Arc<DeviceNamesStore>) -> Self {
        Self {
            device_names_store,
        }
    }

    pub fn get_display_name(&self, serial: &str) -> String {
        self.device_names_store
            .get_name(serial)
            .unwrap_or_else(|| "Quest".to_string())
    }

    pub fn load_custom_name(&self, serial: &str) -> Option<String> {
        self.device_names_store.get_name(serial)
    }

    pub fn set_custom_name(&self, serial: String, name: Option<String>) -> Result<()> {
        if let Some(ref n) = name {
            self.device_names_store.set_name(serial.clone(), n.clone())?;
            tracing::info!("Set custom name for device {}: {}", serial, n);
        } else {
            self.device_names_store.remove_name(&serial)?;
            tracing::info!("Removed custom name for device {}", serial);
        }

        Ok(())
    }

    pub fn ensure_device_registered(&self, serial: &str) {
        if self.device_names_store.get_name(serial).is_none() {
            tracing::info!("New device detected with serial: {}", serial);
        }
    }
}
