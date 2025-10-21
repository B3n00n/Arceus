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

    /// Infer command type from response message content
    fn infer_command_type(message: &str) -> &'static str {
        let message_lower = message.to_lowercase();

        if message_lower.contains("launched") {
            "Launch App"
        } else if message_lower.contains("uninstalled") {
            "Uninstall App"
        } else if message_lower.contains("installed") {
            "Install App"
        } else if message.contains("Volume") {
            "Volume"
        } else if message_lower.contains("shutdown") {
            "Shutdown"
        } else if message_lower.contains("beep") || message_lower.contains("ping") {
            "Ping"
        } else {
            "Command"
        }
    }

    /// Check if a message looks like a package name list
    fn looks_like_package_list(message: &str) -> bool {
        if message.contains(',') {
            // Multiple packages: check all contain dots
            message.split(',').all(|pkg| pkg.trim().contains('.'))
        } else {
            // Single package: check it contains a dot and looks like a package name
            message.contains('.') && !message.contains(' ') && message.len() > 3
        }
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

        // Check if this is an installed apps response
        if success && Self::looks_like_package_list(&message) {
            // This looks like an installed apps list
            let apps: Vec<String> = message
                .split(',')
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

        // Determine command type from message content
        let command_type = Self::infer_command_type(&message);

        let result = if success {
            CommandResult::success(command_type, message.clone())
        } else {
            CommandResult::failure(command_type, message.clone())
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
