use crate::core::models::{BatteryInfo, CommandResult, DeviceState, VolumeInfo};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ArceusEvent {
    #[serde(rename_all = "camelCase")]
    DeviceConnected {
        device: DeviceState,
    },

    #[serde(rename_all = "camelCase")]
    DeviceDisconnected {
        device_id: Uuid,
        serial: String,
    },

    #[serde(rename_all = "camelCase")]
    DeviceUpdated {
        device_id: Uuid,
    },

    #[serde(rename_all = "camelCase")]
    BatteryUpdated {
        device_id: Uuid,
        battery_info: BatteryInfo,
    },

    #[serde(rename_all = "camelCase")]
    VolumeUpdated {
        device_id: Uuid,
        volume_info: VolumeInfo,
    },

    #[serde(rename_all = "camelCase")]
    CommandExecuted {
        device_id: Uuid,
        result: CommandResult,
    },

    #[serde(rename_all = "camelCase")]
    InstalledAppsReceived {
        device_id: Uuid,
        apps: Vec<String>,
    },

    #[serde(rename_all = "camelCase")]
    DeviceNameChanged {
        device_id: Uuid,
        serial: String,
        new_name: Option<String>,
    },

    #[serde(rename_all = "camelCase")]
    ServerStarted {
        tcp_port: u16,
        http_port: u16,
    },

    ServerStopped,

    #[serde(rename_all = "camelCase")]
    HttpServerStarted {
        port: u16,
        url: String,
    },

    #[serde(rename_all = "camelCase")]
    Error {
        message: String,
        context: Option<String>,
    },

    #[serde(rename_all = "camelCase")]
    Info {
        message: String,
    },
}

#[derive(Clone)]
pub struct EventBus {
    app_handle: AppHandle,
}

impl EventBus {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    pub fn emit(&self, event: ArceusEvent) {
        let event_name = "arceus://event";

        if let Err(e) = self.app_handle.emit(event_name, &event) {
            tracing::error!("Failed to emit event {:?}: {}", event, e);
        } else {
            tracing::debug!("Emitted event: {:?}", event);
        }
    }

    pub fn device_connected(&self, device: DeviceState) {
        self.emit(ArceusEvent::DeviceConnected { device });
    }

    pub fn device_disconnected(&self, device_id: Uuid, serial: String) {
        self.emit(ArceusEvent::DeviceDisconnected { device_id, serial });
    }

    pub fn device_updated(&self, device_id: Uuid) {
        self.emit(ArceusEvent::DeviceUpdated { device_id });
    }

    pub fn battery_updated(&self, device_id: Uuid, battery_info: BatteryInfo) {
        self.emit(ArceusEvent::BatteryUpdated {
            device_id,
            battery_info,
        });
    }

    pub fn volume_updated(&self, device_id: Uuid, volume_info: VolumeInfo) {
        self.emit(ArceusEvent::VolumeUpdated {
            device_id,
            volume_info,
        });
    }

    pub fn command_executed(&self, device_id: Uuid, result: CommandResult) {
        self.emit(ArceusEvent::CommandExecuted { device_id, result });
    }

    pub fn installed_apps_received(&self, device_id: Uuid, apps: Vec<String>) {
        self.emit(ArceusEvent::InstalledAppsReceived { device_id, apps });
    }

    pub fn device_name_changed(&self, device_id: Uuid, serial: String, new_name: Option<String>) {
        self.emit(ArceusEvent::DeviceNameChanged {
            device_id,
            serial,
            new_name,
        });
    }

    pub fn server_started(&self, tcp_port: u16, http_port: u16) {
        self.emit(ArceusEvent::ServerStarted {
            tcp_port,
            http_port,
        });
    }

    pub fn server_stopped(&self) {
        self.emit(ArceusEvent::ServerStopped);
    }

    pub fn http_server_started(&self, port: u16, url: String) {
        self.emit(ArceusEvent::HttpServerStarted { port, url });
    }

    pub fn error(&self, message: impl Into<String>, context: Option<String>) {
        self.emit(ArceusEvent::Error {
            message: message.into(),
            context,
        });
    }

    pub fn info(&self, message: impl Into<String>) {
        self.emit(ArceusEvent::Info {
            message: message.into(),
        });
    }
}

