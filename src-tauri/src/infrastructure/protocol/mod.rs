/// Network protocol definitions
/// Opcodes and binary codec for device communication
pub mod opcodes;
mod raw_codec;

pub use raw_codec::{RawPacket, RawPacketCodec};
