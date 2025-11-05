use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInfo {
    pub volume_percentage: u8,
    pub current_volume: u8,
    pub max_volume: u8,
}

impl VolumeInfo {
    pub fn new(volume_percentage: u8, current_volume: u8, max_volume: u8) -> Self {
        Self {
            volume_percentage: volume_percentage.min(100),
            current_volume,
            max_volume,
        }
    }
}
