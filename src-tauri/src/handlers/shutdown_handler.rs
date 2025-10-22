use super::traits::PacketHandler;
use crate::core::{CommandResult, Result};
use crate::network::DeviceConnection;
use crate::protocol::ClientPacket;
use async_trait::async_trait;
use std::sync::Arc;

/// Handler for ShutdownResponse packets (opcode 0x16)
pub struct ShutdownHandler;

impl ShutdownHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for ShutdownHandler {
    fn name(&self) -> &'static str {
        "ShutdownHandler"
    }

    fn handles_packet(&self, packet: &ClientPacket) -> bool {
        matches!(packet, ClientPacket::ShutdownResponse(_))
    }

    async fn handle(&self, device: &Arc<DeviceConnection>, packet: ClientPacket) -> Result<()> {
        if let ClientPacket::ShutdownResponse(_) = packet {
            let result = CommandResult::success("Shutdown", "Device shutdown command sent");
            device.add_command_result(result);

            tracing::debug!(device = %device.serial(), "Shutdown response received");
        }
        Ok(())
    }
}

impl Default for ShutdownHandler {
    fn default() -> Self {
        Self::new()
    }
}
