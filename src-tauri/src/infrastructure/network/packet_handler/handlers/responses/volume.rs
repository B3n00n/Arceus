/// Volume set response handler

use crate::app::EventBus;
use crate::application::dto::{CommandResultDto, VolumeInfoDto};
use crate::domain::models::{DeviceId, Volume};
use crate::domain::repositories::DeviceRepository;
use crate::net::io::ProtocolReadExt;
use async_trait::async_trait;
use byteorder::ReadBytesExt;
use std::io::Cursor;
use std::sync::Arc;

use super::super::super::{PacketHandler, Result};

/// Handles VOLUME_SET_RESPONSE (0x16) packets
/// The client sends back the actual volume percentage achieved after setting
pub struct VolumeSetResponseHandler {
    device_repo: Arc<dyn DeviceRepository>,
    event_bus: Arc<EventBus>,
}

impl VolumeSetResponseHandler {
    pub fn new(device_repo: Arc<dyn DeviceRepository>, event_bus: Arc<EventBus>) -> Self {
        Self { device_repo, event_bus }
    }
}

#[async_trait]
impl PacketHandler for VolumeSetResponseHandler {
    fn opcode(&self) -> u8 {
        crate::infrastructure::protocol::opcodes::VOLUME_SET_RESPONSE
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);
        let success = cursor.read_u8()? != 0;
        let message = cursor.read_string()?;

        tracing::debug!(device_id = %device_id, success, message = %message, "Volume set response");

        if success {
            if let Some(actual_percentage) = message
                .split_whitespace()
                .find_map(|word| word.trim_end_matches('%').parse::<u8>().ok())
            {
                if let Ok(Some(device)) = self.device_repo.find_by_id(device_id).await {
                    let (current, max) = if let Some(volume) = device.volume() {
                        let max = volume.max();
                        let current = (actual_percentage as u16 * max as u16 / 100) as u8;
                        (current, max)
                    } else {
                        let max = 15u8;
                        let current = (actual_percentage as u16 * max as u16 / 100) as u8;
                        (current, max)
                    };

                    if let Ok(volume) = Volume::new(current, max) {
                        if let Ok(Some(device)) = self.device_repo.find_by_id(device_id).await {
                            let updated_device = device.as_ref().clone().with_volume(volume);
                            let _ = self.device_repo.save(updated_device).await;
                        }

                        let volume_info = VolumeInfoDto::new(
                            actual_percentage,
                            current,
                            max,
                        );
                        self.event_bus.volume_updated(device_id.as_uuid().clone(), volume_info);
                    }
                }
            }
        }

        let result = if success {
            CommandResultDto::success("volume_set", &message)
        } else {
            CommandResultDto::failure("volume_set", &message)
        };
        self.event_bus.command_executed(device_id.as_uuid().clone(), result);

        Ok(())
    }
}
