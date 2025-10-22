use super::traits::PacketHandler;
use crate::core::Result;
use crate::network::DeviceConnection;
use crate::protocol::client_packet::InstalledAppsResponse;
use crate::protocol::ClientPacket;
use async_trait::async_trait;
use std::sync::Arc;

/// Handler for InstalledAppsResponse packets (opcode 0x12)
pub struct InstalledAppsHandler;

impl InstalledAppsHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for InstalledAppsHandler {
    fn name(&self) -> &'static str {
        "InstalledAppsHandler"
    }

    fn handles_packet(&self, packet: &ClientPacket) -> bool {
        matches!(packet, ClientPacket::InstalledAppsResponse(_))
    }

    async fn handle(&self, device: &Arc<DeviceConnection>, packet: ClientPacket) -> Result<()> {
        if let ClientPacket::InstalledAppsResponse(InstalledAppsResponse { apps }) = packet {
            let app_count = apps.len();

            // Emit event with installed apps list
            device
                .event_bus()
                .installed_apps_received(device.id(), apps);

            tracing::debug!(
                device = %device.serial(),
                app_count = app_count,
                "Installed apps response received"
            );
        }
        Ok(())
    }
}

impl Default for InstalledAppsHandler {
    fn default() -> Self {
        Self::new()
    }
}
