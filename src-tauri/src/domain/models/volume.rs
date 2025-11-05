/// Volume entity
/// Represents the volume state of a device.

use serde::{Deserialize, Serialize};

/// Volume information for a device
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Volume {
    percentage: u8,
    current: u8,
    max: u8,
}

#[derive(Debug, thiserror::Error)]
pub enum VolumeError {
    #[error("Invalid volume range: current={current}, max={max}. Current must be <= max")]
    InvalidRange { current: u8, max: u8 },

    #[error("Invalid max volume: {0}. Must be greater than 0")]
    InvalidMax(u8),
}

impl Volume {
    /// Create a new Volume from raw levels
    /// This is the primary constructor for Volume when receiving data from devices.
    pub fn new(current: u8, max: u8) -> Result<Self, VolumeError> {
        if max == 0 {
            return Err(VolumeError::InvalidMax(max));
        }

        if current > max {
            return Err(VolumeError::InvalidRange { current, max });
        }

        let percentage = ((current as f64 / max as f64) * 100.0).round() as u8;

        Ok(Self {
            percentage,
            current,
            max,
        })
    }

    pub fn percentage(&self) -> u8 {
        self.percentage
    }

    pub fn current(&self) -> u8 {
        self.current
    }

    pub fn max(&self) -> u8 {
        self.max
    }
}
