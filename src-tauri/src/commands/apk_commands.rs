use super::helpers::CommandResultExt;
use crate::core::{ApkFile, CommandValidator};
use crate::services::ApkService;
use std::sync::Arc;
use tauri::State;

#[tauri::command]
pub async fn list_apks(
    apk_service: State<'_, Arc<ApkService>>,
) -> std::result::Result<Vec<ApkFile>, String> {
    apk_service.list_apks().await.to_command_result()
}

#[tauri::command]
pub async fn add_apk(
    source_path: String,
    apk_service: State<'_, Arc<ApkService>>,
) -> std::result::Result<(), String> {
    apk_service
        .add_apk(source_path.into())
        .await
        .to_command_result()
}

#[tauri::command]
pub async fn remove_apk(
    filename: String,
    apk_service: State<'_, Arc<ApkService>>,
) -> std::result::Result<(), String> {
    CommandValidator::validate_apk_filename(&filename)?;
    apk_service
        .remove_apk(filename)
        .await
        .to_command_result()
}

#[tauri::command]
pub fn open_apk_folder(apk_service: State<'_, Arc<ApkService>>) -> std::result::Result<(), String> {
    apk_service.open_apk_folder().to_command_result()
}
