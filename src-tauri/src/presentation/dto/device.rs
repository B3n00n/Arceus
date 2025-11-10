use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

use super::{BatteryInfoDto, CommandResultDto, VolumeInfoDto};

/// Device information DTO for frontend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfoDto {
    pub id: Uuid,
    pub model: String,
    pub serial: String,
    pub ip: String,
    pub connected_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub custom_name: Option<String>,
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
