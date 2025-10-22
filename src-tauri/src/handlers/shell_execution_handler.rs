use super::traits::PacketHandler;
use crate::core::{CommandResult, Result};
use crate::network::DeviceConnection;
use crate::protocol::client_packet::ShellExecutionResponse;
use crate::protocol::ClientPacket;
use async_trait::async_trait;
use std::sync::Arc;

/// Handler for ShellExecutionResponse packets (opcode 0x11)
pub struct ShellExecutionHandler;

impl ShellExecutionHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for ShellExecutionHandler {
    fn name(&self) -> &'static str {
        "ShellExecutionHandler"
    }

    fn handles_packet(&self, packet: &ClientPacket) -> bool {
        matches!(packet, ClientPacket::ShellExecutionResponse(_))
    }

    async fn handle(&self, device: &Arc<DeviceConnection>, packet: ClientPacket) -> Result<()> {
        if let ClientPacket::ShellExecutionResponse(ShellExecutionResponse {
            success,
            output,
            exit_code,
        }) = packet
        {
            let message = format!("{} (exit code: {})", output, exit_code);
            let result = if success {
                CommandResult::success("Shell Execution", message)
            } else {
                CommandResult::failure("Shell Execution", message)
            };

            device.add_command_result(result);

            tracing::debug!(
                device = %device.serial(),
                success = success,
                exit_code = exit_code,
                "Shell execution response received"
            );
        }
        Ok(())
    }
}

impl Default for ShellExecutionHandler {
    fn default() -> Self {
        Self::new()
    }
}
