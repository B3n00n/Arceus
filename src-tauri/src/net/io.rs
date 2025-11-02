use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{ErrorKind, Read, Write};

pub trait ProtocolWriteExt {
    fn write_string<T: AsRef<str>>(&mut self, text: T) -> Result<(), std::io::Error>;
}

pub trait ProtocolReadExt {
    fn read_string(&mut self) -> Result<String, std::io::Error>;
}

impl<W: Write + WriteBytesExt> ProtocolWriteExt for W {
    fn write_string<T: AsRef<str>>(&mut self, text: T) -> Result<(), std::io::Error> {
        let text = text.as_ref().as_bytes();
        assert!(text.len() <= u32::MAX as usize);
        self.write_u32::<byteorder::BigEndian>(text.len() as u32)?;
        self.write_all(text)?;
        Ok(())
    }
}

impl<R: Read + ReadBytesExt> ProtocolReadExt for R {
    fn read_string(&mut self) -> Result<String, std::io::Error> {
        let length = self.read_u32::<byteorder::BigEndian>()? as usize;
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