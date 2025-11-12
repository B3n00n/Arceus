use crate::domain::commands::RequestBatteryCommand;
use crate::domain::repositories::DeviceRepository;
use crate::domain::services::{CommandExecutor, SessionManager};
use std::sync::Arc;
use std::time::Duration;

/// Background service that periodically polls battery status from connected devices
pub struct BatteryMonitor {
    device_repo: Arc<dyn DeviceRepository>,
    session_manager: Arc<dyn SessionManager>,
    command_executor: Arc<CommandExecutor>,
    interval: Duration,
}

impl BatteryMonitor {
    pub fn new(
        device_repo: Arc<dyn DeviceRepository>,
        session_manager: Arc<dyn SessionManager>,
        command_executor: Arc<CommandExecutor>,
        interval: Duration,
    ) -> Self {
        Self {
            device_repo,
            session_manager,
            command_executor,
            interval,
        }
    }

    pub async fn start(self: Arc<Self>) {
        tracing::info!(
            interval_secs = self.interval.as_secs(),
            "Battery monitor started"
        );

        let mut interval_timer = tokio::time::interval(self.interval);

        loop {
            interval_timer.tick().await;

            if let Err(e) = self.poll_batteries().await {
                tracing::error!(error = %e, "Failed to poll battery status");
            }
        }
    }

    async fn poll_batteries(&self) -> Result<(), Box<dyn std::error::Error>> {
        let devices = self.device_repo.find_all().await?;

        if devices.is_empty() {
            tracing::debug!("No devices to poll battery status");
            return Ok(());
        }

        let device_ids: Vec<_> = devices.iter()
            .map(|d| d.id())
            .filter(|id| self.session_manager.has_session(id))
            .collect();

        if device_ids.is_empty() {
            tracing::debug!("No connected devices to poll battery status");
            return Ok(());
        }

        let count = device_ids.len();
        tracing::debug!(count, "Polling battery status from connected devices");

        let command = Arc::new(RequestBatteryCommand);
        let result = self
            .command_executor
            .execute_batch(device_ids, command)
            .await;

        tracing::debug!(
            succeeded = result.success_count(),
            failed = result.failure_count(),
            "Battery poll completed"
        );

        for (device_id, error) in result.failed {
            tracing::warn!(
                device_id = %device_id,
                error = %error,
                "Failed to poll battery status"
            );
        }

        Ok(())
    }
}
