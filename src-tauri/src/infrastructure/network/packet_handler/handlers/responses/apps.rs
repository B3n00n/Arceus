/// App-related response handlers (INSTALLED_APPS_RESPONSE, CLOSE_ALL_APPS_RESPONSE)

use crate::app::EventBus;
use crate::application::dto::CommandResultDto;
use crate::domain::models::DeviceId;
use crate::net::io::ProtocolReadExt;
use async_trait::async_trait;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use std::sync::Arc;

use super::super::super::{PacketHandler, Result};

/// Handles INSTALLED_APPS_RESPONSE (0x12) packets
pub struct InstalledAppsResponseHandler {
    event_bus: Arc<EventBus>,
}

impl InstalledAppsResponseHandler {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait]
impl PacketHandler for InstalledAppsResponseHandler {
    fn opcode(&self) -> u8 {
        crate::infrastructure::protocol::opcodes::INSTALLED_APPS_RESPONSE
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);
        let count = cursor.read_u32::<BigEndian>()? as usize;

        let mut apps = Vec::with_capacity(count);
        for _ in 0..count {
            let package_name = cursor.read_string()?;
            apps.push(package_name);
        }

        tracing::debug!(device_id = %device_id, app_count = count, "Installed apps response");

        self.event_bus.installed_apps_received(device_id.as_uuid().clone(), apps);

        Ok(())
    }
}

/// Handles CLOSE_ALL_APPS_RESPONSE (0x18) packets
/// Payload: [success: u8][message: String][closed_count: u32][closed_apps: List<String>]
pub struct CloseAllAppsResponseHandler {
    event_bus: Arc<EventBus>,
}

impl CloseAllAppsResponseHandler {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait]
impl PacketHandler for CloseAllAppsResponseHandler {
    fn opcode(&self) -> u8 {
        crate::infrastructure::protocol::opcodes::CLOSE_ALL_APPS_RESPONSE
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);
        let success = cursor.read_u8()? != 0;
        let message = cursor.read_string()?;
        let closed_count = cursor.read_u32::<BigEndian>()? as usize;

        let mut closed_apps = Vec::with_capacity(closed_count);
        for _ in 0..closed_count {
            let package_name = cursor.read_string()?;
            closed_apps.push(package_name);
        }

        tracing::info!(
            device_id = %device_id,
            success = success,
            closed_count = closed_count,
            "Close all apps response: {}",
            message
        );

        let result = if success {
            CommandResultDto::success("close_all_apps", "Successfully closed all apps")
        } else {
            CommandResultDto::failure("close_all_apps", &message)
        };
        self.event_bus.command_executed(device_id.as_uuid().clone(), result);

        Ok(())
    }
}
