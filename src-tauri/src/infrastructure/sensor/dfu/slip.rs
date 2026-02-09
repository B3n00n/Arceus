/// SLIP (Serial Line Internet Protocol) framing for Nordic DFU.

const SLIP_END: u8 = 0xC0;
const SLIP_ESC: u8 = 0xDB;
const SLIP_ESC_END: u8 = 0xDC;
const SLIP_ESC_ESC: u8 = 0xDD;

pub fn slip_encode(data: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(data.len() + data.len() / 10);
    for &byte in data {
        match byte {
            SLIP_END => {
                encoded.push(SLIP_ESC);
                encoded.push(SLIP_ESC_END);
            }
            SLIP_ESC => {
                encoded.push(SLIP_ESC);
                encoded.push(SLIP_ESC_ESC);
            }
            _ => encoded.push(byte),
        }
    }
    encoded
}

pub fn slip_decode(data: &[u8]) -> Vec<u8> {
    let mut decoded = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() {
        if data[i] == SLIP_ESC && i + 1 < data.len() {
            match data[i + 1] {
                SLIP_ESC_END => decoded.push(SLIP_END),
                SLIP_ESC_ESC => decoded.push(SLIP_ESC),
                other => {
                    decoded.push(SLIP_ESC);
                    decoded.push(other);
                }
            }
            i += 2;
        } else {
            decoded.push(data[i]);
            i += 1;
        }
    }
    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_no_special_bytes() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        assert_eq!(slip_decode(&slip_encode(&data)), data);
    }

    #[test]
    fn roundtrip_with_end_byte() {
        let data = vec![0x01, SLIP_END, 0x03];
        let encoded = slip_encode(&data);
        assert!(encoded.contains(&SLIP_ESC));
        assert!(!encoded.contains(&SLIP_END));
        assert_eq!(slip_decode(&encoded), data);
    }

    #[test]
    fn roundtrip_with_esc_byte() {
        let data = vec![SLIP_ESC, 0x02];
        let encoded = slip_encode(&data);
        assert_eq!(slip_decode(&encoded), data);
    }

    #[test]
    fn encode_does_not_contain_end_marker() {
        let data = vec![SLIP_END, SLIP_END, SLIP_END];
        let encoded = slip_encode(&data);
        assert!(!encoded.contains(&SLIP_END));
    }
}
