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