use super::traits::MessageHandler;
use crate::core::{CommandResult, Result};
use crate::network::DeviceConnection;
use crate::protocol::{MessageType, PacketReader};
use async_trait::async_trait;
use bytes::Bytes;
use std::sync::Arc;

pub struct ErrorHandler;

impl ErrorHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MessageHandler for ErrorHandler {
    fn message_type(&self) -> MessageType {
        MessageType::Error
    }

    async fn handle(&self, device: &Arc<DeviceConnection>, payload: Bytes) -> Result<()> {
        let mut reader = PacketReader::new(payload);

        let error_message = reader.read_string()?;
        tracing::warn!("Error from device {}: {}", device.serial(), error_message);
        let result = CommandResult::failure("DeviceError", error_message);
        device.add_command_result(result);

        Ok(())
    }
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}
