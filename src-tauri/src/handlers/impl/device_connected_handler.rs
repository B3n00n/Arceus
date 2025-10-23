use crate::core::error::Result;
use crate::handlers::PacketHandler;
use crate::net::ProtocolReadExt;
use crate::network::DeviceConnection;
use crate::protocol::opcodes;
use async_trait::async_trait;
use std::io::{Read, Write};
use std::sync::Arc;

/// Handles DeviceConnected message (0x01) from client
/// Payload: [model: String][serial: String]
pub struct DeviceConnectedHandler;

impl DeviceConnectedHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for DeviceConnectedHandler {
    fn opcode(&self) -> u8 {
        opcodes::DEVICE_CONNECTED
    }

    async fn handle(
        &self,
        device: &Arc<DeviceConnection>,
        mut src: &mut (dyn Read + Send),
        mut _dst: &mut (dyn Write + Send),
    ) -> Result<()> {
        let model = src.read_string()?;
        let serial = src.read_string()?;

        tracing::info!("Device connected: {} ({})", model, serial);

        device.update_device_info(model, serial);

        Ok(())
    }
}
