use super::traits::PacketHandler;
use crate::core::{CommandResult, Result};
use crate::network::DeviceConnection;
use crate::protocol::client_packet::ApkInstallResponse;
use crate::protocol::ClientPacket;
use async_trait::async_trait;
use std::sync::Arc;

/// Handler for ApkInstallResponse packets (opcode 0x14)
pub struct ApkInstallHandler;

impl ApkInstallHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for ApkInstallHandler {
    fn name(&self) -> &'static str {
        "ApkInstallHandler"
    }

    fn handles_packet(&self, packet: &ClientPacket) -> bool {
        matches!(packet, ClientPacket::ApkInstallResponse(_))
    }

    async fn handle(&self, device: &Arc<DeviceConnection>, packet: ClientPacket) -> Result<()> {
        if let ClientPacket::ApkInstallResponse(ApkInstallResponse { success, message }) = packet
        {
            let result = if success {
                CommandResult::success("APK Install", message)
            } else {
                CommandResult::failure("APK Install", message)
            };

            device.add_command_result(result);

            tracing::debug!(
                device = %device.serial(),
                success = success,
                "APK install response received"
            );
        }
        Ok(())
    }
}

impl Default for ApkInstallHandler {
    fn default() -> Self {
        Self::new()
    }
}
