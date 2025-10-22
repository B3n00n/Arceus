use super::traits::PacketHandler;
use crate::core::{CommandResult, Result};
use crate::network::DeviceConnection;
use crate::protocol::client_packet::VolumeSetResponse;
use crate::protocol::ClientPacket;
use async_trait::async_trait;
use std::sync::Arc;

/// Handler for VolumeSetResponse packets (opcode 0x17)
pub struct VolumeSetHandler;

impl VolumeSetHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for VolumeSetHandler {
    fn name(&self) -> &'static str {
        "VolumeSetHandler"
    }

    fn handles_packet(&self, packet: &ClientPacket) -> bool {
        matches!(packet, ClientPacket::VolumeSetResponse(_))
    }

    async fn handle(&self, device: &Arc<DeviceConnection>, packet: ClientPacket) -> Result<()> {
        if let ClientPacket::VolumeSetResponse(VolumeSetResponse {
            success,
            actual_level,
        }) = packet
        {
            let message = format!("Volume set to {}%", actual_level);
            let result = if success {
                CommandResult::success("Set Volume", message)
            } else {
                CommandResult::failure("Set Volume", message)
            };

            device.add_command_result(result);

            tracing::debug!(
                device = %device.serial(),
                success = success,
                level = actual_level,
                "Volume set response received"
            );
        }
        Ok(())
    }
}

impl Default for VolumeSetHandler {
    fn default() -> Self {
        Self::new()
    }
}
