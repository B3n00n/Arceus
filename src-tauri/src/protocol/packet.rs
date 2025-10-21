use crate::core::error::{ProtocolError, Result};
use bytes::{Buf, BufMut, Bytes, BytesMut};

pub struct PacketReader {
    buf: Bytes,
}

impl PacketReader {
    pub fn new(buf: Bytes) -> Self {
        Self { buf }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        if self.buf.remaining() < 1 {
            return Err(ProtocolError::InsufficientData {
                expected: 1,
                actual: self.buf.remaining(),
            }
            .into());
        }
        Ok(self.buf.get_u8())
    }

    pub fn read_string(&mut self) -> Result<String> {
        // Read length prefix (u32)
        if self.buf.remaining() < 4 {
            return Err(ProtocolError::InsufficientData {
                expected: 4,
                actual: self.buf.remaining(),
            }
            .into());
        }
        let length = self.buf.get_u32() as usize;

        // Read string bytes
        if self.buf.remaining() < length {
            return Err(ProtocolError::InsufficientData {
                expected: length,
                actual: self.buf.remaining(),
            }
            .into());
        }

        let mut bytes = vec![0u8; length];
        self.buf.copy_to_slice(&mut bytes);

        String::from_utf8(bytes).map_err(|e| {
            ProtocolError::InvalidEncoding(format!("Invalid UTF-8: {}", e)).into()
        })
    }

    pub fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_u8()? != 0)
    }
}

pub struct PacketWriter {
    buf: BytesMut,
}

impl PacketWriter {
    pub fn new() -> Self {
        Self {
            buf: BytesMut::new(),
        }
    }

    pub fn write_u8(&mut self, value: u8) -> &mut Self {
        self.buf.put_u8(value);
        self
    }

    pub fn write_string(&mut self, value: &str) -> &mut Self {
        let bytes = value.as_bytes();
        self.buf.put_u32(bytes.len() as u32);
        self.buf.put_slice(bytes);
        self
    }

    pub fn write_bool(&mut self, value: bool) -> &mut Self {
        self.write_u8(if value { 1 } else { 0 })
    }

    pub fn freeze(self) -> Bytes {
        self.buf.freeze()
    }
}

impl Default for PacketWriter {
    fn default() -> Self {
        Self::new()
    }
}
