/// DFU protocol state machine: START → INIT → DATA → STOP.
///
/// Orchestrates the full firmware upload sequence over a [`DfuTransport`].
/// Opcodes and payload formats match `adafruit-nrfutil` (legacy Nordic DFU).

use super::init_packet::build_init_packet;
use super::transport::DfuTransport;
use super::SensorError;
use std::time::Duration;

/// DFU packet opcodes (prepended as first u32 in every payload).
const DFU_INIT_PACKET: u32 = 1;
const DFU_START_PACKET: u32 = 3;
const DFU_DATA_PACKET: u32 = 4;
const DFU_STOP_DATA_PACKET: u32 = 5;

/// Application-only update mode for the START command.
const DFU_UPDATE_MODE_APP: u32 = 4;

const DATA_CHUNK_SIZE: usize = 512;
const CHUNKS_PER_FLASH_PAGE: usize = 8;
const FLASH_PAGE_DELAY_MS: u64 = 102;

/// ~89.7 ms per 4 KiB flash page erase on the nRF52840.
const ERASE_MS_PER_PAGE: u64 = 90;
const FLASH_PAGE_SIZE: usize = 4096;
const MIN_ERASE_DELAY_MS: u64 = 500;

pub fn run_dfu_upload(
    transport: &mut DfuTransport,
    firmware: &[u8],
    on_progress: &dyn Fn(f32),
) -> Result<(), SensorError> {
    on_progress(0.0);
    send_start(transport, firmware.len())?;
    send_init(transport, firmware)?;
    send_data(transport, firmware, on_progress)?;
    send_stop(transport)?;
    on_progress(100.0);
    Ok(())
}

/// START — announce application size so the bootloader can erase flash.
fn send_start(transport: &mut DfuTransport, app_size: usize) -> Result<(), SensorError> {
    tracing::info!("DFU START: application size = {} bytes", app_size);

    let mut payload = Vec::with_capacity(20);
    payload.extend_from_slice(&DFU_START_PACKET.to_le_bytes());
    payload.extend_from_slice(&DFU_UPDATE_MODE_APP.to_le_bytes());
    payload.extend_from_slice(&0u32.to_le_bytes()); // softdevice size
    payload.extend_from_slice(&0u32.to_le_bytes()); // bootloader size
    payload.extend_from_slice(&(app_size as u32).to_le_bytes());

    transport.send_and_ack(&payload)?;

    // The device ACKs immediately at the HCI level, then erases flash.
    // We must wait for the erase before sending INIT.
    let pages = (app_size / FLASH_PAGE_SIZE + 1) as u64;
    let delay = (pages * ERASE_MS_PER_PAGE).max(MIN_ERASE_DELAY_MS);
    tracing::info!("DFU START acknowledged, waiting {}ms for flash erase", delay);
    std::thread::sleep(Duration::from_millis(delay));

    Ok(())
}

/// INIT — send the 14-byte init packet (device type, CRC, etc.).
fn send_init(transport: &mut DfuTransport, firmware: &[u8]) -> Result<(), SensorError> {
    tracing::info!("DFU INIT: sending init packet");

    let init_pkt = build_init_packet(firmware);
    let mut payload = Vec::with_capacity(20);
    payload.extend_from_slice(&DFU_INIT_PACKET.to_le_bytes());
    payload.extend_from_slice(&init_pkt);
    payload.extend_from_slice(&0u16.to_le_bytes()); // padding

    transport.send_and_ack(&payload)?;
    Ok(())
}

/// DATA — stream firmware in 512-byte chunks with flash-write pacing.
fn send_data(transport: &mut DfuTransport, firmware: &[u8], on_progress: &dyn Fn(f32)) -> Result<(), SensorError> {
    let total_chunks = firmware.len().div_ceil(DATA_CHUNK_SIZE);
    let log_interval = (total_chunks / 10).max(1);

    tracing::info!(
        "DFU DATA: sending {} bytes in {} chunks",
        firmware.len(),
        total_chunks
    );

    for (i, chunk) in firmware.chunks(DATA_CHUNK_SIZE).enumerate() {
        let mut payload = Vec::with_capacity(4 + chunk.len());
        payload.extend_from_slice(&DFU_DATA_PACKET.to_le_bytes());
        payload.extend_from_slice(chunk);

        transport.send_and_ack(&payload)?;

        if (i + 1) % CHUNKS_PER_FLASH_PAGE == 0 {
            std::thread::sleep(Duration::from_millis(FLASH_PAGE_DELAY_MS));
        }

        if (i + 1) % log_interval == 0 || i + 1 == total_chunks {
            let pct = ((i + 1) as f32 / total_chunks as f32) * 100.0;
            tracing::info!("DFU progress: {:.0}% ({}/{})", pct, i + 1, total_chunks);
            on_progress(pct);
        }
    }

    Ok(())
}

/// STOP — signal end of transfer; device validates and activates new firmware.
fn send_stop(transport: &mut DfuTransport) -> Result<(), SensorError> {
    tracing::info!("DFU STOP: finalizing upload");

    let payload = DFU_STOP_DATA_PACKET.to_le_bytes();
    transport.send_and_ack(&payload)?;

    std::thread::sleep(Duration::from_millis(190));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_payload_is_20_bytes() {
        let mut payload = Vec::with_capacity(20);
        payload.extend_from_slice(&DFU_START_PACKET.to_le_bytes());
        payload.extend_from_slice(&DFU_UPDATE_MODE_APP.to_le_bytes());
        payload.extend_from_slice(&0u32.to_le_bytes());
        payload.extend_from_slice(&0u32.to_le_bytes());
        payload.extend_from_slice(&327680u32.to_le_bytes());
        assert_eq!(payload.len(), 20);
        assert_eq!(u32::from_le_bytes(payload[16..20].try_into().unwrap()), 327680);
    }

    #[test]
    fn init_payload_is_20_bytes() {
        use super::build_init_packet;
        let init_pkt = build_init_packet(&[0xAA; 100]);
        let mut payload = Vec::with_capacity(20);
        payload.extend_from_slice(&DFU_INIT_PACKET.to_le_bytes());
        payload.extend_from_slice(&init_pkt);
        payload.extend_from_slice(&0u16.to_le_bytes());
        assert_eq!(payload.len(), 20);
    }
}
