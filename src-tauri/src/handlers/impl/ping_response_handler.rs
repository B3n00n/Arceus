use crate::core::error::Result;
use crate::core::CommandResult;
use crate::handlers::PacketHandler;
use crate::network::DeviceConnection;
use crate::protocol::opcodes;
use async_trait::async_trait;
use byteorder::ReadBytesExt;
use std::io::{Read, Write};
use std::sync::Arc;

/// Handles PingResponse (0x13) from client
/// Payload: [timestamp: u64]
pub struct PingResponseHandler;

impl PingResponseHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for PingResponseHandler {
    fn opcode(&self) -> u8 {
        opcodes::PING_RESPONSE
    }

    async fn handle(
        &self,
        device: &Arc<DeviceConnection>,
        src: &mut (dyn Read + Send),
        _dst: &mut (dyn Write + Send),
    ) -> Result<()> {
        let timestamp = src.read_u64::<byteorder::BigEndian>()?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let latency = now.saturating_sub(timestamp);

        tracing::info!("Ping response: {}ms latency", latency);
        device.add_command_result(CommandResult::success("ping", format!("Latency: {}ms", latency)));

        Ok(())
    }
}
