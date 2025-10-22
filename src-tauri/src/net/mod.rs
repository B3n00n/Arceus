pub mod io;
pub mod macros;

pub use io::{PacketReadError, ProtocolReadExt, ProtocolReadable, ProtocolWriteExt, ProtocolWritable};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketLength {
    Fixed(usize),
    VariableByte,
    VariableShort,
}
