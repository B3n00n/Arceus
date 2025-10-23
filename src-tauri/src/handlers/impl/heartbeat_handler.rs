use crate::core::error::Result;
use crate::handlers::PacketHandler;
use crate::network::DeviceConnection;
use crate::protocol::opcodes;
use async_trait::async_trait;
use std::io::{Read, Write};
use std::sync::Arc;

/// Handles Heartbeat message (0x02) from client
/// Payload: empty
pub struct HeartbeatHandler;

impl HeartbeatHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for HeartbeatHandler {
    fn opcode(&self) -> u8 {
        opcodes::HEARTBEAT
    }

    async fn handle(
        &self,
        device: &Arc<DeviceConnection>,
        mut _src: &mut (dyn Read + Send),
        mut _dst: &mut (dyn Write + Send),
    ) -> Result<()> {
        device.update_last_seen();
        tracing::debug!("Heartbeat from device {}", device.id());
        Ok(())
    }
}
