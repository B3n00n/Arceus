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

/// Handles UninstallAppResponse (0x15) from client
/// Payload: [success: bool][message: String]
pub struct UninstallAppResponseHandler;

impl UninstallAppResponseHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for UninstallAppResponseHandler {
    fn opcode(&self) -> u8 {
        opcodes::UNINSTALL_APP_RESPONSE
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
            tracing::info!("Uninstall succeeded: {}", message);
            device.add_command_result(CommandResult::success("uninstall_app", message));
        } else {
            tracing::warn!("Uninstall failed: {}", message);
            device.add_command_result(CommandResult::failure("uninstall_app", message));
        }

        Ok(())
    }
}
