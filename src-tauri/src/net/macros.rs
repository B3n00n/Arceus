#[macro_export]
macro_rules! packets {
    (
        $ident:ident {
            $($packet:ident => (opcode: $opcode:literal, $length:expr)), * $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub enum $ident {
            $(
                $packet($packet),
            )*
        }

        impl $ident {
            pub fn opcode(&self) -> u8 {
                match self {
                    $(
                        $ident::$packet(_) => $opcode,
                    )*
                }
            }

            pub fn length(&self) -> $crate::net::PacketLength {
                match self {
                    $(
                        $ident::$packet(_) => $length,
                    )*
                }
            }
        }

        impl $crate::net::io::ProtocolReadable for $ident {
            fn read<T>(src: &mut T) -> Result<Self, $crate::net::io::PacketReadError>
            where
                T: std::io::Read + byteorder::ReadBytesExt,
                Self: Sized
            {
                let opcode = src.read_u8()?;
                match opcode {
                    $(
                        opcode if opcode == $opcode => {
                            let length = match $length {
                                $crate::net::PacketLength::Fixed(len) => len,
                                $crate::net::PacketLength::VariableByte => src.read_u8()? as usize,
                                $crate::net::PacketLength::VariableShort => src.read_u16::<byteorder::BigEndian>()? as usize
                            };

                            let mut payload = vec![0; length];
                            src.read_exact(&mut payload)?;

                            let mut cursor = std::io::Cursor::new(&payload[..]);
                            Ok($ident::$packet($packet::read(&mut cursor)?))
                        }
                    )*

                    _ => Err($crate::net::io::PacketReadError::InvalidOpcode(opcode))
                }
            }
        }

        impl $crate::net::io::ProtocolWritable for $ident {
            fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
            where T: std::io::Write + byteorder::WriteBytesExt {
                dst.write_u8(self.opcode())?;
                match self {
                    $(
                        $ident::$packet(packet) => {
                            match self.length() {
                                $crate::net::PacketLength::Fixed(_) => packet.write(dst)?,
                                $crate::net::PacketLength::VariableByte => {
                                    let mut msg_dst = Vec::with_capacity(u8::MAX.into());
                                    packet.write(&mut msg_dst)?;

                                    assert!(msg_dst.len() <= u8::MAX.into());
                                    dst.write_u8(msg_dst.len() as u8)?;
                                    dst.write_all(&msg_dst)?;
                                },
                                $crate::net::PacketLength::VariableShort => {
                                    let mut msg_dst = Vec::with_capacity(u16::MAX.into());
                                    packet.write(&mut msg_dst)?;

                                    assert!(msg_dst.len() <= u16::MAX.into());
                                    dst.write_u16::<byteorder::BigEndian>(msg_dst.len() as u16)?;
                                    dst.write_all(&msg_dst)?;
                                }
                            }
                        }
                    )*
                }
                Ok(())
            }
        }

        $(
            impl From<$packet> for $ident {
                fn from(packet: $packet) -> Self {
                    $ident::$packet(packet)
                }
            }
        )*
    };
}
