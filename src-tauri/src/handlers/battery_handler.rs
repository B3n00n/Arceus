use super::traits::MessageHandler;
use crate::core::{BatteryInfo, Result};
use crate::network::DeviceConnection;
use crate::protocol::{MessageType, PacketReader};
use async_trait::async_trait;
use bytes::Bytes;
use std::sync::Arc;

pub struct BatteryHandler;

impl BatteryHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MessageHandler for BatteryHandler {
    fn message_type(&self) -> MessageType {
        MessageType::BatteryStatus
    }

    async fn handle(&self, device: &Arc<DeviceConnection>, payload: Bytes) -> Result<()> {
        let mut reader = PacketReader::new(payload);
        let level = reader.read_u8()?;
        let is_charging = reader.read_bool()?;
        let battery_info = BatteryInfo::new(level, is_charging);
        device.update_battery(battery_info);

        tracing::trace!(
            "Device {} battery: {}%{}",
            device.serial(),
            level,
            if is_charging { " (charging)" } else { "" }
        );

        Ok(())
    }
}

impl Default for BatteryHandler {
    fn default() -> Self {
        Self::new()
    }
}
