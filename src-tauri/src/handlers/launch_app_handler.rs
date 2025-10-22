use super::traits::PacketHandler;
use crate::core::{CommandResult, Result};
use crate::network::DeviceConnection;
use crate::protocol::client_packet::LaunchAppResponse;
use crate::protocol::ClientPacket;
use async_trait::async_trait;
use std::sync::Arc;

/// Handler for LaunchAppResponse packets (opcode 0x10)
pub struct LaunchAppHandler;

impl LaunchAppHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for LaunchAppHandler {
    fn name(&self) -> &'static str {
        "LaunchAppHandler"
    }

    fn handles_packet(&self, packet: &ClientPacket) -> bool {
        matches!(packet, ClientPacket::LaunchAppResponse(_))
    }

    async fn handle(&self, device: &Arc<DeviceConnection>, packet: ClientPacket) -> Result<()> {
        if let ClientPacket::LaunchAppResponse(LaunchAppResponse { success, message }) = packet {
            let result = if success {
                CommandResult::success("Launch App", message)
            } else {
                CommandResult::failure("Launch App", message)
            };

            device.add_command_result(result);

            tracing::debug!(
                device = %device.serial(),
                success = success,
                "Launch app response received"
            );
        }
        Ok(())
    }
}

impl Default for LaunchAppHandler {
    fn default() -> Self {
        Self::new()
    }
}
