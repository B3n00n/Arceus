use crate::core::Result;
use crate::network::DeviceConnection;
use crate::protocol::ClientPacket;
use async_trait::async_trait;
use std::sync::Arc;

/// Trait for handling specific types of client packets
///
/// Each handler implements this trait to process specific packet types
/// from the ClientPacket enum. Handlers should check the packet type
/// using pattern matching in `handles_packet()`.
#[async_trait]
pub trait PacketHandler: Send + Sync {
    /// Returns a descriptive name for this handler (for logging)
    fn name(&self) -> &'static str;

    /// Check if this handler can process the given packet type
    fn handles_packet(&self, packet: &ClientPacket) -> bool;

    /// Handle the packet
    ///
    /// This method is called when a packet matching this handler's type is received.
    /// The device parameter provides access to device state and methods for sending responses.
    async fn handle(&self, device: &Arc<DeviceConnection>, packet: ClientPacket) -> Result<()>;
}
