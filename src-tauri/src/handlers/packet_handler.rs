use crate::core::error::Result;
use crate::network::DeviceConnection;
use async_trait::async_trait;
use std::io::{Read, Write};
use std::sync::Arc;

#[async_trait]
pub trait PacketHandler: Send + Sync {
    fn opcode(&self) -> u8;

    /// Handle a packet
    ///
    /// # Arguments
    /// * `device` - The device connection this packet came from
    /// * `src` - Reader positioned at the start of the packet payload (after opcode and length)
    /// * `dst` - Writer for the complete response packet (must write opcode + length + payload)
    ///
    /// # Returns
    /// Ok(()) if handled successfully, Err if there was an error
    async fn handle(
        &self,
        device: &Arc<DeviceConnection>,
        src: &mut (dyn Read + Send),
        dst: &mut (dyn Write + Send),
    ) -> Result<()>;
}
