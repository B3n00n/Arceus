use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{ErrorKind, Read, Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PacketReadError {
    #[error("invalid opcode `{0}`")]
    InvalidOpcode(u8),
    #[error("read error")]
    ReadError(#[from] std::io::Error),
    #[error("connection closed")]
    StreamClosed,
}

pub trait ProtocolReadable {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: std::io::Read + byteorder::ReadBytesExt,
        Self: Sized;
}

pub trait ProtocolWritable {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt;
}

pub trait ProtocolWriteExt {
    fn write_string<T: AsRef<str>>(&mut self, text: T) -> Result<(), std::io::Error>;
}

pub trait ProtocolReadExt {
    fn read_string(&mut self) -> Result<String, std::io::Error>;
}

impl<W: Write + WriteBytesExt> ProtocolWriteExt for W {
    fn write_string<T: AsRef<str>>(&mut self, text: T) -> Result<(), std::io::Error> {
        let text = text.as_ref().as_bytes();
        assert!(text.len() <= u8::MAX as usize);
        self.write_u8(text.len() as u8)?;
        self.write_all(text)?;
        Ok(())
    }
}

impl<R: Read + ReadBytesExt> ProtocolReadExt for R {
    fn read_string(&mut self) -> Result<String, std::io::Error> {
        let length = self.read_u8()? as usize;
        let mut dst = vec![0; length];
        self.read_exact(&mut dst)?;

        match String::from_utf8(dst) {
            Ok(str) => Ok(str),
            Err(_) => Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "invalid string data",
            )),
        }
    }
}
