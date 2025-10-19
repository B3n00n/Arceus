use crate::core::{DeviceState, Result};
use crate::network::ConnectionManager;
use crate::storage::DeviceNamesStore;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeviceService {
    connection_manager: Arc<ConnectionManager>,
    device_names_store: Arc<DeviceNamesStore>,
}

impl DeviceService {
    pub fn new(
        connection_manager: Arc<ConnectionManager>,
        device_names_store: Arc<DeviceNamesStore>,
    ) -> Self {
        Self {
            connection_manager,
            device_names_store,
        }
    }

    pub fn get_all_devices(&self) -> Vec<DeviceState> {
        let mut devices = self.connection_manager.get_all_states();

        for device in &mut devices {
            if let Some(name) = self.device_names_store.get_name(&device.info.serial) {
                device.info.custom_name = Some(name);
            }
        }

        devices
    }

    pub fn get_device(&self, id: Uuid) -> Option<DeviceState> {
        let device = self.connection_manager.get(id)?;
        let mut state = device.get_state();

        if let Some(name) = self.device_names_store.get_name(&state.info.serial) {
            state.info.custom_name = Some(name);
        }

        Some(state)
    }

    pub fn set_device_name(&self, serial: String, name: Option<String>) -> Result<()> {
        if let Some(ref n) = name {
            self.device_names_store.set_name(serial.clone(), n.clone())?;
        } else {
            self.device_names_store.remove_name(&serial)?;
        }

        if let Some(device) = self.connection_manager.get_by_serial(&serial) {
            device.set_custom_name(name);
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
