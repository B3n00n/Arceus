use super::traits::PacketHandler;
use crate::core::{CommandResult, Result};
use crate::network::DeviceConnection;
use crate::protocol::client_packet::UninstallAppResponse;
use crate::protocol::ClientPacket;
use async_trait::async_trait;
use std::sync::Arc;

/// Handler for UninstallAppResponse packets (opcode 0x15)
pub struct UninstallAppHandler;

impl UninstallAppHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for UninstallAppHandler {
    fn name(&self) -> &'static str {
        "UninstallAppHandler"
    }

    fn handles_packet(&self, packet: &ClientPacket) -> bool {
        matches!(packet, ClientPacket::UninstallAppResponse(_))
    }

    async fn handle(&self, device: &Arc<DeviceConnection>, packet: ClientPacket) -> Result<()> {
        if let ClientPacket::UninstallAppResponse(UninstallAppResponse { success, message }) =
            packet
        {
            let result = if success {
                CommandResult::success("Uninstall App", message)
            } else {
                CommandResult::failure("Uninstall App", message)
            };

            device.add_command_result(result);

            tracing::debug!(
                device = %device.serial(),
                success = success,
                "Uninstall app response received"
            );
        }
        Ok(())
    }
}

impl Default for UninstallAppHandler {
    fn default() -> Self {
        Self::new()
    }
}
