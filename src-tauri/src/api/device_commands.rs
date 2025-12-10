use crate::api::helpers::execute_batch_command;
use crate::application::dto::{BatchResultDto, DeviceStateDto};
use crate::application::services::{ClientApkService, DeviceApplicationService};
use crate::domain::commands::{
    ClearWifiCredentialsCommand, CloseAllAppsCommand, ConfigureDeviceCommand,
    DisplayMessageCommand, ExecuteShellCommand, GetInstalledAppsCommand, GetVolumeCommand,
    InstallApkCommand, LaunchAppCommand, PingCommand, RequestBatteryCommand,
    RestartDeviceCommand, SetVolumeCommand, UninstallAppCommand,
};
use crate::domain::models::{DeviceId, PackageName, Serial};
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

/// Get all connected devices
#[tauri::command]
pub async fn get_devices(
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<Vec<DeviceStateDto>, String> {
    let devices = device_service
        .get_all_devices()
        .await
        .map_err(|e| format!("Failed to get devices: {}", e))?;

    Ok(devices.iter().map(DeviceStateDto::from).collect())
}

/// Get a specific device by ID
#[tauri::command]
pub async fn get_device(
    device_id: String,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<Option<DeviceStateDto>, String> {
    let uuid = Uuid::parse_str(&device_id)
        .map_err(|e| format!("Invalid device ID: {}", e))?;
    let device_id = DeviceId::from_uuid(uuid);

    let device = device_service
        .get_device(device_id)
        .await
        .map_err(|e| format!("Failed to get device: {}", e))?;

    Ok(device.as_ref().map(DeviceStateDto::from))
}

/// Set a custom name for a device
#[tauri::command]
pub async fn set_device_name(
    serial: String,
    name: Option<String>,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<(), String> {
    let serial = Serial::new(serial)
        .map_err(|e| format!("Invalid serial number: {}", e))?;

    device_service
        .set_device_name(serial, name)
        .await
        .map_err(|e| format!("Failed to set device name: {}", e))
}

/// Launch an app on multiple devices
#[tauri::command]
pub async fn launch_app(
    device_ids: Vec<String>,
    package_name: String,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    let package_name = PackageName::new(package_name)
        .map_err(|e| format!("Invalid package name: {}", e))?;

    execute_batch_command(
        device_ids,
        &device_service,
        LaunchAppCommand::new(package_name),
    )
    .await
}

/// Execute a shell command on multiple devices
#[tauri::command]
pub async fn execute_shell(
    device_ids: Vec<String>,
    command: String,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    execute_batch_command(
        device_ids,
        &device_service,
        ExecuteShellCommand::new(command),
    )
    .await
}

/// Uninstall an app from multiple devices
#[tauri::command]
pub async fn uninstall_app(
    device_ids: Vec<String>,
    package_name: String,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    let package_name = PackageName::new(package_name)
        .map_err(|e| format!("Invalid package name: {}", e))?;

    execute_batch_command(
        device_ids,
        &device_service,
        UninstallAppCommand::new(package_name),
    )
    .await
}

/// Request battery status from multiple devices
#[tauri::command]
pub async fn request_battery(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    execute_batch_command(device_ids, &device_service, RequestBatteryCommand).await
}

/// Ping multiple devices
#[tauri::command]
pub async fn ping_devices(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    execute_batch_command(device_ids, &device_service, PingCommand).await
}

/// Set volume on multiple devices
#[tauri::command]
pub async fn set_volume(
    device_ids: Vec<String>,
    level: u8,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    let command = SetVolumeCommand::new(level)
        .map_err(|e| format!("Invalid volume level: {}", e))?;

    execute_batch_command(device_ids, &device_service, command).await
}

/// Get volume from multiple devices
#[tauri::command]
pub async fn get_volume(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    execute_batch_command(device_ids, &device_service, GetVolumeCommand).await
}

/// Get installed apps from multiple devices
#[tauri::command]
pub async fn get_installed_apps(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    execute_batch_command(device_ids, &device_service, GetInstalledAppsCommand).await
}

/// Restart multiple devices
#[tauri::command]
pub async fn restart_devices(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    execute_batch_command(device_ids, &device_service, RestartDeviceCommand).await
}

/// Install APK from remote URL on multiple devices
#[tauri::command]
pub async fn install_remote_apk(
    device_ids: Vec<String>,
    url: String,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    execute_batch_command(device_ids, &device_service, InstallApkCommand::new(url)).await
}

/// Install APK from local file on multiple devices
#[tauri::command]
pub async fn install_local_apk(
    device_ids: Vec<String>,
    filename: String,
    apk_service: State<'_, Arc<crate::application::services::ApkApplicationService>>,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    let apks = apk_service
        .list_apks()
        .await
        .map_err(|e| format!("Failed to list APKs: {}", e))?;

    let apk = apks
        .iter()
        .find(|a| a.filename == filename)
        .ok_or_else(|| format!("APK '{}' not found", filename))?;

    tracing::info!(
        filename = %filename,
        url = %apk.url,
        size_bytes = apk.size_bytes,
        "Installing local APK"
    );

    let result = execute_batch_command(
        device_ids,
        &device_service,
        InstallApkCommand::new(apk.url.clone()),
    )
    .await?;

    tracing::info!(
        succeeded = result.success_count,
        failed = result.failure_count,
        "Local APK install batch completed"
    );

    Ok(result)
}

/// Close all running applications on multiple devices
#[tauri::command]
pub async fn close_all_apps(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    execute_batch_command(device_ids, &device_service, CloseAllAppsCommand).await
}

/// Configure device WiFi and server connection settings
#[tauri::command]
pub async fn configure_device(
    device_ids: Vec<String>,
    wifi_ssid: Option<String>,
    wifi_password: Option<String>,
    server_ip: String,
    server_port: u16,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    let command = ConfigureDeviceCommand::new(wifi_ssid, wifi_password, server_ip, server_port)
        .map_err(|e| format!("Invalid configuration: {}", e))?;

    execute_batch_command(device_ids, &device_service, command).await
}

/// Clear WiFi credentials on multiple devices
#[tauri::command]
pub async fn clear_wifi_credentials(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    execute_batch_command(device_ids, &device_service, ClearWifiCredentialsCommand).await
}

/// Display a message notification on multiple devices
#[tauri::command]
pub async fn display_message(
    device_ids: Vec<String>,
    message: String,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    execute_batch_command(
        device_ids,
        &device_service,
        DisplayMessageCommand::new(message),
    )
    .await
}

/// Check for client APK updates and download if available
/// Returns true if an update was downloaded, false if already up to date
#[tauri::command]
pub async fn check_and_update_client_apk(
    client_apk_service: State<'_, Arc<ClientApkService>>,
) -> Result<bool, String> {
    client_apk_service
        .check_and_download_if_needed()
        .await
        .map_err(|e| format!("Failed to check/update client APK: {}", e))
}
