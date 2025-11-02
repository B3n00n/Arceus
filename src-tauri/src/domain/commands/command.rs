use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandResponse {
    /// Command executed successfully with no data
    Success,
    /// Command executed successfully with response data
    SuccessWithData(Vec<u8>),
    /// Command is pending (waiting for device response)
    Pending,
}

/// Batch execution result
/// Tracks success and failure for multiple device operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchResult<T> {
    pub succeeded: Vec<(crate::domain::models::DeviceId, T)>,
    pub failed: Vec<(crate::domain::models::DeviceId, String)>,
}

impl<T> BatchResult<T> {
    pub fn new() -> Self {
        Self {
            succeeded: Vec::new(),
            failed: Vec::new(),
        }
    }

    /// Add a successful result
    pub fn add_success(&mut self, id: crate::domain::models::DeviceId, result: T) {
        self.succeeded.push((id, result));
    }

    /// Add a failed result
    pub fn add_failure(&mut self, id: crate::domain::models::DeviceId, error: String) {
        self.failed.push((id, error));
    }

    /// Get the number of successful operations
    pub fn success_count(&self) -> usize {
        self.succeeded.len()
    }

    /// Get the number of failed operations
    pub fn failure_count(&self) -> usize {
        self.failed.len()
    }

    /// Get the total number of operations
    pub fn total_count(&self) -> usize {
        self.success_count() + self.failure_count()
    }

    /// Calculate the success rate (0.0 to 1.0)
    pub fn success_rate(&self) -> f64 {
        let total = self.total_count();
        if total == 0 {
            0.0
        } else {
            self.success_count() as f64 / total as f64
        }
    }
}

impl<T> Default for BatchResult<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
pub trait Command: Send + Sync + Debug {
    fn opcode(&self) -> u8;
    fn name(&self) -> &'static str;
    fn serialize(&self) -> Result<Vec<u8>, std::io::Error>;
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
