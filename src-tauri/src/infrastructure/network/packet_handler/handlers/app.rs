/// Application-related packet handlers (FOREGROUND_APP_CHANGED)

use crate::app::EventBus;
use crate::application::dto::DeviceStateDto;
use crate::domain::models::DeviceId;
use crate::domain::repositories::DeviceRepository;
use crate::infrastructure::protocol::opcodes;
use crate::net::io::ProtocolReadExt;
use async_trait::async_trait;
use std::io::Cursor;
use std::sync::Arc;

use super::super::{PacketHandler, Result};

/// Handles FOREGROUND_APP_CHANGED (0x06) packets
/// Payload: [package_name: String][app_name: String]
pub struct ForegroundAppChangedHandler {
    device_repo: Arc<dyn DeviceRepository>,
    event_bus: Arc<EventBus>,
}

impl ForegroundAppChangedHandler {
    pub fn new(device_repo: Arc<dyn DeviceRepository>, event_bus: Arc<EventBus>) -> Self {
        Self {
            device_repo,
            event_bus,
        }
    }
}

#[async_trait]
impl PacketHandler for ForegroundAppChangedHandler {
    fn opcode(&self) -> u8 {
        opcodes::FOREGROUND_APP_CHANGED
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);

        let package_name = cursor.read_string()?;
        let app_name = cursor.read_string()?;

        tracing::debug!(
            device_id = %device_id,
            package_name = %package_name,
            app_name = %app_name,
            "Foreground app changed"
        );

        // Update device with running app info
        if let Ok(Some(device)) = self.device_repo.find_by_id(device_id).await {
            let updated_device = device.as_ref().clone().with_running_app(app_name.clone());
            self.device_repo.save(updated_device.clone()).await?;

            // Emit event to frontend with full device state
            let device_state = DeviceStateDto::from(&Arc::new(updated_device));
            self.event_bus.device_updated(device_state);
        }

        Ok(())
    }
}
