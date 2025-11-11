use crate::application::dto::BatchResultDto;
use crate::application::services::DeviceApplicationService;
use crate::domain::commands::Command;
use crate::domain::models::DeviceId;
use std::sync::Arc;
use uuid::Uuid;

pub fn parse_device_ids(ids: Vec<String>) -> Result<Vec<DeviceId>, String> {
    ids.iter()
        .map(|s| {
            Uuid::parse_str(s)
                .map(DeviceId::from_uuid)
                .map_err(|e| format!("Invalid device ID '{}': {}", s, e))
        })
        .collect()
}

pub async fn execute_batch_command<C>(
    device_ids: Vec<String>,
    device_service: &Arc<DeviceApplicationService>,
    command: C,
) -> Result<BatchResultDto, String>
where
    C: Command + 'static,
{
    let ids = parse_device_ids(device_ids)?;
    let result = device_service
        .execute_command_batch(ids, Arc::new(command))
        .await;
    Ok(result.into())
}
