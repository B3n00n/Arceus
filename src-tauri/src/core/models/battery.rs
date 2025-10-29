use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BatteryInfo {
    pub headset_level: u8,
    pub is_charging: bool,
    pub last_updated: DateTime<Utc>,
}

impl BatteryInfo {
    pub fn new(headset_level: u8, is_charging: bool) -> Self {
        Self {
            headset_level: headset_level.min(100),
            is_charging,
            last_updated: Utc::now(),
        }
    }
}
