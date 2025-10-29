use crate::core::{DeviceState, Result};
use crate::network::{ConnectionManager, DeviceNameManager};
use crate::storage::DeviceNamesStore;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeviceService {
    connection_manager: Arc<ConnectionManager>,
    device_manager: Arc<DeviceNameManager>,
}

impl DeviceService {
    pub fn new(
        connection_manager: Arc<ConnectionManager>,
        device_names_store: Arc<DeviceNamesStore>,
    ) -> Self {
        let device_manager = Arc::new(DeviceNameManager::new(device_names_store));

        Self {
            connection_manager,
            device_manager,
        }
    }

    pub fn get_all_devices(&self) -> Vec<DeviceState> {
        let mut devices = self.connection_manager.get_all_states();

        for device in &mut devices {
            device.info.custom_name = self.device_manager.load_custom_name(&device.info.serial);
        }

        devices
    }

    pub fn get_device(&self, id: Uuid) -> Option<DeviceState> {
        let device = self.connection_manager.get(id)?;
        let mut state = device.get_state();

        state.info.custom_name = self.device_manager.load_custom_name(&state.info.serial);

        Some(state)
    }

    pub fn set_device_name(&self, serial: String, name: Option<String>) -> Result<()> {
        tracing::info!("Setting device name for serial {}: {:?}", serial, name);

        // Save to database via DeviceManager
        self.device_manager.set_custom_name(serial.clone(), name.clone())?;

        // Update the live device connection if it exists
        if let Some(device) = self.connection_manager.find_by_serial(&serial) {
            tracing::info!("Found device with serial {}, updating custom name", serial);
            device.set_custom_name(name);
        } else {
            tracing::warn!("Device with serial {} not found in connection manager", serial);
        }

        Ok(())
    }

    pub async fn launch_app(&self, device_ids: Vec<Uuid>, package_name: String) -> Result<()> {
        for id in device_ids {
            if let Some(device) = self.connection_manager.get(id) {
                device.launch_app(&package_name).await?;
            }
        }
        Ok(())
    }

    pub async fn uninstall_app(&self, device_ids: Vec<Uuid>, package_name: String) -> Result<()> {
        for id in device_ids {
            if let Some(device) = self.connection_manager.get(id) {
                device.uninstall_app(&package_name).await?;
            }
        }
        Ok(())
    }

    pub async fn request_battery(&self, device_ids: Vec<Uuid>) -> Result<()> {
        for id in device_ids {
            if let Some(device) = self.connection_manager.get(id) {
                device.request_battery().await?;
            }
        }
        Ok(())
    }

    pub async fn ping_devices(&self, device_ids: Vec<Uuid>) -> Result<()> {
        for id in device_ids {
            if let Some(device) = self.connection_manager.get(id) {
                device.ping().await?;
            }
        }
        Ok(())
    }

    pub async fn set_volume(&self, device_ids: Vec<Uuid>, level: u8) -> Result<()> {
        for id in device_ids {
            if let Some(device) = self.connection_manager.get(id) {
                device.set_volume(level).await?;
            }
        }
        Ok(())
    }

    pub async fn get_volume(&self, device_ids: Vec<Uuid>) -> Result<()> {
        for id in device_ids {
            if let Some(device) = self.connection_manager.get(id) {
                device.request_volume().await?;
            }
        }
        Ok(())
    }

    pub async fn execute_shell(&self, device_ids: Vec<Uuid>, command: String) -> Result<()> {
        for id in device_ids {
            if let Some(device) = self.connection_manager.get(id) {
                device.execute_shell(&command).await?;
            }
        }
        Ok(())
    }

    pub async fn get_installed_apps(&self, device_ids: Vec<Uuid>) -> Result<()> {
        for id in device_ids {
            if let Some(device) = self.connection_manager.get(id) {
                device.get_installed_apps().await?;
            }
        }
        Ok(())
    }

    pub async fn install_remote_apk(&self, device_ids: Vec<Uuid>, url: String) -> Result<()> {
        for id in device_ids {
            if let Some(device) = self.connection_manager.get(id) {
                device.install_remote_apk(&url).await?;
            }
        }
        Ok(())
    }

    pub async fn install_local_apk(&self, device_ids: Vec<Uuid>, url: String) -> Result<()> {
        for id in device_ids {
            if let Some(device) = self.connection_manager.get(id) {
                device.install_local_apk(&url).await?;
            }
        }
        Ok(())
    }

    pub async fn restart_devices(&self, device_ids: Vec<Uuid>) -> Result<()> {
        for id in device_ids {
            if let Some(device) = self.connection_manager.get(id) {
                device.restart().await?;
            }
        }
        Ok(())
    }
}
