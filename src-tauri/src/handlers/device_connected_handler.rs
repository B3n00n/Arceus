use super::traits::MessageHandler;
use crate::core::Result;
use crate::network::DeviceConnection;
use crate::protocol::{MessageType, PacketReader};
use async_trait::async_trait;
use bytes::Bytes;
use std::sync::Arc;

pub struct DeviceConnectedHandler;

impl DeviceConnectedHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MessageHandler for DeviceConnectedHandler {
    fn message_type(&self) -> MessageType {
        MessageType::DeviceConnected
    }

    async fn handle(&self, device: &Arc<DeviceConnection>, payload: Bytes) -> Result<()> {
        let mut reader = PacketReader::new(payload);
        let model = reader.read_string()?;
        let serial = reader.read_string()?;
        device.update_device_info(model, serial);

        tracing::info!("Device connected: {} ({})", device.serial(), device.id());

        Ok(())
    }
}

impl Default for DeviceConnectedHandler {
    fn default() -> Self {
        Self::new()
    }
}
