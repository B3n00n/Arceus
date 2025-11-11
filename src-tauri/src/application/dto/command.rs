use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::commands::BatchResult;

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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchResultDto {
    pub success_count: usize,
    pub failure_count: usize,
    pub total_count: usize,
    pub success_rate: f64,
    pub succeeded: Vec<String>,
    pub failed: Vec<FailedDeviceDto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FailedDeviceDto {
    pub device_id: String,
    pub error_message: String,
    pub error_code: String,
    pub is_retriable: bool,
}

impl<T> From<BatchResult<T>> for BatchResultDto {
    fn from(result: BatchResult<T>) -> Self {
        BatchResultDto {
            success_count: result.success_count(),
            failure_count: result.failure_count(),
            total_count: result.total_count(),
            success_rate: result.success_rate(),
            succeeded: result
                .succeeded
                .iter()
                .map(|(id, _)| id.as_uuid().to_string())
                .collect(),
            failed: result
                .failed
                .iter()
                .map(|(id, err)| FailedDeviceDto {
                    device_id: id.as_uuid().to_string(),
                    error_message: err.clone(),
                    error_code: "COMMAND_FAILED".to_string(),
                    is_retriable: false,
                })
                .collect(),
        }
    }
}
