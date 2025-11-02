/// Battery entity
/// Represents the battery state of a device.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Battery information for a device
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Battery {
    level: u8,
    is_charging: bool,
    last_updated: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum BatteryError {
    #[error("Invalid battery level: {0}. Must be between 0 and 100")]
    InvalidLevel(u8),
}

impl Battery {
    pub fn new(level: u8, is_charging: bool) -> Result<Self, BatteryError> {
        if level > 100 {
            return Err(BatteryError::InvalidLevel(level));
        }

        Ok(Self {
            level,
            is_charging,
            last_updated: Utc::now(),
        })
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn is_charging(&self) -> bool {
        self.is_charging
    }

    pub fn last_updated(&self) -> DateTime<Utc> {
        self.last_updated
    }
}
