/// Firmware binary patching for BLE device name

use super::{Result, SensorError};

/// Default placeholder string in firmware
const DEFAULT_PLACEHOLDER: &str = "PLACEHOLDER_BLE_NAME_HERE";

/// Patches firmware binary to set a custom BLE device name
pub struct FirmwarePatcher;

impl FirmwarePatcher {
    /// Patch the device name in firmware binary
    ///
    /// Finds the placeholder string and replaces it with the new name,
    /// padded with null bytes to maintain the same length.
    pub fn patch_device_name(firmware: &[u8], device_name: &str) -> Result<Vec<u8>> {
        let placeholder = DEFAULT_PLACEHOLDER.as_bytes();
        let max_len = placeholder.len();

        // Validate name length
        if device_name.len() > max_len {
            return Err(SensorError::NameTooLong { max: max_len });
        }

        // Find placeholder in firmware
        let offset = Self::find_placeholder(firmware, placeholder)
            .ok_or(SensorError::PlaceholderNotFound)?;

        tracing::debug!(
            "Found placeholder at offset 0x{:08X}, replacing with '{}'",
            offset,
            device_name
        );

        // Create patched firmware
        let mut patched = firmware.to_vec();

        // Create null-padded replacement
        let mut replacement = vec![0u8; max_len];
        replacement[..device_name.len()].copy_from_slice(device_name.as_bytes());

        // Apply patch
        patched[offset..offset + max_len].copy_from_slice(&replacement);

        Ok(patched)
    }

    /// Find the placeholder string in firmware
    fn find_placeholder(firmware: &[u8], placeholder: &[u8]) -> Option<usize> {
        firmware
            .windows(placeholder.len())
            .position(|window| window == placeholder)
    }

    /// Check if firmware contains the placeholder
    pub fn has_placeholder(firmware: &[u8]) -> bool {
        Self::find_placeholder(firmware, DEFAULT_PLACEHOLDER.as_bytes()).is_some()
    }

    /// Get the maximum allowed device name length
    pub fn max_name_length() -> usize {
        DEFAULT_PLACEHOLDER.len()
    }
}