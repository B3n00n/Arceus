use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

    pub fn level_normalized(&self) -> f32 {
        self.headset_level as f32 / 100.0
    }

    pub fn is_low(&self) -> bool {
        self.headset_level < 20
    }

    pub fn is_critical(&self) -> bool {
        self.headset_level < 10
    }
}
