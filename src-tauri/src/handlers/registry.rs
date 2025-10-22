use super::traits::PacketHandler;
use crate::core::Result;
use crate::network::DeviceConnection;
use crate::protocol::ClientPacket;
use std::sync::Arc;

/// Registry for client packet handlers
///
/// This registry routes incoming ClientPackets to their appropriate handlers
/// based on the packet type (discriminant of the enum).
pub struct HandlerRegistry {
    handlers: Vec<Arc<dyn PacketHandler>>,
}

impl HandlerRegistry {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    /// Register a handler for client packets
    pub fn register<H>(&mut self, handler: H) -> &mut Self
    where
        H: PacketHandler + 'static,
    {
        tracing::debug!("Registering handler: {}", handler.name());
        self.handlers.push(Arc::new(handler));
        self
    }

    /// Find a handler that can handle the given packet
    fn find_handler(&self, packet: &ClientPacket) -> Option<Arc<dyn PacketHandler>> {
        self.handlers
            .iter()
            .find(|h| h.handles_packet(packet))
            .map(Arc::clone)
    }

    /// Handle a client packet by routing it to the appropriate handler
    pub async fn handle_packet(
        &self,
        device: &Arc<DeviceConnection>,
        packet: ClientPacket,
    ) -> Result<()> {
        let handler = self.find_handler(&packet);

        match handler {
            Some(h) => {
                tracing::trace!(
                    opcode = packet.opcode(),
                    handler = %h.name(),
                    device_id = %device.id(),
                    "Routing packet to handler"
                );

                h.handle(device, packet).await.map_err(|e| {
                    tracing::error!(
                        handler = %h.name(),
                        device_id = %device.id(),
                        error = %e,
                        "Handler failed"
                    );
                    e
                })?;

                Ok(())
            }
            None => {
                tracing::warn!(
                    opcode = packet.opcode(),
                    device_id = %device.id(),
                    "No handler registered for packet type"
                );
                Ok(()) // Not an error, just no handler registered
            }
        }
    }
}

impl Default for HandlerRegistry {
    fn default() -> Self {
        Self::new()
    }
}
