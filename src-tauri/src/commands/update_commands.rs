use crate::core::models::update::UpdateStatus;
use crate::services::update_service::UpdateService;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;

#[tauri::command]
pub async fn check_for_updates(
    update_service: State<'_, Arc<Mutex<UpdateService>>>,
) -> Result<UpdateStatus, String> {
    update_service.lock().await.check_for_updates().await
}

#[tauri::command]
pub async fn download_and_install_update(
    update_service: State<'_, Arc<Mutex<UpdateService>>>,
) -> Result<(), String> {
    update_service.lock().await.download_and_install().await
}

#[tauri::command]
pub async fn skip_update(app: AppHandle) -> Result<(), String> {
    transition_to_main_window(app)
}

#[tauri::command]
pub async fn close_updater_and_show_main(app: AppHandle) -> Result<(), String> {
    transition_to_main_window(app)
}

fn transition_to_main_window(app: AppHandle) -> Result<(), String> {
    if let Some(updater_window) = app.get_webview_window("updater") {
        let _ = updater_window.close();
    }

    if let Some(main_window) = app.get_webview_window("main") {
        let _ = main_window.show();
        let _ = main_window.set_focus();
    }

    Ok(())
}