use super::traits::MessageHandler;
use crate::core::Result;
use crate::network::DeviceConnection;
use crate::protocol::MessageType;
use async_trait::async_trait;
use bytes::Bytes;
use std::sync::Arc;

pub struct HeartbeatHandler;

impl HeartbeatHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MessageHandler for HeartbeatHandler {
    fn message_type(&self) -> MessageType {
        MessageType::Heartbeat
    }

    async fn handle(&self, device: &Arc<DeviceConnection>, _payload: Bytes) -> Result<()> {
        tracing::trace!("Heartbeat from device {}", device.serial());
        Ok(())
    }
}

impl Default for HeartbeatHandler {
    fn default() -> Self {
        Self::new()
    }
}
