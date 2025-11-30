use serde::{Deserialize, Serialize};

/// Progress information for device operations (download/install)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OperationProgressDto {
    pub operation_type: OperationType,
    pub operation_id: String,
    pub stage: OperationStage,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OperationType {
    Download,
    Install,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OperationStage {
    Started,
    InProgress,
    Completed,
    Failed,
}

impl OperationProgressDto {
    pub fn new(
        operation_type: OperationType,
        operation_id: String,
        stage: OperationStage,
        percentage: f32,
    ) -> Self {
        Self {
            operation_type,
            operation_id,
            stage,
            percentage,
        }
    }
}
