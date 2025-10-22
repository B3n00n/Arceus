use super::traits::PacketHandler;
use crate::core::{CommandResult, Result};
use crate::network::DeviceConnection;
use crate::protocol::client_packet::PingResponse;
use crate::protocol::ClientPacket;
use async_trait::async_trait;
use std::sync::Arc;

/// Handler for PingResponse packets (opcode 0x13)
pub struct PingHandler;

impl PingHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for PingHandler {
    fn name(&self) -> &'static str {
        "PingHandler"
    }

    fn handles_packet(&self, packet: &ClientPacket) -> bool {
        matches!(packet, ClientPacket::PingResponse(_))
    }

    async fn handle(&self, device: &Arc<DeviceConnection>, packet: ClientPacket) -> Result<()> {
        if let ClientPacket::PingResponse(PingResponse { timestamp }) = packet {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            let latency_ms = now.saturating_sub(timestamp);

            let result = CommandResult::success("Ping", format!("Latency: {}ms", latency_ms));
            device.add_command_result(result);

            tracing::debug!(
                device = %device.serial(),
                latency_ms = latency_ms,
                "Ping response received"
            );
        }
        Ok(())
    }
}

impl Default for PingHandler {
    fn default() -> Self {
        Self::new()
    }
}
