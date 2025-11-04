use crate::domain::commands::RequestBatteryCommand;
use crate::domain::repositories::DeviceRepository;
use crate::domain::services::CommandExecutor;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;

/// Background service that periodically polls battery status from connected devices
pub struct BatteryMonitor {
    device_repo: Arc<dyn DeviceRepository>,
    command_executor: Arc<CommandExecutor>,
    interval: Duration,
}

impl BatteryMonitor {
    /// Create a new BatteryMonitor
    pub fn new(
        device_repo: Arc<dyn DeviceRepository>,
        command_executor: Arc<CommandExecutor>,
        interval: Duration,
    ) -> Self {
        Self {
            device_repo,
            command_executor,
            interval,
        }
    }

    /// Start the battery monitoring loop
    pub async fn start(self: Arc<Self>, mut shutdown_rx: broadcast::Receiver<()>) {
        tracing::info!(
            interval_secs = self.interval.as_secs(),
            "Battery monitor started"
        );

        let mut interval_timer = tokio::time::interval(self.interval);

        loop {
            tokio::select! {
                _ = interval_timer.tick() => {
                    if let Err(e) = self.poll_batteries().await {
                        tracing::error!(error = %e, "Failed to poll battery status");
                    }
                }
                _ = shutdown_rx.recv() => {
                    tracing::info!("Battery monitor shutdown signal received");
                    break;
                }
            }
        }

        tracing::info!("Battery monitor stopped");
    }

    /// Poll battery status from all devices
    async fn poll_batteries(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get all devices (only connected devices are in the repository)
        let devices = self.device_repo.find_all().await?;

        if devices.is_empty() {
            tracing::debug!("No devices to poll battery status");
            return Ok(());
        }

        let device_ids: Vec<_> = devices.iter().map(|d| d.id()).collect();
        let count = device_ids.len();

        tracing::debug!(count, "Polling battery status from devices");

        // Execute battery request command on all connected devices
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

        // Log any failures
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
