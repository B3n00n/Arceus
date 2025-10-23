use crate::core::error::Result;
use crate::core::models::battery::BatteryInfo;
use crate::handlers::PacketHandler;
use crate::network::DeviceConnection;
use crate::protocol::opcodes;
use async_trait::async_trait;
use byteorder::ReadBytesExt;
use std::io::{Read, Write};
use std::sync::Arc;

/// Handles BatteryStatus message (0x03) from client
/// Payload: [level: u8][is_charging: bool]
pub struct BatteryStatusHandler;

impl BatteryStatusHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for BatteryStatusHandler {
    fn opcode(&self) -> u8 {
        opcodes::BATTERY_STATUS
    }

    async fn handle(
        &self,
        device: &Arc<DeviceConnection>,
        src: &mut (dyn Read + Send),
        _dst: &mut (dyn Write + Send),
    ) -> Result<()> {
        let level = src.read_u8()?;
        let is_charging = src.read_u8()? != 0;

        tracing::debug!("Battery: {}%, charging={}", level, is_charging);

        let battery = BatteryInfo::new(level, is_charging);
        device.update_battery(battery);

        Ok(())
    }
}
