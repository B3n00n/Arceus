use crate::network::ConnectionManager;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;

pub struct BatteryMonitor {
    connection_manager: Arc<ConnectionManager>,
    interval_duration: Duration,
    running: Arc<tokio::sync::RwLock<bool>>,
}

impl BatteryMonitor {
    pub fn new(connection_manager: Arc<ConnectionManager>, interval_secs: u64) -> Self {
        Self {
            connection_manager,
            interval_duration: Duration::from_secs(interval_secs),
            running: Arc::new(tokio::sync::RwLock::new(false)),
        }
    }

    pub async fn start(self: Arc<Self>) {
        *self.running.write().await = true;
        tracing::info!(
            "Battery monitor started (interval: {:?})",
            self.interval_duration
        );

        let mut interval_timer = interval(self.interval_duration);

        while *self.running.read().await {
            interval_timer.tick().await;

            let devices = self.connection_manager.get_all();
            for device in devices {
                if let Err(e) = device.request_battery().await {
                    tracing::warn!(
                        "Failed to request battery from device {}: {}",
                        device.serial(),
                        e
                    );
                }
            }

            tracing::trace!("Battery status requested from all devices");
        }

        tracing::info!("Battery monitor stopped");
    }

    pub async fn stop(&self) {
        *self.running.write().await = false;
    }
}
