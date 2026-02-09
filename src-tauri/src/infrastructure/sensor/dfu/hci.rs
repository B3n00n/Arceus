/// HCI packet framing for Nordic DFU serial transport.
///
/// Each packet has a 4-byte header, a payload, and a 2-byte CRC16,
/// all SLIP-encoded and wrapped with 0xC0 delimiters.

use super::crc16::calc_crc16;
use super::slip::slip_encode;

const SLIP_END: u8 = 0xC0;

pub struct HciSequence {
    seq: u8,
}

impl HciSequence {
    pub fn new() -> Self {
        Self { seq: 0 }
    }

    /// Build a complete wire-ready HCI packet with SLIP framing.
    ///
    /// Sequence number is incremented BEFORE building (matching Python nrfutil),
    /// so the first packet uses seq=1.
    pub fn build_packet(&mut self, payload: &[u8]) -> Vec<u8> {
        self.seq = (self.seq + 1) % 8;
        let len = payload.len() as u16;

        let byte0 = self.seq | (((self.seq + 1) % 8) << 3) | (1 << 6) | (1 << 7);
        let byte1 = 14u8 | ((len as u8 & 0x0F) << 4);
        let byte2 = ((len & 0xFF0) >> 4) as u8;
        let byte3 = (!byte0.wrapping_add(byte1).wrapping_add(byte2)).wrapping_add(1);

        let header = [byte0, byte1, byte2, byte3];

        // CRC16 over header + payload
        let mut crc_data = Vec::with_capacity(4 + payload.len());
        crc_data.extend_from_slice(&header);
        crc_data.extend_from_slice(payload);
        let crc = calc_crc16(&crc_data);

        // SLIP encode: header + payload + CRC16 LE
        let mut raw = crc_data;
        raw.push(crc as u8);
        raw.push((crc >> 8) as u8);

        let slip_body = slip_encode(&raw);

        let mut packet = Vec::with_capacity(slip_body.len() + 2);
        packet.push(SLIP_END);
        packet.extend_from_slice(&slip_body);
        packet.push(SLIP_END);

        packet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_starts_and_ends_with_slip_end() {
        let mut seq = HciSequence::new();
        let pkt = seq.build_packet(&[0x01, 0x02]);
        assert_eq!(pkt[0], SLIP_END);
        assert_eq!(*pkt.last().unwrap(), SLIP_END);
    }

    #[test]
    fn first_packet_uses_seq_1() {
        let mut seq = HciSequence::new();
        seq.build_packet(&[0x00]);
        assert_eq!(seq.seq, 1); // pre-incremented to 1
    }

    #[test]
    fn sequence_wraps_at_8() {
        let mut seq = HciSequence::new();
        for _ in 0..8 {
            seq.build_packet(&[0x00]);
        }
        // After 8 packets: 1,2,3,4,5,6,7,0 → wraps to 0
        assert_eq!(seq.seq, 0);
    }

    #[test]
    fn header_checksum_is_twos_complement() {
        let seq = 1u8; // first packet uses seq=1
        let byte0 = seq | (((seq + 1) % 8) << 3) | (1 << 6) | (1 << 7);
        let byte1 = 14u8;
        let byte2 = 0u8;
        let byte3 = (!byte0.wrapping_add(byte1).wrapping_add(byte2)).wrapping_add(1);
        let sum = byte0.wrapping_add(byte1).wrapping_add(byte2).wrapping_add(byte3);
        assert_eq!(sum, 0);
    }
}
