use super::traits::MessageHandler;
use crate::core::{CommandResult, Result};
use crate::network::DeviceConnection;
use crate::protocol::{MessageType, PacketReader};
use async_trait::async_trait;
use bytes::Bytes;
use std::sync::Arc;

pub struct CommandResponseHandler;

impl CommandResponseHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MessageHandler for CommandResponseHandler {
    fn message_type(&self) -> MessageType {
        MessageType::CommandResponse
    }

    async fn handle(&self, device: &Arc<DeviceConnection>, payload: Bytes) -> Result<()> {
        let mut reader = PacketReader::new(payload);
        let success = reader.read_bool()?;
        let message = reader.read_string()?;
        let result = if success {
            CommandResult::success("Command", message.clone())
        } else {
            CommandResult::failure("Command", message.clone())
        };

        device.add_command_result(result);

        tracing::debug!(
            "Device {} command {}: {}",
            device.serial(),
            if success { "succeeded" } else { "failed" },
            message
        );

        Ok(())
    }
}

impl Default for CommandResponseHandler {
    fn default() -> Self {
        Self::new()
    }
}
