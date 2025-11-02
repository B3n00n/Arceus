use crate::application::services::ApkApplicationService;
use crate::core::ApkFile;
use std::sync::Arc;
use tauri::State;

/// List all APK files
#[tauri::command]
pub async fn list_apks(
    apk_service: State<'_, Arc<ApkApplicationService>>,
) -> Result<Vec<ApkFile>, String> {
    let apk_infos = apk_service
        .list_apks()
        .await
        .map_err(|e| format!("Failed to list APKs: {}", e))?;

    // Convert ApkInfo to ApkFile
    let apk_files = apk_infos
        .into_iter()
        .map(|info| ApkFile::new(info.filename, info.size_bytes, info.url))
        .collect();

    Ok(apk_files)
}

/// Add an APK file from a source path
#[tauri::command]
pub async fn add_apk(
    source_path: String,
    apk_service: State<'_, Arc<ApkApplicationService>>,
) -> Result<(), String> {
    let _filename = apk_service
        .add_apk(source_path.into())
        .await
        .map_err(|e| format!("Failed to add APK: {}", e))?;

    Ok(())
}

/// Remove an APK file
#[tauri::command]
pub async fn remove_apk(
    filename: String,
    apk_service: State<'_, Arc<ApkApplicationService>>,
) -> Result<(), String> {
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return Err("Invalid filename: path traversal not allowed".to_string());
    }

    apk_service
        .remove_apk(&filename)
        .await
        .map_err(|e| format!("Failed to remove APK: {}", e))?;

    Ok(())
}

/// Open the APK folder in the system file explorer
#[tauri::command]
pub fn open_apk_folder(apk_service: State<'_, Arc<ApkApplicationService>>) -> Result<(), String> {
    apk_service
        .open_apk_folder()
        .map_err(|e| format!("Failed to open APK folder: {}", e))
}
