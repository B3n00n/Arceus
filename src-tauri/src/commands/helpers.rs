use uuid::Uuid;

/// Parse device ID strings into UUIDs with proper error handling and logging
pub fn parse_device_ids(device_ids: Vec<String>) -> std::result::Result<Vec<Uuid>, String> {
    device_ids
        .iter()
        .map(|s| {
            Uuid::parse_str(s).map_err(|e| {
                let msg = format!("Invalid device ID '{}': {}", s, e);
                tracing::warn!("{}", msg);
                msg
            })
        })
        .collect()
}

/// Extension trait for converting service Results to command-friendly String errors
/// with automatic logging
pub trait CommandResultExt<T> {
    fn to_command_result(self) -> std::result::Result<T, String>;
}

impl<T, E: std::fmt::Display> CommandResultExt<T> for std::result::Result<T, E> {
    fn to_command_result(self) -> std::result::Result<T, String> {
        self.map_err(|e| {
            let msg = e.to_string();
            tracing::error!("Command error: {}", msg);
            msg
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_device_ids() {
        let valid_ids = vec![
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8".to_string(),
        ];
        let result = parse_device_ids(valid_ids);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }

    #[test]
    fn test_parse_invalid_device_id() {
        let invalid_ids = vec!["not-a-uuid".to_string()];
        let result = parse_device_ids(invalid_ids);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid device ID"));
    }

    #[test]
    fn test_parse_mixed_device_ids() {
        let mixed_ids = vec![
            "550e8400-e29b-41d4-a716-446655440000".to_string(),
            "invalid".to_string(),
        ];
        let result = parse_device_ids(mixed_ids);
        assert!(result.is_err()); // Should fail on first invalid ID
    }

    #[test]
    fn test_parse_empty_device_ids() {
        let empty_ids: Vec<String> = vec![];
        let result = parse_device_ids(empty_ids);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
