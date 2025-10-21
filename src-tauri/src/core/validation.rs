use super::error::{ArceusError, HandlerError};

/// Trait for types that can validate themselves
pub trait Validatable {
    fn validate(&self) -> Result<(), ArceusError>;
}

/// Command parameter validation utilities
pub struct CommandValidator;

impl CommandValidator {
    /// Validate volume level (Meta Quest supports 0-15)
    pub fn validate_volume_level(level: u8) -> Result<(), String> {
        const MAX_VOLUME: u8 = 15;
        if level > MAX_VOLUME {
            return Err(format!(
                "Volume level {} out of range (0-{})",
                level, MAX_VOLUME
            ));
        }
        Ok(())
    }

    /// Validate Android package name format
    pub fn validate_package_name(name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err("Package name cannot be empty".to_string());
        }
        if !name.contains('.') {
            return Err(format!(
                "Invalid package name format: '{}'. Expected format: com.example.app",
                name
            ));
        }
        // Android package names must be lowercase and contain only alphanumeric, dots, and underscores
        if !name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '.' || c == '_')
        {
            return Err(format!(
                "Invalid package name: '{}'. Must contain only lowercase letters, digits, dots, and underscores",
                name
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_volume() {
        assert!(CommandValidator::validate_volume_level(0).is_ok());
        assert!(CommandValidator::validate_volume_level(15).is_ok());
    }

    #[test]
    fn test_invalid_volume() {
        assert!(CommandValidator::validate_volume_level(16).is_err());
        assert!(CommandValidator::validate_volume_level(255).is_err());
    }

    #[test]
    fn test_valid_package_name() {
        assert!(CommandValidator::validate_package_name("com.example.app").is_ok());
        assert!(CommandValidator::validate_package_name("com.oculus.questgame").is_ok());
        assert!(CommandValidator::validate_package_name("com.beat_games.beatsaber").is_ok());
    }

    #[test]
    fn test_invalid_package_name() {
        assert!(CommandValidator::validate_package_name("").is_err());
        assert!(CommandValidator::validate_package_name("invalid").is_err());
        assert!(CommandValidator::validate_package_name("Com.Example.App").is_err()); // uppercase
        assert!(CommandValidator::validate_package_name("com.example app").is_err()); // space
    }

    #[test]
    fn test_valid_shell_command() {
        assert!(CommandValidator::validate_shell_command("ls -la").is_ok());
        assert!(CommandValidator::validate_shell_command("pm list packages").is_ok());
    }

    #[test]
    fn test_invalid_shell_command() {
        assert!(CommandValidator::validate_shell_command("").is_err());
        assert!(CommandValidator::validate_shell_command("   ").is_err());
    }

    #[test]
    fn test_valid_apk_url() {
        assert!(CommandValidator::validate_apk_url("http://example.com/app.apk").is_ok());
        assert!(CommandValidator::validate_apk_url("https://example.com/app.apk").is_ok());
    }

    #[test]
    fn test_invalid_apk_url() {
        assert!(CommandValidator::validate_apk_url("").is_err());
        assert!(CommandValidator::validate_apk_url("ftp://example.com/app.apk").is_err());
        assert!(CommandValidator::validate_apk_url("example.com/app.apk").is_err());
    }

    #[test]
    fn test_valid_apk_filename() {
        assert!(CommandValidator::validate_apk_filename("app.apk").is_ok());
        assert!(CommandValidator::validate_apk_filename("my-game.apk").is_ok());
    }

    #[test]
    fn test_invalid_apk_filename() {
        assert!(CommandValidator::validate_apk_filename("").is_err());
        assert!(CommandValidator::validate_apk_filename("app.zip").is_err());
        assert!(CommandValidator::validate_apk_filename("../app.apk").is_err());
        assert!(CommandValidator::validate_apk_filename("/etc/app.apk").is_err());
        assert!(CommandValidator::validate_apk_filename("path\\to\\app.apk").is_err());
    }
}
