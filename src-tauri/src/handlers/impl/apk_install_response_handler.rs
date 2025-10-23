use crate::core::error::Result;
use crate::handlers::PacketHandler;
use crate::net::ProtocolReadExt;
use crate::network::DeviceConnection;
use crate::protocol::opcodes;
use async_trait::async_trait;
use byteorder::ReadBytesExt;
use std::io::{Read, Write};
use std::sync::Arc;

/// Handles ApkInstallResponse (0x14) from client
/// Payload: [success: bool][message: String]
pub struct ApkInstallResponseHandler;

impl ApkInstallResponseHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for ApkInstallResponseHandler {
    fn opcode(&self) -> u8 {
        opcodes::APK_INSTALL_RESPONSE
    }

    async fn handle(
        &self,
        _device: &Arc<DeviceConnection>,
        mut src: &mut (dyn Read + Send),
        mut _dst: &mut (dyn Write + Send),
    ) -> Result<()> {
        let success = src.read_u8()? != 0;
        let message = src.read_string()?;

        if success {
            tracing::info!("APK install succeeded: {}", message);
        } else {
            tracing::warn!("APK install failed: {}", message);
        }

        Ok(())
    }
}
