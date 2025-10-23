use crate::core::error::{ArceusError, Result};
use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

/// Raw packet structure: [opcode: u8][length: u16 BE][payload: varies]
#[derive(Debug, Clone)]
pub struct RawPacket {
    pub opcode: u8,
    pub payload: Vec<u8>,
}

impl RawPacket {
    pub fn new(opcode: u8, payload: Vec<u8>) -> Self {
        Self { opcode, payload }
    }

    pub fn empty(opcode: u8) -> Self {
        Self {
            opcode,
            payload: Vec::new(),
        }
    }
}

/// Simple codec for reading/writing raw packets
pub struct RawPacketCodec;

impl Decoder for RawPacketCodec {
    type Item = RawPacket;
    type Error = ArceusError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        // Need at least opcode + length (3 bytes)
        if src.len() < 3 {
            return Ok(None);
        }

        let opcode = src[0];
        let length = u16::from_be_bytes([src[1], src[2]]) as usize;

        // Check if we have the full packet
        let total_needed = 3 + length;
        if src.len() < total_needed {
            src.reserve(total_needed - src.len());
            return Ok(None);
        }

        // Extract the packet
        src.advance(3); // Skip opcode + length
        let payload = src.split_to(length).to_vec();

        Ok(Some(RawPacket { opcode, payload }))
    }
}

impl Encoder<RawPacket> for RawPacketCodec {
    type Error = ArceusError;

    fn encode(&mut self, item: RawPacket, dst: &mut BytesMut) -> Result<()> {
        let length = item.payload.len() as u16;
        dst.reserve(3 + item.payload.len());

        dst.put_u8(item.opcode);
        dst.put_u16(length);
        dst.put_slice(&item.payload);

        Ok(())
    }
}
