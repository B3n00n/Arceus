use crate::core::models::update::{UpdateInfo, UpdateProgress, UpdateStatus};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tauri_plugin_updater::{Update, UpdaterExt};
use tokio::sync::Mutex;

const USER_AGENT: &str = "arceus-updater/1.0";
const GITHUB_ACCEPT_HEADER: &str = "application/vnd.github.v3+json";

pub struct UpdateService {
    app_handle: AppHandle,
    current_update: Arc<Mutex<Option<Update>>>,
}

impl UpdateService {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            current_update: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn check_for_updates(&self) -> Result<UpdateStatus, String> {
        self.emit_status(UpdateStatus::Checking);

        let updater = self
            .app_handle
            .updater_builder()
            .header("User-Agent", USER_AGENT)
            .map_err(|e| format!("Failed to set User-Agent header: {}", e))?
            .header("Accept", GITHUB_ACCEPT_HEADER)
            .map_err(|e| format!("Failed to set Accept header: {}", e))?
            .build()
            .map_err(|e| format!("Failed to initialize updater: {}", e))?;

        match updater.check().await {
            Ok(Some(update)) => {
                let update_info = UpdateInfo {
                    version: update.version.to_string(),
                    current_version: update.current_version.to_string(),
                    body: update.body.clone(),
                    date: update.date.map(|d| d.to_string()),
                    is_available: true,
                };

                *self.current_update.lock().await = Some(update);

                let status = UpdateStatus::UpdateAvailable(update_info);
                self.emit_status(status.clone());
                Ok(status)
            }
            Ok(None) => Ok(UpdateStatus::NoUpdate),
            Err(e) => {
                let error_msg = format!("Failed to check for updates: {}", e);
                let status = UpdateStatus::Error {
                    message: error_msg.clone(),
                };
                self.emit_status(status.clone());
                Err(error_msg)
            }
        }
    }

    pub async fn download_and_install(&self) -> Result<(), String> {
        let update = self
            .current_update
            .lock()
            .await
            .take()
            .ok_or_else(|| "No update available to download".to_string())?;

        let app_handle = self.app_handle.clone();
        let bytes_downloaded = Arc::new(Mutex::new(0u64));
        let bytes_downloaded_clone = bytes_downloaded.clone();

        self.emit_status(UpdateStatus::Installing);

        update
            .download_and_install(
                move |chunk_len, content_len| {
                    let bytes_handle = bytes_downloaded_clone.clone();
                    let app = app_handle.clone();

                    tauri::async_runtime::spawn(async move {
                        let mut downloaded = bytes_handle.lock().await;
                        *downloaded += chunk_len as u64;

                        let progress =
                            UpdateProgress::new(chunk_len as u64, content_len, *downloaded);

                        let status = UpdateStatus::Downloading {
                            progress: progress.percentage.unwrap_or(0.0),
                            bytes_downloaded: *downloaded,
                            total_bytes: content_len.unwrap_or(0),
                        };

                        let _ = app.emit("update-status", &status);
                    });
                },
                Default::default,
            )
            .await
            .map_err(|e| {
                let error_msg = format!("Failed to download and install update: {}", e);
                self.emit_status(UpdateStatus::Error {
                    message: error_msg.clone(),
                });
                error_msg
            })?;

        self.emit_status(UpdateStatus::Complete);
        let _ = self.app_handle.restart();
    }

    fn emit_status(&self, status: UpdateStatus) {
        let _ = self.app_handle.emit("update-status", &status);
    }
}

pub fn create_update_service(app_handle: AppHandle) -> Arc<Mutex<UpdateService>> {
    Arc::new(Mutex::new(UpdateService::new(app_handle)))
}