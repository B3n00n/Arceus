use crate::application::dto::{BatteryInfoDto, CommandResultDto, DeviceStateDto, VolumeInfoDto};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ArceusEvent {
    #[serde(rename_all = "camelCase")]
    DeviceConnected {
        device: DeviceStateDto,
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
        battery_info: BatteryInfoDto,
    },

    #[serde(rename_all = "camelCase")]
    VolumeUpdated {
        device_id: Uuid,
        volume_info: VolumeInfoDto,
    },

    #[serde(rename_all = "camelCase")]
    CommandExecuted {
        device_id: Uuid,
        result: CommandResultDto,
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

    #[serde(rename_all = "camelCase")]
    GameStarted {
        game_name: String,
        process_id: Option<u32>,
        content_server_url: String,
    },

    #[serde(rename_all = "camelCase")]
    GameStopped {
        game_name: String,
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
        }
    }

    pub fn device_connected(&self, device: DeviceStateDto) {
        self.emit(ArceusEvent::DeviceConnected { device });
    }

    pub fn battery_updated(&self, device_id: Uuid, battery_info: BatteryInfoDto) {
        self.emit(ArceusEvent::BatteryUpdated {
            device_id,
            battery_info,
        });
    }

    pub fn volume_updated(&self, device_id: Uuid, volume_info: VolumeInfoDto) {
        self.emit(ArceusEvent::VolumeUpdated {
            device_id,
            volume_info,
        });
    }

    pub fn command_executed(&self, device_id: Uuid, result: CommandResultDto) {
        self.emit(ArceusEvent::CommandExecuted { device_id, result });
    }

    pub fn installed_apps_received(&self, device_id: Uuid, apps: Vec<String>) {
        self.emit(ArceusEvent::InstalledAppsReceived { device_id, apps });
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

    pub fn game_started(&self, game_name: String, process_id: Option<u32>, content_server_url: String) {
        self.emit(ArceusEvent::GameStarted {
            game_name,
            process_id,
            content_server_url,
        });
    }

    pub fn game_stopped(&self, game_name: String) {
        self.emit(ArceusEvent::GameStopped { game_name });
    }
}

