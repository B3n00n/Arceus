use crate::core::error::Result;
use crate::handlers::PacketHandler;
use crate::net::ProtocolReadExt;
use crate::network::DeviceConnection;
use crate::protocol::opcodes;
use async_trait::async_trait;
use byteorder::ReadBytesExt;
use std::io::{Read, Write};
use std::sync::Arc;

/// Handles InstalledAppsResponse (0x12) from client
/// Payload: [count: u32][apps: Vec<String>]
pub struct InstalledAppsResponseHandler;

impl InstalledAppsResponseHandler {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PacketHandler for InstalledAppsResponseHandler {
    fn opcode(&self) -> u8 {
        opcodes::INSTALLED_APPS_RESPONSE
    }

    async fn handle(
        &self,
        device: &Arc<DeviceConnection>,
        mut src: &mut (dyn Read + Send),
        mut _dst: &mut (dyn Write + Send),
    ) -> Result<()> {
        let count = src.read_u32::<byteorder::BigEndian>()?;
        let mut apps = Vec::with_capacity(count as usize);

        for _ in 0..count {
            apps.push(src.read_string()?);
        }

        tracing::info!("Received {} installed apps", count);
        device.update_installed_apps(apps);

        Ok(())
    }
}
