use super::packet_handler::PacketHandler;
use crate::core::error::{ArceusError, ProtocolError, Result};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Arc;

pub struct HandlerRegistry {
    handlers: HashMap<u8, Arc<dyn PacketHandler>>,
}

impl HandlerRegistry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register(&mut self, handler: Arc<dyn PacketHandler>) {
        let opcode = handler.opcode();
        tracing::debug!(
            "Registered handler for opcode 0x{:02X}",
            opcode
        );
        self.handlers.insert(opcode, handler);
    }

    /// Handle an incoming packet by dispatching to the appropriate handler
    ///
    /// # Arguments
    /// * `device` - The device connection this packet came from
    /// * `opcode` - The packet opcode
    /// * `src` - Reader positioned at the start of the payload
    /// * `dst` - Writer for the response packet
    ///
    /// # Returns
    /// Ok(()) if handled successfully, Err if no handler found or handler failed
    pub async fn handle(
        &self,
        device: &Arc<crate::network::DeviceConnection>,
        opcode: u8,
        src: &mut (dyn Read + Send),
        dst: &mut (dyn Write + Send),
    ) -> Result<()> {
        let handler = self.handlers.get(&opcode).ok_or_else(|| {
            tracing::warn!("No handler for opcode 0x{:02X}", opcode);
            ArceusError::Protocol(ProtocolError::MalformedPacket(
                format!("No handler registered for opcode 0x{:02X}", opcode)
            ))
        })?;

        handler.handle(device, src, dst).await
    }
}

impl Default for HandlerRegistry {
    fn default() -> Self {
        Self::new()
    }
}
