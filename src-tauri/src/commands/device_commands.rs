use super::helpers::{parse_device_ids, CommandResultExt};
use crate::core::{CommandValidator, DeviceState};
use crate::services::{ApkService, DeviceService};
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn get_devices(
    device_service: State<'_, Arc<DeviceService>>,
)->std::result::Result<Vec<DeviceState>, String> {
    Ok(device_service.get_all_devices())
}

#[tauri::command]
pub async fn get_device(
    device_id: String,
    device_service: State<'_, Arc<DeviceService>>,
) -> std::result::Result<Option<DeviceState>, String> {
    let id = Uuid::parse_str(&device_id).to_command_result()?;
    Ok(device_service.get_device(id))
}

#[tauri::command]
pub async fn set_device_name(
    serial: String,
    name: Option<String>,
    device_service: State<'_, Arc<DeviceService>>,
) -> std::result::Result<(), String> {
    device_service
        .set_device_name(serial, name)
        .to_command_result()
}

#[tauri::command]
pub async fn launch_app(
    device_ids: Vec<String>,
    package_name: String,
    device_service: State<'_, Arc<DeviceService>>,
) -> std::result::Result<(), String> {
    CommandValidator::validate_package_name(&package_name)?;
    let ids = parse_device_ids(device_ids)?;
    device_service
        .launch_app(ids, package_name)
        .await
        .to_command_result()
}

#[tauri::command]
pub async fn uninstall_app(
    device_ids: Vec<String>,
    package_name: String,
    device_service: State<'_, Arc<DeviceService>>,
) -> std::result::Result<(), String> {
    CommandValidator::validate_package_name(&package_name)?;
    let ids = parse_device_ids(device_ids)?;
    device_service
        .uninstall_app(ids, package_name)
        .await
        .to_command_result()
}

#[tauri::command]
pub async fn request_battery(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceService>>,
) -> std::result::Result<(), String> {
    let ids = parse_device_ids(device_ids)?;
    device_service
        .request_battery(ids)
        .await
        .to_command_result()
}

#[tauri::command]
pub async fn ping_devices(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceService>>,
) -> std::result::Result<(), String> {
    let ids = parse_device_ids(device_ids)?;
    device_service
        .ping_devices(ids)
        .await
        .to_command_result()
}

#[tauri::command]
pub async fn set_volume(
    device_ids: Vec<String>,
    level: u8,
    device_service: State<'_, Arc<DeviceService>>,
) -> std::result::Result<(), String> {
    CommandValidator::validate_volume_level(level)?;
    let ids = parse_device_ids(device_ids)?;
    device_service
        .set_volume(ids, level)
        .await
        .to_command_result()
}

#[tauri::command]
pub async fn get_volume(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceService>>,
) -> std::result::Result<(), String> {
    let ids = parse_device_ids(device_ids)?;
    device_service
        .get_volume(ids)
        .await
        .to_command_result()
}

#[tauri::command]
pub async fn execute_shell(
    device_ids: Vec<String>,
    command: String,
    device_service: State<'_, Arc<DeviceService>>,
) -> std::result::Result<(), String> {
    CommandValidator::validate_shell_command(&command)?;
    let ids = parse_device_ids(device_ids)?;
    device_service
        .execute_shell(ids, command)
        .await
        .to_command_result()
}

#[tauri::command]
pub async fn get_installed_apps(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceService>>,
) -> std::result::Result<(), String> {
    let ids = parse_device_ids(device_ids)?;
    device_service
        .get_installed_apps(ids)
        .await
        .to_command_result()
}

#[tauri::command]
pub async fn install_remote_apk(
    device_ids: Vec<String>,
    url: String,
    device_service: State<'_, Arc<DeviceService>>,
) -> std::result::Result<(), String> {
    CommandValidator::validate_apk_url(&url)?;
    let ids = parse_device_ids(device_ids)?;
    device_service
        .install_remote_apk(ids, url)
        .await
        .to_command_result()
}

#[tauri::command]
pub async fn install_local_apk(
    device_ids: Vec<String>,
    filename: String,
    device_service: State<'_, Arc<DeviceService>>,
    apk_service: State<'_, Arc<ApkService>>,
) -> std::result::Result<(), String> {
    CommandValidator::validate_apk_filename(&filename)?;
    let ids = parse_device_ids(device_ids)?;
    let url = apk_service.get_http_server().get_apk_url(&filename);
    device_service
        .install_local_apk(ids, url)
        .await
        .to_command_result()
}

#[tauri::command]
pub async fn restart_devices(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceService>>,
) -> std::result::Result<(), String> {
    let ids = parse_device_ids(device_ids)?;
    device_service
        .restart_devices(ids)
        .await
        .to_command_result()
}
