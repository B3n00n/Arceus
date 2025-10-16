use crate::core::{DeviceState};
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
)->std::result::Result<Option<DeviceState>, String> {
    let id = Uuid::parse_str(&device_id).map_err(|e| e.to_string())?;
    Ok(device_service.get_device(id))
}

#[tauri::command]
pub async fn set_device_name(
    serial: String,
    name: Option<String>,
    device_service: State<'_, Arc<DeviceService>>,
)->std::result::Result<(), String> {
    device_service
        .set_device_name(serial, name)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn launch_app(
    device_ids: Vec<String>,
    package_name: String,
    device_service: State<'_, Arc<DeviceService>>,
)->std::result::Result<(), String> {
    let ids: std::result::Result<Vec<Uuid>, _> = device_ids
        .iter()
        .map(|s| Uuid::parse_str(s))
        .collect();
    let ids = ids.map_err(|e| e.to_string())?;

    device_service
        .launch_app(ids, package_name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn uninstall_app(
    device_ids: Vec<String>,
    package_name: String,
    device_service: State<'_, Arc<DeviceService>>,
)->std::result::Result<(), String> {
    let ids: std::result::Result<Vec<Uuid>, _> = device_ids.iter().map(|s| Uuid::parse_str(s)).collect();
    let ids = ids.map_err(|e| e.to_string())?;

    device_service
        .uninstall_app(ids, package_name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn request_battery(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceService>>,
)->std::result::Result<(), String> {
    let ids: std::result::Result<Vec<Uuid>, _> = device_ids.iter().map(|s| Uuid::parse_str(s)).collect();
    let ids = ids.map_err(|e| e.to_string())?;

    device_service
        .request_battery(ids)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ping_devices(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceService>>,
)->std::result::Result<(), String> {
    tracing::info!("ping_devices command called with {} device(s)", device_ids.len());

    let ids: std::result::Result<Vec<Uuid>, _> = device_ids.iter().map(|s| Uuid::parse_str(s)).collect();
    let ids = ids.map_err(|e| {
        tracing::error!("Failed to parse device IDs: {}", e);
        e.to_string()
    })?;

    tracing::info!("Parsed device IDs: {:?}", ids);

    device_service
        .ping_devices(ids)
        .await
        .map_err(|e| {
            tracing::error!("ping_devices failed: {}", e);
            e.to_string()
        })
}

#[tauri::command]
pub async fn set_volume(
    device_ids: Vec<String>,
    level: u8,
    device_service: State<'_, Arc<DeviceService>>,
)->std::result::Result<(), String> {
    let ids: std::result::Result<Vec<Uuid>, _> = device_ids.iter().map(|s| Uuid::parse_str(s)).collect();
    let ids = ids.map_err(|e| e.to_string())?;

    device_service
        .set_volume(ids, level)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_volume(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceService>>,
)->std::result::Result<(), String> {
    let ids: std::result::Result<Vec<Uuid>, _> = device_ids.iter().map(|s| Uuid::parse_str(s)).collect();
    let ids = ids.map_err(|e| e.to_string())?;

    device_service
        .get_volume(ids)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn execute_shell(
    device_ids: Vec<String>,
    command: String,
    device_service: State<'_, Arc<DeviceService>>,
)->std::result::Result<(), String> {
    let ids: std::result::Result<Vec<Uuid>, _> = device_ids.iter().map(|s| Uuid::parse_str(s)).collect();
    let ids = ids.map_err(|e| e.to_string())?;

    device_service
        .execute_shell(ids, command)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_installed_apps(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceService>>,
)->std::result::Result<(), String> {
    let ids: std::result::Result<Vec<Uuid>, _> = device_ids.iter().map(|s| Uuid::parse_str(s)).collect();
    let ids = ids.map_err(|e| e.to_string())?;

    device_service
        .get_installed_apps(ids)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn install_remote_apk(
    device_ids: Vec<String>,
    url: String,
    device_service: State<'_, Arc<DeviceService>>,
)->std::result::Result<(), String> {
    let ids: std::result::Result<Vec<Uuid>, _> = device_ids.iter().map(|s| Uuid::parse_str(s)).collect();
    let ids = ids.map_err(|e| e.to_string())?;

    device_service
        .install_remote_apk(ids, url)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn install_local_apk(
    device_ids: Vec<String>,
    filename: String,
    device_service: State<'_, Arc<DeviceService>>,
    apk_service: State<'_, Arc<ApkService>>,
)->std::result::Result<(), String> {
    let ids: std::result::Result<Vec<Uuid>, _> = device_ids.iter().map(|s| Uuid::parse_str(s)).collect();
    let ids = ids.map_err(|e| e.to_string())?;

    let url = apk_service.get_http_server().get_apk_url(&filename);

    device_service
        .install_local_apk(ids, url)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn shutdown_devices(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceService>>,
)->std::result::Result<(), String> {
    let ids: std::result::Result<Vec<Uuid>, _> = device_ids.iter().map(|s| Uuid::parse_str(s)).collect();
    let ids = ids.map_err(|e| e.to_string())?;

    device_service
        .shutdown_devices(ids)
        .await
        .map_err(|e| e.to_string())
}
