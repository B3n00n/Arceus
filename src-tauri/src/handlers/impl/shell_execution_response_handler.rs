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

/// Handles ShellExecutionResponse (0x11) from client
/// Payload: [success: bool][output: String][exit_code: i32]
pub struct ShellExecutionResponseHandler;

impl ShellExecutionResponseHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for ShellExecutionResponseHandler {
    fn opcode(&self) -> u8 {
        opcodes::SHELL_EXECUTION_RESPONSE
    }

    async fn handle(
        &self,
        device: &Arc<DeviceConnection>,
        mut src: &mut (dyn Read + Send),
        mut _dst: &mut (dyn Write + Send),
    ) -> Result<()> {
        let success = src.read_u8()? != 0;
        let output = src.read_string()?;
        let exit_code = src.read_i32::<byteorder::BigEndian>()?;
        let device_name = device.display_name();

        tracing::info!("Shell execution (exit {}): {}", exit_code, if success { "success" } else { "failed" });
        tracing::debug!("Output: {}", output);

        let message = if output.len() > 100 {
            format!("{}: {}... (exit code: {})", device_name, &output[..100], exit_code)
        } else {
            format!("{}: {} (exit code: {})", device_name, output, exit_code)
        };

        if success {
            device.add_command_result(CommandResult::success("shell_execution", message));
        } else {
            device.add_command_result(CommandResult::failure("shell_execution", message));
        }

        Ok(())
    }
}
