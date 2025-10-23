use crate::core::error::Result;
use crate::core::CommandResult;
use crate::handlers::PacketHandler;
use crate::net::ProtocolReadExt;
use crate::network::DeviceConnection;
use crate::protocol::opcodes;
use async_trait::async_trait;
use byteorder::ReadBytesExt;
use std::io::{Read, Write};
use std::sync::Arc;

/// Handles LaunchAppResponse (0x10) from client
/// Payload: [success: bool][message: String]
pub struct LaunchAppResponseHandler;

impl LaunchAppResponseHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for LaunchAppResponseHandler {
    fn opcode(&self) -> u8 {
        opcodes::LAUNCH_APP_RESPONSE
    }

    async fn handle(
        &self,
        device: &Arc<DeviceConnection>,
        mut src: &mut (dyn Read + Send),
        mut _dst: &mut (dyn Write + Send),
    ) -> Result<()> {
        let success = src.read_u8()? != 0;
        let message = src.read_string()?;

        if success {
            tracing::info!("Launch app succeeded: {}", message);
            device.add_command_result(CommandResult::success("launch_app", message));
        } else {
            tracing::warn!("Launch app failed: {}", message);
            device.add_command_result(CommandResult::failure("launch_app", message));
        }

        Ok(())
    }
}
