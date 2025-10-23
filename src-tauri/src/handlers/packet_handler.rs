use crate::core::error::Result;
use crate::network::DeviceConnection;
use async_trait::async_trait;
use std::io::{Read, Write};
use std::sync::Arc;

#[async_trait]
pub trait PacketHandler: Send + Sync {
    fn opcode(&self) -> u8;

    async fn handle(
        &self,
        device: &Arc<DeviceConnection>,
        src: &mut (dyn Read + Send),
        dst: &mut (dyn Write + Send),
    ) -> Result<()>;
}
