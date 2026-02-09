/// Nordic DFU init packet construction.
///
/// Builds the 14-byte init packet that replaces the .dat file inside a DFU ZIP.
/// All fields are little-endian.

use super::crc16::calc_crc16;

const DEVICE_TYPE: u16 = 0x0052;
const DEVICE_REV: u16 = 0xFFFF;
const APP_VERSION: u32 = 0xFFFFFFFF;
const SD_COUNT: u16 = 1;
const SD_REQ: u16 = 0xFFFE;

pub fn build_init_packet(firmware: &[u8]) -> [u8; 14] {
    let firmware_crc = calc_crc16(firmware);

    let mut pkt = [0u8; 14];
    pkt[0..2].copy_from_slice(&DEVICE_TYPE.to_le_bytes());
    pkt[2..4].copy_from_slice(&DEVICE_REV.to_le_bytes());
    pkt[4..8].copy_from_slice(&APP_VERSION.to_le_bytes());
    pkt[8..10].copy_from_slice(&SD_COUNT.to_le_bytes());
    pkt[10..12].copy_from_slice(&SD_REQ.to_le_bytes());
    pkt[12..14].copy_from_slice(&firmware_crc.to_le_bytes());
    pkt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_packet_is_14_bytes() {
        let pkt = build_init_packet(&[0xAA; 100]);
        assert_eq!(pkt.len(), 14);
    }

    #[test]
    fn init_packet_has_correct_device_type() {
        let pkt = build_init_packet(&[0x00; 10]);
        let dev_type = u16::from_le_bytes([pkt[0], pkt[1]]);
        assert_eq!(dev_type, 0x0052);
    }

    #[test]
    fn init_packet_crc_matches_firmware() {
        let firmware = vec![0x01, 0x02, 0x03];
        let pkt = build_init_packet(&firmware);
        let pkt_crc = u16::from_le_bytes([pkt[12], pkt[13]]);
        assert_eq!(pkt_crc, calc_crc16(&firmware));
    }
}
