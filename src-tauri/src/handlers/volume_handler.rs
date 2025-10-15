
use super::traits::MessageHandler;
use crate::core::{Result, VolumeInfo};
use crate::network::DeviceConnection;
use crate::protocol::{MessageType, PacketReader};
use async_trait::async_trait;
use bytes::Bytes;
use std::sync::Arc;

pub struct VolumeHandler;

impl VolumeHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MessageHandler for VolumeHandler {
    fn message_type(&self) -> MessageType {
        MessageType::VolumeStatus
    }

    async fn handle(&self, device: &Arc<DeviceConnection>, payload: Bytes) -> Result<()> {
        let mut reader = PacketReader::new(payload);

        let level = reader.read_u8()?;
        let volume_info = VolumeInfo::new(level);
        device.update_volume(volume_info);

        tracing::trace!("Device {} volume: {}%", device.serial(), level);

        Ok(())
    }
}

impl Default for VolumeHandler {
    fn default() -> Self {
        Self::new()
    }
}
