use crate::core::models::update::UpdateStatus;
use crate::services::update_service::UpdateService;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn check_for_updates(
    update_service: State<'_, Arc<Mutex<UpdateService>>>,
) -> Result<UpdateStatus, String> {
    let service = update_service.lock().await;
    service.check_for_updates().await
}

#[tauri::command]
pub async fn download_and_install_update(
    update_service: State<'_, Arc<Mutex<UpdateService>>>,
) -> Result<(), String> {
    let service = update_service.lock().await;
    service.download_and_install().await
}

#[tauri::command]
pub async fn skip_update() -> Result<(), String> {
    Ok(())
}