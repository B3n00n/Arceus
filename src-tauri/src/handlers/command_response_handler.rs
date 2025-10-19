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

        // Check if this is an installed apps response (contains newline-separated package names)
        if success && message.contains('\n') && message.lines().all(|line| line.contains('.')) {
            // This looks like an installed apps list
            let apps: Vec<String> = message
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            if !apps.is_empty() {
                let app_count = apps.len();
                device.event_bus().installed_apps_received(device.id(), apps);
                tracing::debug!("Device {} returned {} installed apps", device.serial(), app_count);
                return Ok(());
            }
        }

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
