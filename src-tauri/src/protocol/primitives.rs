use crate::core::error::{ProtocolError, Result};
use bytes::{Buf, BufMut};

pub fn read_u8(buf: &mut impl Buf) -> Result<u8> {
    if buf.remaining() < 1 {
        return Err(ProtocolError::InsufficientData {
            expected: 1,
            actual: buf.remaining(),
        }
        .into());
    }
    Ok(buf.get_u8())
}

pub fn read_u16(buf: &mut impl Buf) -> Result<u16> {
    if buf.remaining() < 2 {
        return Err(ProtocolError::InsufficientData {
            expected: 2,
            actual: buf.remaining(),
        }
        .into());
    }
    Ok(buf.get_u16())
}

pub fn read_u32(buf: &mut impl Buf) -> Result<u32> {
    if buf.remaining() < 4 {
        return Err(ProtocolError::InsufficientData {
            expected: 4,
            actual: buf.remaining(),
        }
        .into());
    }
    Ok(buf.get_u32())
}

pub fn read_string(buf: &mut impl Buf) -> Result<String> {
    let length = read_u32(buf)? as usize;

    if buf.remaining() < length {
        return Err(ProtocolError::InsufficientData {
            expected: length,
            actual: buf.remaining(),
        }
        .into());
    }

    let mut bytes = vec![0u8; length];
    buf.copy_to_slice(&mut bytes);

    String::from_utf8(bytes).map_err(|e| {
        ProtocolError::InvalidEncoding(format!("Invalid UTF-8: {}", e)).into()
    })
}

pub fn read_ascii_string(buf: &mut impl Buf) -> Result<String> {
    let length = read_u16(buf)? as usize;

    if buf.remaining() < length {
        return Err(ProtocolError::InsufficientData {
            expected: length,
            actual: buf.remaining(),
        }
        .into());
    }

    let mut bytes = vec![0u8; length];
    buf.copy_to_slice(&mut bytes);

    if !bytes.iter().all(|&b| b.is_ascii()) {
        return Err(ProtocolError::InvalidEncoding("Non-ASCII characters found".to_string()).into());
    }

    String::from_utf8(bytes)
        .map_err(|e| ProtocolError::InvalidEncoding(format!("Invalid ASCII: {}", e)).into())
}

pub fn write_u8(buf: &mut impl BufMut, value: u8) {
    buf.put_u8(value);
}

pub fn write_u16(buf: &mut impl BufMut, value: u16) {
    buf.put_u16(value);
}

pub fn write_u32(buf: &mut impl BufMut, value: u32) {
    buf.put_u32(value);
}

pub fn write_string(buf: &mut impl BufMut, value: &str) {
    let bytes = value.as_bytes();
    write_u32(buf, bytes.len() as u32);
    buf.put_slice(bytes);
}

pub fn write_ascii_string(buf: &mut impl BufMut, value: &str) -> Result<()> {
    let bytes = value.as_bytes();

    if !bytes.iter().all(|&b| b.is_ascii()) {
        return Err(ProtocolError::InvalidEncoding("Non-ASCII characters found".to_string()).into());
    }

    if bytes.len() > u16::MAX as usize {
        return Err(
            ProtocolError::MalformedPacket("ASCII string too long".to_string()).into(),
        );
    }

    write_u16(buf, bytes.len() as u16);
    buf.put_slice(bytes);
    Ok(())
}

