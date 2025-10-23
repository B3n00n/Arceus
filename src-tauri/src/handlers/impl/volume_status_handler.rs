use crate::core::error::Result;
use crate::core::models::volume::VolumeInfo;
use crate::handlers::PacketHandler;
use crate::network::DeviceConnection;
use crate::protocol::opcodes;
use async_trait::async_trait;
use byteorder::ReadBytesExt;
use std::io::{Read, Write};
use std::sync::Arc;

/// Handles VolumeStatus message (0x04) from client
/// Payload: [percentage: u8][current: u8][max: u8]
pub struct VolumeStatusHandler;

impl VolumeStatusHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for VolumeStatusHandler {
    fn opcode(&self) -> u8 {
        opcodes::VOLUME_STATUS
    }

    async fn handle(
        &self,
        device: &Arc<DeviceConnection>,
        src: &mut (dyn Read + Send),
        _dst: &mut (dyn Write + Send),
    ) -> Result<()> {
        let percentage = src.read_u8()?;
        let current = src.read_u8()?;
        let max = src.read_u8()?;

        tracing::debug!("Volume: {}% ({}/{})", percentage, current, max);

        let volume = VolumeInfo::new(percentage, current, max);
        device.update_volume(volume);

        Ok(())
    }
}
