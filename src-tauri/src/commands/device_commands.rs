use crate::application::services::DeviceApplicationService;
use crate::core::models::device::{DeviceInfo, DeviceState};
use crate::domain::commands::{
    ExecuteShellCommand, GetInstalledAppsCommand, GetVolumeCommand, InstallApkCommand,
    LaunchAppCommand, PingCommand, RequestBatteryCommand, RestartDeviceCommand,
    SetVolumeCommand, UninstallAppCommand,
};
use crate::domain::models::{DeviceId, PackageName, Serial};
use serde::Serialize;
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

/// Helper to parse UUID strings to DeviceIds
fn parse_device_ids(ids: Vec<String>) -> Result<Vec<DeviceId>, String> {
    ids.iter()
        .map(|s| {
            Uuid::parse_str(s)
                .map(DeviceId::from_uuid)
                .map_err(|e| format!("Invalid device ID '{}': {}", s, e))
        })
        .collect()
}

/// Helper to convert domain Device to DeviceState for frontend compatibility
fn convert_device_to_state(device: &crate::domain::models::Device) -> DeviceState {
    let info = DeviceInfo {
        id: device.id().as_uuid().clone(),
        model: device.model().to_string(),
        serial: device.serial().as_str().to_string(),
        ip: device.ip().as_str().to_string(),
        connected_at: device.connected_at(),
        last_seen: device.last_seen(),
        custom_name: device.custom_name().map(|s| s.to_string()),
    };

    let battery = device.battery().map(|b| crate::core::models::battery::BatteryInfo {
        headset_level: b.level(),
        is_charging: b.is_charging(),
        last_updated: b.last_updated(),
    });

    let volume = device.volume().map(|v| {
        crate::core::models::volume::VolumeInfo::new(
            v.percentage(),
            v.current(),
            v.max(),
        )
    });

    DeviceState {
        info,
        battery,
        volume,
        command_history: std::collections::VecDeque::new(),
        is_connected: device.is_connected(),
    }
}

/// Helper to convert BatchResult to BatchResultDto for frontend
fn convert_batch_result_to_dto<T>(result: crate::domain::commands::BatchResult<T>) -> BatchResultDto {
    BatchResultDto {
        success_count: result.success_count(),
        failure_count: result.failure_count(),
        total_count: result.total_count(),
        success_rate: result.success_rate(),
        succeeded: result
            .succeeded
            .iter()
            .map(|(id, _)| id.as_uuid().to_string())
            .collect(),
        failed: result
            .failed
            .iter()
            .map(|(id, err)| FailedDeviceDto {
                device_id: id.as_uuid().to_string(),
                error_message: err.clone(),
                error_code: "COMMAND_FAILED".to_string(),
                is_retriable: false,
            })
            .collect(),
    }
}

