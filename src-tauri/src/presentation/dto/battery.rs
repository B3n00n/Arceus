use serde::{Deserialize, Serialize};

/// Battery information DTO for frontend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BatteryInfoDto {
    pub headset_level: u8,
    pub is_charging: bool,
}
