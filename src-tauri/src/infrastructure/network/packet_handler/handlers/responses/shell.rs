/// Shell execution response handler

use crate::app::EventBus;
use crate::application::dto::CommandResultDto;
use crate::domain::models::DeviceId;
use crate::net::io::ProtocolReadExt;
use async_trait::async_trait;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use std::sync::Arc;

use super::super::super::{PacketHandler, Result};

/// Handles SHELL_EXECUTION_RESPONSE (0x11) packets
/// Payload: [success: u8][output: String][exit_code: i32]
pub struct ShellExecutionResponseHandler {
    event_bus: Arc<EventBus>,
}

impl ShellExecutionResponseHandler {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait]
impl PacketHandler for ShellExecutionResponseHandler {
    fn opcode(&self) -> u8 {
        crate::infrastructure::protocol::opcodes::SHELL_EXECUTION_RESPONSE
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);
        let success = cursor.read_u8()? != 0;
        let output = cursor.read_string()?;
        let exit_code = cursor.read_i32::<BigEndian>()?;

        tracing::debug!(
            device_id = %device_id,
            success = success,
            exit_code = exit_code,
            "Shell execution response"
        );

        let result = if success {
            CommandResultDto::success("shell_execution", output)
        } else {
            CommandResultDto::failure("shell_execution", output)
        };
        self.event_bus.command_executed(device_id.as_uuid().clone(), result);

        Ok(())
    }
}
