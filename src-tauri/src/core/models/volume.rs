use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VolumeInfo {
    pub level: u8,
    pub last_updated: DateTime<Utc>,
}

impl VolumeInfo {
    pub fn new(level: u8) -> Self {
        Self {
            level: level.min(100),
            last_updated: Utc::now(),
        }
    }
}
