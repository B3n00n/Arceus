use crate::core::{ApkFile};
use crate::services::ApkService;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn list_apks(
    apk_service: State<'_, Arc<ApkService>>,
)->std::result::Result<Vec<ApkFile>, String> {
    apk_service.list_apks().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_apk(
    source_path: String,
    apk_service: State<'_, Arc<ApkService>>,
)->std::result::Result<(), String> {
    apk_service
        .add_apk(source_path.into())
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_apk(
    filename: String,
    apk_service: State<'_, Arc<ApkService>>,
)->std::result::Result<(), String> {
    apk_service
        .remove_apk(filename)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_apk_folder(apk_service: State<'_, Arc<ApkService>>) -> std::result::Result<(), String> {
    apk_service.open_apk_folder().map_err(|e| e.to_string())
}