/// DTO for batch command results
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchResultDto {
    pub success_count: usize,
    pub failure_count: usize,
    pub total_count: usize,
    pub success_rate: f64,
    pub succeeded: Vec<String>,
    pub failed: Vec<FailedDeviceDto>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FailedDeviceDto {
    pub device_id: String,
    pub error_message: String,
    pub error_code: String,
    pub is_retriable: bool,
}

/// Get all connected devices
#[tauri::command]
pub async fn get_devices(
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<Vec<DeviceState>, String> {
    let devices = device_service
        .get_all_devices()
        .await
        .map_err(|e| format!("Failed to get devices: {}", e))?;

    // Convert domain Device to DeviceState for frontend compatibility
    let device_states: Vec<DeviceState> = devices
        .iter()
        .map(convert_device_to_state)
        .collect();

    Ok(device_states)
}

/// Get a specific device by ID
#[tauri::command]
pub async fn get_device(
    device_id: String,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<Option<DeviceState>, String> {
    let uuid = Uuid::parse_str(&device_id)
        .map_err(|e| format!("Invalid device ID: {}", e))?;
    let device_id = DeviceId::from_uuid(uuid);

    let device = device_service
        .get_device(device_id)
        .await
        .map_err(|e| format!("Failed to get device: {}", e))?;

    Ok(device.as_ref().map(convert_device_to_state))
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
    let ids = parse_device_ids(device_ids)?;
    let package_name = PackageName::new(package_name)
        .map_err(|e| format!("Invalid package name: {}", e))?;

    let command = LaunchAppCommand::new(package_name);
    let result = device_service
        .execute_command_batch(ids, Arc::new(command))
        .await;

    Ok(convert_batch_result_to_dto(result))
}

/// Execute a shell command on multiple devices
#[tauri::command]
pub async fn execute_shell(
    device_ids: Vec<String>,
    command: String,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    let ids = parse_device_ids(device_ids)?;
    let shell_command = ExecuteShellCommand::new(command);

    let result = device_service
        .execute_command_batch(ids, Arc::new(shell_command))
        .await;

    Ok(convert_batch_result_to_dto(result))
}

/// Uninstall an app from multiple devices
#[tauri::command]
pub async fn uninstall_app(
    device_ids: Vec<String>,
    package_name: String,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    let ids = parse_device_ids(device_ids)?;
    let package_name = PackageName::new(package_name)
        .map_err(|e| format!("Invalid package name: {}", e))?;

    let command = UninstallAppCommand::new(package_name);
    let result = device_service
        .execute_command_batch(ids, Arc::new(command))
        .await;

    Ok(convert_batch_result_to_dto(result))
}

/// Request battery status from multiple devices
#[tauri::command]
pub async fn request_battery(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    let ids = parse_device_ids(device_ids)?;
    let command = RequestBatteryCommand;

    let result = device_service
        .execute_command_batch(ids, Arc::new(command))
        .await;

    Ok(convert_batch_result_to_dto(result))
}

/// Ping multiple devices
#[tauri::command]
pub async fn ping_devices(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    let ids = parse_device_ids(device_ids)?;
    let command = PingCommand;

    let result = device_service
        .execute_command_batch(ids, Arc::new(command))
        .await;

    Ok(convert_batch_result_to_dto(result))
}

/// Set volume on multiple devices
#[tauri::command]
pub async fn set_volume(
    device_ids: Vec<String>,
    level: u8,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    if level > 100 {
        return Err(format!("Volume level must be 0-100, got {}", level));
    }

    let ids = parse_device_ids(device_ids)?;
    let command = SetVolumeCommand::new(level)
        .map_err(|e| format!("Invalid volume level: {}", e))?;

    let result = device_service
        .execute_command_batch(ids, Arc::new(command))
        .await;

    Ok(convert_batch_result_to_dto(result))
}

/// Get volume from multiple devices
#[tauri::command]
pub async fn get_volume(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    let ids = parse_device_ids(device_ids)?;
    let command = GetVolumeCommand;

    let result = device_service
        .execute_command_batch(ids, Arc::new(command))
        .await;

    Ok(convert_batch_result_to_dto(result))
}

/// Get installed apps from multiple devices
#[tauri::command]
pub async fn get_installed_apps(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    let ids = parse_device_ids(device_ids)?;
    let command = GetInstalledAppsCommand;

    let result = device_service
        .execute_command_batch(ids, Arc::new(command))
        .await;

    Ok(convert_batch_result_to_dto(result))
}

/// Restart multiple devices
#[tauri::command]
pub async fn restart_devices(
    device_ids: Vec<String>,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    let ids = parse_device_ids(device_ids)?;
    let command = RestartDeviceCommand;

    let result = device_service
        .execute_command_batch(ids, Arc::new(command))
        .await;

    Ok(convert_batch_result_to_dto(result))
}

/// Install APK from remote URL on multiple devices
#[tauri::command]
pub async fn install_remote_apk(
    device_ids: Vec<String>,
    url: String,
    device_service: State<'_, Arc<DeviceApplicationService>>,
) -> Result<BatchResultDto, String> {
    let ids = parse_device_ids(device_ids)?;
    let command = InstallApkCommand::new(url);

    let result = device_service
        .execute_command_batch(ids, Arc::new(command))
        .await;

    Ok(convert_batch_result_to_dto(result))
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

    let ids = parse_device_ids(device_ids)?;
    let command = InstallApkCommand::new(apk.url.clone());

    let result = device_service
        .execute_command_batch(ids, Arc::new(command))
        .await;

    tracing::info!(
        succeeded = result.success_count(),
        failed = result.failure_count(),
        "Local APK install batch completed"
    );

    Ok(convert_batch_result_to_dto(result))
}
