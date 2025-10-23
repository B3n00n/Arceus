pub mod io;
pub mod macros;

pub use io::{ProtocolReadExt, ProtocolWriteExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketLength {
    Fixed(usize),
    VariableByte,
    VariableShort,
}
