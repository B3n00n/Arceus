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

/// Handles VolumeSetResponse (0x17) from client
/// Payload: [success: bool][message: String]
pub struct VolumeSetResponseHandler;

impl VolumeSetResponseHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for VolumeSetResponseHandler {
    fn opcode(&self) -> u8 {
        opcodes::VOLUME_SET_RESPONSE
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
            tracing::info!("Volume set successfully: {}", message);
            device.add_command_result(CommandResult::success("set_volume", message));
        } else {
            tracing::warn!("Volume set failed: {}", message);
            device.add_command_result(CommandResult::failure("set_volume", message));
        }

        Ok(())
    }
}
