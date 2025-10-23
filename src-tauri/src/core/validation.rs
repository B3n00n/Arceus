use super::error::{ArceusError, HandlerError};

/// Trait for types that can validate themselves
pub trait Validatable {
    fn validate(&self) -> Result<(), ArceusError>;
}

/// Command parameter validation utilities
pub struct CommandValidator;

impl CommandValidator {
    /// Validate volume level (Meta Quest supports 0-100)
    pub fn validate_volume_level(level: u8) -> Result<(), String> {
        const MAX_VOLUME: u8 = 100;
        if level > MAX_VOLUME {
            return Err(format!(
                "Volume level {} out of range (0-{})",
                level, MAX_VOLUME
            ));
        }
        Ok(())
    }

    /// Validate shell command (basic safety checks)
    pub fn validate_shell_command(cmd: &str) -> Result<(), String> {
        if cmd.trim().is_empty() {
            return Err("Shell command cannot be empty".to_string());
        }
        // Warn about potentially dangerous commands (but don't block them)
        let dangerous_patterns = ["rm -rf", "dd if=", "mkfs", "format"];
        for pattern in dangerous_patterns {
            if cmd.contains(pattern) {
                tracing::warn!(
                    "Potentially dangerous command detected: '{}' contains '{}'",
                    cmd,
                    pattern
                );
            }
        }
        Ok(())
    }

    /// Validate URL format for APK installation
    pub fn validate_apk_url(url: &str) -> Result<(), String> {
        if url.is_empty() {
            return Err("APK URL cannot be empty".to_string());
        }
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(format!("Invalid URL: '{}'. Must start with http:// or https://", url));
        }
        if !url.ends_with(".apk") {
            tracing::warn!("URL '{}' does not end with .apk extension", url);
        }
        Ok(())
    }

    /// Validate APK filename
    pub fn validate_apk_filename(filename: &str) -> Result<(), String> {
        if filename.is_empty() {
            return Err("APK filename cannot be empty".to_string());
        }
        if !filename.ends_with(".apk") {
            return Err(format!("Invalid filename: '{}'. Must end with .apk", filename));
        }
        // Check for path traversal attempts
        if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
            return Err(format!(
                "Invalid filename: '{}'. Cannot contain path separators or '..'",
                filename
            ));
        }
        Ok(())
    }
}

// Implement Validatable for domain models
impl Validatable for crate::core::VolumeInfo {
    fn validate(&self) -> Result<(), ArceusError> {
        if self.volume_percentage > 100 {
            return Err(HandlerError::InvalidPayload(format!(
                "Volume percentage {} > 100",
                self.volume_percentage
            ))
            .into());
        }
        if self.max_volume > 0 && self.current_volume > self.max_volume {
            return Err(HandlerError::InvalidPayload(format!(
                "Current volume {} > max volume {}",
                self.current_volume, self.max_volume
            ))
            .into());
        }
        Ok(())
    }
}

impl Validatable for crate::core::BatteryInfo {
    fn validate(&self) -> Result<(), ArceusError> {
        if self.headset_level > 100 {
            return Err(HandlerError::InvalidPayload(format!(
                "Battery level {} > 100",
                self.headset_level
            ))
            .into());
        }
        Ok(())
    }
}