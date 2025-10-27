use crate::core::error::Result;
use crate::core::CommandResult;
use crate::handlers::PacketHandler;
use crate::net::ProtocolReadExt;
use crate::network::DeviceConnection;
use crate::protocol::opcodes;
use async_trait::async_trait;
use std::io::{Read, Write};
use std::sync::Arc;

/// Handles ApkDownloadStarted (0x17) from client
/// Payload: [filename_or_url: String]
/// Sent when the client starts downloading an APK
pub struct ApkDownloadStartedHandler;

impl ApkDownloadStartedHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for ApkDownloadStartedHandler {
    fn opcode(&self) -> u8 {
        opcodes::APK_DOWNLOAD_STARTED
    }

    async fn handle(
        &self,
        device: &Arc<DeviceConnection>,
        mut src: &mut (dyn Read + Send),
        mut _dst: &mut (dyn Write + Send),
    ) -> Result<()> {
        let filename_or_url = src.read_string()?;

        tracing::info!("APK download started: {}", filename_or_url);
        device.add_command_result(CommandResult::success(
            "install_apk",
            format!("Download started: {}", filename_or_url),
        ));

        Ok(())
    }
}
