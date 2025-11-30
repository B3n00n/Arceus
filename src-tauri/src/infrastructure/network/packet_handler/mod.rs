/// Packet handler system for processing incoming device packets
/// Handlers update the device repository based on received packets.

use crate::domain::models::DeviceId;
use crate::infrastructure::protocol::RawPacket;
use async_trait::async_trait;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, crate::app::error::ArceusError>;

mod handlers;
pub use handlers::*;

#[async_trait]
pub trait PacketHandler: Send + Sync {
    fn opcode(&self) -> u8;
    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()>;
}

pub struct PacketHandlerRegistry {
    handlers: std::collections::HashMap<u8, Arc<dyn PacketHandler>>,
}

impl PacketHandlerRegistry {
    pub fn new(
        device_repo: Arc<dyn crate::domain::repositories::DeviceRepository>,
        device_name_repo: Arc<dyn crate::domain::repositories::DeviceNameRepository>,
        event_bus: Arc<crate::app::EventBus>,
        session_manager: Arc<crate::infrastructure::network::device_session_manager::DeviceSessionManager>,
    ) -> Self {
        let mut registry = Self {
            handlers: std::collections::HashMap::new(),
        };

        registry.register(Arc::new(DeviceConnectedHandler::new(
            device_repo.clone(),
            device_name_repo.clone(),
            event_bus.clone(),
            session_manager.clone(),
        )));
        registry.register(Arc::new(HeartbeatHandler::new()));
        registry.register(Arc::new(BatteryStatusHandler::new(
            device_repo.clone(),
            event_bus.clone(),
        )));
        registry.register(Arc::new(VolumeStatusHandler::new(
            device_repo.clone(),
            event_bus.clone(),
        )));
        registry.register(Arc::new(ForegroundAppChangedHandler::new(
            device_repo.clone(),
            event_bus.clone(),
        )));

        // Response handlers
        registry.register(Arc::new(LaunchAppResponseHandler::new(event_bus.clone())));
        registry.register(Arc::new(ShellExecutionResponseHandler::new(event_bus.clone())));
        registry.register(Arc::new(InstalledAppsResponseHandler::new(event_bus.clone())));
        registry.register(Arc::new(PingResponseHandler::new(event_bus.clone())));
        registry.register(Arc::new(ApkInstallResponseHandler::new(event_bus.clone())));
        registry.register(Arc::new(UninstallAppResponseHandler::new(event_bus.clone())));
        registry.register(Arc::new(VolumeSetResponseHandler::new(
            device_repo.clone(),
            event_bus.clone(),
        )));
        registry.register(Arc::new(ApkDownloadStartedHandler::new(event_bus.clone())));
        registry.register(Arc::new(ApkDownloadProgressHandler::new(event_bus.clone(), device_repo.clone())));
        registry.register(Arc::new(ApkInstallProgressHandler::new(event_bus.clone(), device_repo.clone())));
        registry.register(Arc::new(CloseAllAppsResponseHandler::new(event_bus.clone())));

        registry
    }

    /// Register a packet handler
    pub fn register(&mut self, handler: Arc<dyn PacketHandler>) {
        let opcode = handler.opcode();
        self.handlers.insert(opcode, handler);
    }

    /// Handle a received packet
    pub async fn handle(&self, device_id: DeviceId, packet: RawPacket) -> Result<()> {
        match self.handlers.get(&packet.opcode) {
            Some(handler) => {
                handler.handle(device_id, packet.payload).await?;
                Ok(())
            }
            None => {
                tracing::debug!(
                    device_id = %device_id,
                    opcode = packet.opcode,
                    "No handler registered for opcode"
                );
                Ok(())
            }
        }
    }
}
