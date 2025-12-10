use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use uuid::Uuid;

use super::{BatteryInfoDto, CommandResultDto, VolumeInfoDto};
use crate::domain::models::Device;

/// Device information DTO for frontend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfoDto {
    pub id: Uuid,
    pub model: String,
    pub serial: String,
    pub version: String,
    pub connected_at: DateTime<Utc>,
    pub custom_name: Option<String>,
    pub running_app: Option<String>,
}

/// Complete device state DTO for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceStateDto {
    pub info: DeviceInfoDto,
    pub battery: Option<BatteryInfoDto>,
    pub volume: Option<VolumeInfoDto>,
    pub command_history: VecDeque<CommandResultDto>,
}

impl From<&Arc<Device>> for DeviceStateDto {
    fn from(device: &Arc<Device>) -> Self {
        let info = DeviceInfoDto {
            id: device.id().as_uuid().clone(),
            model: device.model().to_string(),
            serial: device.serial().as_str().to_string(),
            version: device.version().to_string(),
            connected_at: device.connected_at(),
            custom_name: device.custom_name().map(|s| s.to_string()),
            running_app: device.running_app().map(|s| s.to_string()),
        };

        let battery = device.battery().map(|b| BatteryInfoDto {
            headset_level: b.level(),
            is_charging: b.is_charging(),
        });

        let volume = device.volume().map(|v| {
            VolumeInfoDto::new(
                v.percentage(),
                v.current(),
                v.max(),
            )
        });

        DeviceStateDto {
            info,
            battery,
            volume,
            command_history: VecDeque::new(),
        }
    }
}

impl From<Arc<Device>> for DeviceStateDto {
    fn from(device: Arc<Device>) -> Self {
        Self::from(&device)
    }
}
