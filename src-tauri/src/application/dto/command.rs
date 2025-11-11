use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Command execution result DTO for frontend
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CommandResultDto {
    pub timestamp: DateTime<Utc>,
    pub command_type: String,
    pub success: bool,
    pub message: String,
}

impl CommandResultDto {
    pub fn success(command_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            command_type: command_type.into(),
            success: true,
            message: message.into(),
        }
    }

    pub fn failure(command_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            command_type: command_type.into(),
            success: false,
            message: message.into(),
        }
    }
}
