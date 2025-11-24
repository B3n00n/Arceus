/// Status update packet handlers (BATTERY_STATUS, VOLUME_STATUS)

use crate::app::EventBus;
use crate::application::dto::{BatteryInfoDto, VolumeInfoDto};
use crate::domain::models::{Battery, DeviceId, Volume};
use crate::domain::repositories::DeviceRepository;
use crate::infrastructure::protocol::opcodes;
use async_trait::async_trait;
use byteorder::ReadBytesExt;
use std::io::Cursor;
use std::sync::Arc;

use super::super::{PacketHandler, Result};

/// Handles BATTERY_STATUS (0x03) packets
/// Payload: [level: u8][is_charging: bool]
pub struct BatteryStatusHandler {
    device_repo: Arc<dyn DeviceRepository>,
    event_bus: Arc<EventBus>,
}

impl BatteryStatusHandler {
    pub fn new(device_repo: Arc<dyn DeviceRepository>, event_bus: Arc<EventBus>) -> Self {
        Self {
            device_repo,
            event_bus,
        }
    }
}

#[async_trait]
impl PacketHandler for BatteryStatusHandler {
    fn opcode(&self) -> u8 {
        opcodes::BATTERY_STATUS
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);

        let level = cursor.read_u8()?;
        let is_charging = cursor.read_u8()? != 0;

        tracing::debug!(
            device_id = %device_id,
            level = level,
            is_charging = is_charging,
            "Battery status received"
        );

        // Update device with battery info
        let battery = Battery::new(level, is_charging)
            .map_err(|e| crate::app::error::ArceusError::DomainValidation(format!("Invalid battery: {}", e)))?;

        if let Ok(Some(device)) = self.device_repo.find_by_id(device_id).await {
            let updated_device = device.as_ref().clone().with_battery(battery);
            self.device_repo.save(updated_device).await?;
        }

        // Emit event
        let battery_info = BatteryInfoDto {
            headset_level: level,
            is_charging,
        };
        self.event_bus.battery_updated(device_id.as_uuid().clone(), battery_info);

        Ok(())
    }
}

/// Handles VOLUME_STATUS (0x04) packets
/// Payload: [current: u8][max: u8]
pub struct VolumeStatusHandler {
    device_repo: Arc<dyn DeviceRepository>,
    event_bus: Arc<EventBus>,
}

impl VolumeStatusHandler {
    pub fn new(device_repo: Arc<dyn DeviceRepository>, event_bus: Arc<EventBus>) -> Self {
        Self {
            device_repo,
            event_bus,
        }
    }
}

#[async_trait]
impl PacketHandler for VolumeStatusHandler {
    fn opcode(&self) -> u8 {
        opcodes::VOLUME_STATUS
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);

        let first = cursor.read_u8()?;
        let second = cursor.read_u8()?;

        // Device sends [percentage(0-100)][max], but we need [current][max]
        // If first > second, assume device sent [percentage][max] format
        let (current, max) = if first > second && second > 0 {
            // Percentage format: calculate current from percentage
            let percentage = first;
            let max = second;
            let current = ((percentage as f32 / 100.0) * max as f32).round() as u8;
            (current, max)
        } else {
            // Assume [current][max] format
            (first, second)
        };

        tracing::debug!(
            device_id = %device_id,
            current = current,
            max = max,
            "Volume status received (parsed from first={}, second={})",
            first,
            second
        );

        // Update device with volume info
        let volume = Volume::new(current, max)
            .map_err(|e| crate::app::error::ArceusError::DomainValidation(format!("Invalid volume: {}", e)))?;

        if let Ok(Some(device)) = self.device_repo.find_by_id(device_id).await {
            let updated_device = device.as_ref().clone().with_volume(volume);
            self.device_repo.save(updated_device).await?;
        }

        // Emit event
        let percentage = ((current as f32 / max as f32) * 100.0) as u8;
        let volume_info = VolumeInfoDto::new(
            percentage,
            current,
            max,
        );
        self.event_bus.volume_updated(device_id.as_uuid().clone(), volume_info);

        Ok(())
    }
}
