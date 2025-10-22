use crate::core::error::{ArceusError, Result};
use crate::protocol::{ClientPacket, ServerPacket};
use bytes::{BytesMut, BufMut, Buf};
use tokio_util::codec::{Decoder, Encoder};
use std::io::Cursor;
use crate::net::io::{ProtocolReadable, ProtocolWritable};

/// Codec for framing client packets (CLIENT → SERVER)
pub struct ClientPacketCodec;

impl Decoder for ClientPacketCodec {
    type Item = ClientPacket;
    type Error = ArceusError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        if src.is_empty() {
            return Ok(None);
        }

        // Try to read the packet
        let mut cursor = Cursor::new(&src[..]);

        match ClientPacket::read(&mut cursor) {
            Ok(packet) => {
                // Consume the bytes we just read
                let pos = cursor.position() as usize;
                src.advance(pos);
                Ok(Some(packet))
            }
            Err(e) => {
                // Check if it's an "insufficient data" error (we need more bytes)
                if matches!(e, crate::net::io::PacketReadError::ReadError(ref io_err)
                    if io_err.kind() == std::io::ErrorKind::UnexpectedEof)
                {
                    // Need more data
                    Ok(None)
                } else {
                    // Actual error
                    Err(ArceusError::Protocol(crate::core::error::ProtocolError::MalformedPacket(
                        format!("Failed to decode client packet: {}", e)
                    )))
                }
            }
        }
    }
}

impl Encoder<ServerPacket> for ClientPacketCodec {
    type Error = ArceusError;

    fn encode(&mut self, item: ServerPacket, dst: &mut BytesMut) -> Result<()> {
        let mut buffer = Vec::new();
        item.write(&mut buffer)
            .map_err(|e| ArceusError::Protocol(crate::core::error::ProtocolError::MalformedPacket(
                format!("Failed to encode server packet: {}", e)
            )))?;

        dst.put_slice(&buffer);
        Ok(())
    }
}

/// Codec for framing server packets (SERVER → CLIENT, used in testing)
pub struct ServerPacketCodec;

impl Decoder for ServerPacketCodec {
    type Item = ServerPacket;
    type Error = ArceusError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        if src.is_empty() {
            return Ok(None);
        }

        let mut cursor = Cursor::new(&src[..]);

        match ServerPacket::read(&mut cursor) {
            Ok(packet) => {
                let pos = cursor.position() as usize;
                src.advance(pos);
                Ok(Some(packet))
            }
            Err(e) => {
                if matches!(e, crate::net::io::PacketReadError::ReadError(ref io_err)
                    if io_err.kind() == std::io::ErrorKind::UnexpectedEof)
                {
                    Ok(None)
                } else {
                    Err(ArceusError::Protocol(crate::core::error::ProtocolError::MalformedPacket(
                        format!("Failed to decode server packet: {}", e)
                    )))
                }
            }
        }
    }
}

impl Encoder<ClientPacket> for ServerPacketCodec {
    type Error = ArceusError;

    fn encode(&mut self, item: ClientPacket, dst: &mut BytesMut) -> Result<()> {
        let mut buffer = Vec::new();
        item.write(&mut buffer)
            .map_err(|e| ArceusError::Protocol(crate::core::error::ProtocolError::MalformedPacket(
                format!("Failed to encode client packet: {}", e)
            )))?;

        dst.put_slice(&buffer);
        Ok(())
    }
}
