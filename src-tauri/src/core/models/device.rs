use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

use super::{BatteryInfo, CommandResult, VolumeInfo};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub id: Uuid,
    pub model: String,
    pub serial: String,
    pub ip: String,
    pub connected_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub custom_name: Option<String>,
}

impl DeviceInfo {
    pub fn with_id(id: Uuid, model: String, serial: String, ip: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            model,
            serial,
            ip,
            connected_at: now,
            last_seen: now,
            custom_name: None,
        }
    }

    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    }

    pub fn set_custom_name(&mut self, name: Option<String>) {
        self.custom_name = name;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceState {
    pub info: DeviceInfo,
    pub battery: Option<BatteryInfo>,
    pub volume: Option<VolumeInfo>,
    pub command_history: VecDeque<CommandResult>,
    pub is_connected: bool,
}

impl DeviceState {
    pub fn new(info: DeviceInfo) -> Self {
        Self {
            info,
            battery: None,
            volume: None,
            command_history: VecDeque::new(),
            is_connected: true,
        }
    }

    pub fn add_command_result(&mut self, result: CommandResult) {
        const MAX_HISTORY_SIZE: usize = 50;

        self.command_history.push_front(result);
        if self.command_history.len() > MAX_HISTORY_SIZE {
            self.command_history.pop_back();
        }
    }

    pub fn update_battery(&mut self, battery: BatteryInfo) {
        self.battery = Some(battery);
    }

    pub fn update_volume(&mut self, volume: VolumeInfo) {
        self.volume = Some(volume);
    }

    pub fn mark_disconnected(&mut self) {
        self.is_connected = false;
    }
}
