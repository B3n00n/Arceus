/// Device Session - Pure I/O layer
/// Handles low-level network communication with a device.
/// No business logic, state management, or event emission - just I/O.

use crate::domain::models::DeviceId;
use crate::infrastructure::protocol::{RawPacket, RawPacketCodec};
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_util::codec::Framed;

pub struct DeviceSession {
    /// Unique identifier for this session
    id: DeviceId,
    /// Read half of the framed stream
    read_stream: Arc<Mutex<futures::stream::SplitStream<Framed<TcpStream, RawPacketCodec>>>>,
    /// Write half of the framed stream
    write_stream:
        Arc<Mutex<futures::stream::SplitSink<Framed<TcpStream, RawPacketCodec>, RawPacket>>>,
    /// Remote address of the device
    addr: SocketAddr,
}

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Failed to receive packet: {0}")]
    ReceiveError(String),

    #[error("Failed to send packet: {0}")]
    SendError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

impl DeviceSession {
    pub fn new(stream: TcpStream, id: DeviceId, addr: SocketAddr) -> Self {
        let framed = Framed::new(stream, RawPacketCodec);
        let (write, read) = framed.split();

        Self {
            id,
            read_stream: Arc::new(Mutex::new(read)),
            write_stream: Arc::new(Mutex::new(write)),
            addr,
        }
    }

    /// Receive a packet from the device
    /// Returns `None` if the stream has closed gracefully.
    pub async fn receive_packet(&self) -> Result<Option<RawPacket>, SessionError> {
        let mut stream = self.read_stream.lock().await;

        match stream.next().await {
            Some(Ok(packet)) => {
                tracing::trace!(
                    device_id = %self.id,
                    opcode = packet.opcode,
                    payload_len = packet.payload.len(),
                    "Received packet"
                );

                Ok(Some(packet))
            }
            Some(Err(e)) => {
                tracing::error!(
                    device_id = %self.id,
                    error = %e,
                    "Error receiving packet"
                );
                Err(SessionError::ReceiveError(e.to_string()))
            }
            None => {
                tracing::debug!(device_id = %self.id, "Stream closed");
                Ok(None)
            }
        }
    }

    /// Send a packet to the device
    pub async fn send_packet(&self, packet: RawPacket) -> Result<(), SessionError> {
        let mut stream = self.write_stream.lock().await;

        tracing::trace!(
            device_id = %self.id,
            opcode = packet.opcode,
            payload_len = packet.payload.len(),
            "Sending packet"
        );

        stream
            .send(packet)
            .await
            .map_err(|e| SessionError::SendError(e.to_string()))?;

        Ok(())
    }
}

// Implement Debug manually to avoid printing the entire stream state
impl std::fmt::Debug for DeviceSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceSession")
            .field("id", &self.id)
            .field("addr", &self.addr)
            .finish()
    }
}
