/// CRC16-CCITT used by the Nordic DFU protocol.
///
/// Polynomial 0x8005, initial value 0xFFFF, bit-reversed.

pub fn calc_crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc = (crc >> 8 & 0x00FF) | (crc << 8 & 0xFF00);
        crc ^= byte as u16;
        crc ^= (crc & 0x00FF) >> 4;
        crc ^= (crc << 8) << 4;
        crc ^= ((crc & 0x00FF) << 4) << 1;
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_data_returns_initial() {
        assert_eq!(calc_crc16(&[]), 0xFFFF);
    }

    #[test]
    fn known_vector() {
        // "123456789" is the standard CRC16-CCITT test vector
        let data = b"123456789";
        let crc = calc_crc16(data);
        // This specific polynomial/init combo yields 0x29B1
        assert_eq!(crc, 0x29B1);
    }

    #[test]
    fn single_byte() {
        let crc = calc_crc16(&[0x00]);
        assert_ne!(crc, 0xFFFF); // Should differ from empty
    }
}
