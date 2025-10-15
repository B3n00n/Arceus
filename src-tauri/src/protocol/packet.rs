use super::primitives::*;
use crate::core::error::Result;
use bytes::{Buf, BufMut, Bytes, BytesMut};

pub struct PacketReader {
    buf: Bytes,
}

impl PacketReader {
    pub fn new(buf: Bytes) -> Self {
        Self { buf }
    }

    pub fn from_vec(data: Vec<u8>) -> Self {
        Self::new(Bytes::from(data))
    }

    pub fn remaining(&self) -> usize {
        self.buf.remaining()
    }

    pub fn has_remaining(&self) -> bool {
        self.buf.has_remaining()
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        read_u8(&mut self.buf)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        read_u16(&mut self.buf)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        read_u32(&mut self.buf)
    }

    pub fn read_string(&mut self) -> Result<String> {
        read_string(&mut self.buf)
    }

    pub fn read_ascii_string(&mut self) -> Result<String> {
        read_ascii_string(&mut self.buf)
    }

    pub fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_u8()? != 0)
    }

    pub fn remaining_bytes(&self) -> &[u8] {
        self.buf.chunk()
    }

    pub fn into_bytes(self) -> Bytes {
        self.buf
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

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: BytesMut::with_capacity(capacity),
        }
    }

    pub fn write_u8(&mut self, value: u8) -> &mut Self {
        write_u8(&mut self.buf, value);
        self
    }

    pub fn write_u16(&mut self, value: u16) -> &mut Self {
        write_u16(&mut self.buf, value);
        self
    }

    pub fn write_u32(&mut self, value: u32) -> &mut Self {
        write_u32(&mut self.buf, value);
        self
    }

    pub fn write_string(&mut self, value: &str) -> &mut Self {
        write_string(&mut self.buf, value);
        self
    }

    pub fn write_ascii_string(&mut self, value: &str) -> Result<&mut Self> {
        write_ascii_string(&mut self.buf, value)?;
        Ok(self)
    }

    pub fn write_bool(&mut self, value: bool) -> &mut Self {
        self.write_u8(if value { 1 } else { 0 })
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        self.buf.put_slice(bytes);
        self
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn freeze(self) -> Bytes {
        self.buf.freeze()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.buf.to_vec()
    }
}

impl Default for PacketWriter {
    fn default() -> Self {
        Self::new()
    }
}

