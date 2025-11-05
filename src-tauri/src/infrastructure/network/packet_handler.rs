/// Packet handler system for processing incoming device packets
/// Handlers update the device repository based on received packets.

use crate::core::{models::CommandResult, EventBus};
use crate::domain::models::{Battery, Device, DeviceId, Serial, Volume};
use crate::domain::repositories::{DeviceNameRepository, DeviceRepository};
use crate::infrastructure::network::session_manager::SessionManager;
use crate::net::io::ProtocolReadExt;
use crate::protocol::{opcodes, RawPacket};
use async_trait::async_trait;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, crate::core::error::ArceusError>;

#[async_trait]
pub trait PacketHandler: Send + Sync {
    fn opcode(&self) -> u8;
    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()>;
}

pub struct PacketHandlerRegistry {
    handlers: std::collections::HashMap<u8, Arc<dyn PacketHandler>>,
}

impl PacketHandlerRegistry {
    pub fn new(
        device_repo: Arc<dyn DeviceRepository>,
        device_name_repo: Arc<dyn DeviceNameRepository>,
        event_bus: Arc<EventBus>,
        session_manager: Arc<SessionManager>,
    ) -> Self {
        let mut registry = Self {
            handlers: std::collections::HashMap::new(),
        };

        registry.register(Arc::new(DeviceConnectedHandler::new(
            device_repo.clone(),
            device_name_repo.clone(),
            event_bus.clone(),
            session_manager.clone(),
        )));
        registry.register(Arc::new(HeartbeatHandler::new(device_repo.clone())));
        registry.register(Arc::new(BatteryStatusHandler::new(
            device_repo.clone(),
            event_bus.clone(),
        )));
        registry.register(Arc::new(VolumeStatusHandler::new(
            device_repo.clone(),
            event_bus.clone(),
        )));

        // Response handlers
        registry.register(Arc::new(LaunchAppResponseHandler::new(event_bus.clone())));
        registry.register(Arc::new(ShellExecutionResponseHandler::new(event_bus.clone())));
        registry.register(Arc::new(InstalledAppsResponseHandler::new(event_bus.clone())));
        registry.register(Arc::new(PingResponseHandler::new(event_bus.clone())));
        registry.register(Arc::new(ApkInstallResponseHandler::new(event_bus.clone())));
        registry.register(Arc::new(UninstallAppResponseHandler::new(event_bus.clone())));
        registry.register(Arc::new(VolumeSetResponseHandler::new(
            device_repo.clone(),
            event_bus.clone(),
        )));
        registry.register(Arc::new(ApkDownloadStartedHandler::new(event_bus.clone())));
        registry.register(Arc::new(CloseAllAppsResponseHandler::new(event_bus.clone())));

        registry
    }

    /// Register a packet handler
    pub fn register(&mut self, handler: Arc<dyn PacketHandler>) {
        let opcode = handler.opcode();
        self.handlers.insert(opcode, handler);
    }

    /// Handle a received packet
    pub async fn handle(&self, device_id: DeviceId, packet: RawPacket) -> Result<()> {
        match self.handlers.get(&packet.opcode) {
            Some(handler) => {
                handler.handle(device_id, packet.payload).await?;
                Ok(())
            }
            None => {
                tracing::debug!(
                    device_id = %device_id,
                    opcode = packet.opcode,
                    "No handler registered for opcode"
                );
                Ok(())
            }
        }
    }
}

/// Handles DEVICE_CONNECTED (0x01) packets
/// Payload: [model: String][serial: String]
struct DeviceConnectedHandler {
    device_repo: Arc<dyn DeviceRepository>,
    device_name_repo: Arc<dyn DeviceNameRepository>,
    event_bus: Arc<EventBus>,
    session_manager: Arc<SessionManager>,
}

impl DeviceConnectedHandler {
    fn new(
        device_repo: Arc<dyn DeviceRepository>,
        device_name_repo: Arc<dyn DeviceNameRepository>,
        event_bus: Arc<EventBus>,
        session_manager: Arc<SessionManager>,
    ) -> Self {
        Self {
            device_repo,
            device_name_repo,
            event_bus,
            session_manager,
        }
    }

    /// Helper to send initial status requests to a newly connected device
    async fn send_initial_status_requests(device_id: DeviceId, session_manager: Arc<SessionManager>) {
        // Brief delay to ensure device is ready
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let Some(session) = session_manager.get_session(&device_id) else {
            return;
        };

        // Request battery status
        let _ = session.send_packet(RawPacket {
            opcode: opcodes::REQUEST_BATTERY,
            payload: vec![],
        }).await;

        // Request volume status
        let _ = session.send_packet(RawPacket {
            opcode: opcodes::GET_VOLUME,
            payload: vec![],
        }).await;

        tracing::debug!(device_id = %device_id, "Sent initial battery and volume requests");
    }
}

#[async_trait]
impl PacketHandler for DeviceConnectedHandler {
    fn opcode(&self) -> u8 {
        opcodes::DEVICE_CONNECTED
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);

        let model = cursor.read_string()?;
        let serial_str = cursor.read_string()?;

        tracing::info!(
            device_id = %device_id,
            model = %model,
            serial = %serial_str,
            "Device connected packet received"
        );

        let serial = Serial::new(serial_str)
            .map_err(|e| crate::core::error::ArceusError::Other(format!("Invalid serial: {}", e)))?;

        // Get the temporary device that was created on TCP connection
        let temp_device = self.device_repo.find_by_id(device_id).await.ok().flatten();

        let temp_device = match temp_device {
            Some(device) => device,
            None => {
                tracing::warn!("Device connected packet without prior TCP connection");
                return Ok(());
            }
        };

        // Create device with real info from the packet
        let device = Device::new(device_id, serial.clone(), model.clone(), temp_device.ip().clone());

        // Load custom name from database if exists
        let custom_name = self.device_name_repo.get_name(&serial).await.ok().flatten();
        let device = device.with_custom_name(custom_name.clone());

        self.device_repo.save(device.clone()).await?;

        tracing::info!(
            device_id = %device_id,
            serial = %serial.as_str(),
            "Device connected"
        );

        // Emit DeviceConnected event to frontend
        let device_info = crate::core::models::device::DeviceInfo {
            id: device.id().as_uuid().clone(),
            model: model.clone(),
            serial: serial.as_str().to_string(),
            ip: device.ip().as_str().to_string(),
            connected_at: device.connected_at(),
            last_seen: device.last_seen(),
            custom_name: custom_name,
        };
        let device_state = crate::core::models::device::DeviceState {
            info: device_info,
            battery: device.battery().map(|b| crate::core::models::battery::BatteryInfo {
                headset_level: b.level(),
                is_charging: b.is_charging(),
            }),
            volume: device.volume().map(|v| {
                crate::core::models::volume::VolumeInfo::new(v.percentage(), v.current(), v.max())
            }),
            command_history: std::collections::VecDeque::new(),
        };
        self.event_bus.device_connected(device_state);
        
        // Request initial connection data
        tokio::spawn(Self::send_initial_status_requests(
            device.id(),
            self.session_manager.clone(),
        ));

        Ok(())
    }
}

/// Handles HEARTBEAT (0x02) packets
/// No payload
struct HeartbeatHandler {}

impl HeartbeatHandler {
    fn new(_device_repo: Arc<dyn DeviceRepository>) -> Self {
        Self {}
    }
}

#[async_trait]
impl PacketHandler for HeartbeatHandler {
    fn opcode(&self) -> u8 {
        opcodes::HEARTBEAT
    }

    async fn handle(&self, device_id: DeviceId, _payload: Vec<u8>) -> Result<()> {
        tracing::trace!(device_id = %device_id, "Heartbeat received");

        // Update last_seen is already handled in the message loop
        // This handler just acknowledges the heartbeat

        Ok(())
    }
}

/// Handles BATTERY_STATUS (0x03) packets
/// Payload: [level: u8][is_charging: bool]
struct BatteryStatusHandler {
    device_repo: Arc<dyn DeviceRepository>,
    event_bus: Arc<EventBus>,
}

impl BatteryStatusHandler {
    fn new(device_repo: Arc<dyn DeviceRepository>, event_bus: Arc<EventBus>) -> Self {
        Self {
            device_repo,
            event_bus,
        }
    }
}

#[async_trait]
impl PacketHandler for BatteryStatusHandler {
    fn opcode(&self) -> u8 {
        opcodes::BATTERY_STATUS
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);

        let level = cursor.read_u8()?;
        let is_charging = cursor.read_u8()? != 0;

        tracing::debug!(
            device_id = %device_id,
            level = level,
            is_charging = is_charging,
            "Battery status received"
        );

        // Update device with battery info
        if let Ok(Some(device)) = self.device_repo.find_by_id(device_id).await {
            let battery = Battery::new(level, is_charging)
                .map_err(|e| crate::core::error::ArceusError::Other(format!("Invalid battery: {}", e)))?;

            let updated_device = device.with_battery(battery);
            self.device_repo.save(updated_device).await?;

            // Emit event
            let battery_info = crate::core::models::battery::BatteryInfo {
                headset_level: level,
                is_charging,
            };
            self.event_bus.battery_updated(device_id.as_uuid().clone(), battery_info);
        }

        Ok(())
    }
}

/// Handles VOLUME_STATUS (0x04) packets
/// Payload: [current: u8][max: u8]
struct VolumeStatusHandler {
    device_repo: Arc<dyn DeviceRepository>,
    event_bus: Arc<EventBus>,
}

impl VolumeStatusHandler {
    fn new(device_repo: Arc<dyn DeviceRepository>, event_bus: Arc<EventBus>) -> Self {
        Self {
            device_repo,
            event_bus,
        }
    }
}

#[async_trait]
impl PacketHandler for VolumeStatusHandler {
    fn opcode(&self) -> u8 {
        opcodes::VOLUME_STATUS
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);

        let first = cursor.read_u8()?;
        let second = cursor.read_u8()?;

        // Device sends [percentage(0-100)][max], but we need [current][max]
        // If first > second, assume device sent [percentage][max] format
        let (current, max) = if first > second && second > 0 {
            // Percentage format: calculate current from percentage
            let percentage = first;
            let max = second;
            let current = ((percentage as f32 / 100.0) * max as f32).round() as u8;
            (current, max)
        } else {
            // Assume [current][max] format
            (first, second)
        };

        tracing::debug!(
            device_id = %device_id,
            current = current,
            max = max,
            "Volume status received (parsed from first={}, second={})",
            first,
            second
        );

        // Update device with volume info
        if let Ok(Some(device)) = self.device_repo.find_by_id(device_id).await {
            let volume = Volume::new(current, max)
                .map_err(|e| crate::core::error::ArceusError::Other(format!("Invalid volume: {}", e)))?;

            let updated_device = device.with_volume(volume);
            self.device_repo.save(updated_device).await?;

            // Emit event
            let percentage = ((current as f32 / max as f32) * 100.0) as u8;
            let volume_info = crate::core::models::volume::VolumeInfo::new(
                percentage,
                current,
                max,
            );
            self.event_bus.volume_updated(device_id.as_uuid().clone(), volume_info);
        }

        Ok(())
    }
}

// =============================================================================
// Response Handlers (0x10-0x17)
// =============================================================================

/// Handles PING_RESPONSE (0x13) packets
struct PingResponseHandler {
    event_bus: Arc<EventBus>,
}

impl PingResponseHandler {
    fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait]
impl PacketHandler for PingResponseHandler {
    fn opcode(&self) -> u8 {
        crate::protocol::opcodes::PING_RESPONSE
    }

    async fn handle(&self, device_id: DeviceId, _payload: Vec<u8>) -> Result<()> {
        tracing::debug!(device_id = %device_id, "Ping response received");

        // Emit event to frontend
        let result = CommandResult::success("ping", "Ping successful");
        self.event_bus.command_executed(device_id.as_uuid().clone(), result);

        Ok(())
    }
}

/// Handles LAUNCH_APP_RESPONSE (0x10) packets
struct LaunchAppResponseHandler {
    event_bus: Arc<EventBus>,
}

impl LaunchAppResponseHandler {
    fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait]
impl PacketHandler for LaunchAppResponseHandler {
    fn opcode(&self) -> u8 {
        crate::protocol::opcodes::LAUNCH_APP_RESPONSE
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);
        let success = cursor.read_u8()? != 0;

        tracing::debug!(device_id = %device_id, success, "Launch app response");

        let result = if success {
            CommandResult::success("launch_app", "App launched successfully")
        } else {
            CommandResult::failure("launch_app", "Failed to launch app")
        };
        self.event_bus.command_executed(device_id.as_uuid().clone(), result);

        Ok(())
    }
}

/// Handles SHELL_EXECUTION_RESPONSE (0x11) packets
/// Payload: [success: u8][output: String][exit_code: i32]
struct ShellExecutionResponseHandler {
    event_bus: Arc<EventBus>,
}

impl ShellExecutionResponseHandler {
    fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait]
impl PacketHandler for ShellExecutionResponseHandler {
    fn opcode(&self) -> u8 {
        crate::protocol::opcodes::SHELL_EXECUTION_RESPONSE
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);
        let success = cursor.read_u8()? != 0;
        let output = cursor.read_string()?;
        let exit_code = cursor.read_i32::<BigEndian>()?;

        tracing::debug!(
            device_id = %device_id,
            success = success,
            exit_code = exit_code,
            "Shell execution response"
        );

        let result = if success {
            CommandResult::success("shell_execution", output)
        } else {
            CommandResult::failure("shell_execution", output)
        };
        self.event_bus.command_executed(device_id.as_uuid().clone(), result);

        Ok(())
    }
}

/// Handles INSTALLED_APPS_RESPONSE (0x12) packets
struct InstalledAppsResponseHandler {
    event_bus: Arc<EventBus>,
}

impl InstalledAppsResponseHandler {
    fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait]
impl PacketHandler for InstalledAppsResponseHandler {
    fn opcode(&self) -> u8 {
        crate::protocol::opcodes::INSTALLED_APPS_RESPONSE
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);
        let count = cursor.read_u32::<BigEndian>()? as usize;

        let mut apps = Vec::with_capacity(count);
        for _ in 0..count {
            let package_name = cursor.read_string()?;
            apps.push(package_name);
        }

        tracing::debug!(device_id = %device_id, app_count = count, "Installed apps response");

        self.event_bus.installed_apps_received(device_id.as_uuid().clone(), apps);

        Ok(())
    }
}

/// Handles APK_INSTALL_RESPONSE (0x14) packets
struct ApkInstallResponseHandler {
    event_bus: Arc<EventBus>,
}

impl ApkInstallResponseHandler {
    fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait]
impl PacketHandler for ApkInstallResponseHandler {
    fn opcode(&self) -> u8 {
        crate::protocol::opcodes::APK_INSTALL_RESPONSE
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);
        let success = cursor.read_u8()? != 0;

        if success {
            tracing::info!(device_id = %device_id, "APK installed successfully");
        } else {
            tracing::warn!(device_id = %device_id, "APK installation failed");
        }

        let result = if success {
            CommandResult::success("apk_install", "APK installed successfully")
        } else {
            CommandResult::failure("apk_install", "Failed to install APK")
        };
        self.event_bus.command_executed(device_id.as_uuid().clone(), result);

        Ok(())
    }
}

/// Handles UNINSTALL_APP_RESPONSE (0x15) packets
struct UninstallAppResponseHandler {
    event_bus: Arc<EventBus>,
}

impl UninstallAppResponseHandler {
    fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait]
impl PacketHandler for UninstallAppResponseHandler {
    fn opcode(&self) -> u8 {
        crate::protocol::opcodes::UNINSTALL_APP_RESPONSE
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);
        let success = cursor.read_u8()? != 0;

        tracing::debug!(device_id = %device_id, success, "Uninstall app response");

        let result = if success {
            CommandResult::success("uninstall_app", "App uninstalled successfully")
        } else {
            CommandResult::failure("uninstall_app", "Failed to uninstall app")
        };
        self.event_bus.command_executed(device_id.as_uuid().clone(), result);

        Ok(())
    }
}

/// Handles VOLUME_SET_RESPONSE (0x16) packets
/// The client sends back the actual volume percentage achieved after setting
struct VolumeSetResponseHandler {
    device_repo: Arc<dyn DeviceRepository>,
    event_bus: Arc<EventBus>,
}

impl VolumeSetResponseHandler {
    fn new(device_repo: Arc<dyn DeviceRepository>, event_bus: Arc<EventBus>) -> Self {
        Self { device_repo, event_bus }
    }
}

#[async_trait]
impl PacketHandler for VolumeSetResponseHandler {
    fn opcode(&self) -> u8 {
        crate::protocol::opcodes::VOLUME_SET_RESPONSE
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);
        let success = cursor.read_u8()? != 0;
        let message = cursor.read_string()?;

        tracing::debug!(device_id = %device_id, success, message = %message, "Volume set response");

        if success {
            if let Some(actual_percentage) = message
                .split_whitespace()
                .find_map(|word| word.trim_end_matches('%').parse::<u8>().ok())
            {
                if let Ok(Some(device)) = self.device_repo.find_by_id(device_id).await {
                    let (current, max) = if let Some(volume) = device.volume() {
                        let max = volume.max();
                        let current = (actual_percentage as u16 * max as u16 / 100) as u8;
                        (current, max)
                    } else {
                        let max = 15u8;
                        let current = (actual_percentage as u16 * max as u16 / 100) as u8;
                        (current, max)
                    };

                    if let Ok(volume) = Volume::new(current, max) {
                        let updated_device = device.with_volume(volume);
                        let _ = self.device_repo.save(updated_device).await;

                        let volume_info = crate::core::models::volume::VolumeInfo::new(
                            actual_percentage,
                            current,
                            max,
                        );
                        self.event_bus.volume_updated(device_id.as_uuid().clone(), volume_info);
                    }
                }
            }
        }

        let result = if success {
            CommandResult::success("volume_set", &message)
        } else {
            CommandResult::failure("volume_set", &message)
        };
        self.event_bus.command_executed(device_id.as_uuid().clone(), result);

        Ok(())
    }
}

/// Handles APK_DOWNLOAD_STARTED (0x17) packets
struct ApkDownloadStartedHandler {
    event_bus: Arc<EventBus>,
}

impl ApkDownloadStartedHandler {
    fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait]
impl PacketHandler for ApkDownloadStartedHandler {
    fn opcode(&self) -> u8 {
        crate::protocol::opcodes::APK_DOWNLOAD_STARTED
    }

    async fn handle(&self, device_id: DeviceId, _payload: Vec<u8>) -> Result<()> {
        tracing::info!(device_id = %device_id, "APK download started on device");

        let result = CommandResult::success("apk_download", "APK download started");
        self.event_bus.command_executed(device_id.as_uuid().clone(), result);

        Ok(())
    }
}

/// Handles CLOSE_ALL_APPS_RESPONSE (0x18) packets
/// Payload: [success: u8][message: String][closed_count: u32][closed_apps: List<String>]
struct CloseAllAppsResponseHandler {
    event_bus: Arc<EventBus>,
}

impl CloseAllAppsResponseHandler {
    fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait]
impl PacketHandler for CloseAllAppsResponseHandler {
    fn opcode(&self) -> u8 {
        crate::protocol::opcodes::CLOSE_ALL_APPS_RESPONSE
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        let mut cursor = Cursor::new(payload);
        let success = cursor.read_u8()? != 0;
        let message = cursor.read_string()?;
        let closed_count = cursor.read_u32::<BigEndian>()? as usize;

        let mut closed_apps = Vec::with_capacity(closed_count);
        for _ in 0..closed_count {
            let package_name = cursor.read_string()?;
            closed_apps.push(package_name);
        }

        tracing::info!(
            device_id = %device_id,
            success = success,
            closed_count = closed_count,
            "Close all apps response: {}",
            message
        );

        let result = if success {
            CommandResult::success("close_all_apps", "Successfully closed all apps")
        } else {
            CommandResult::failure("close_all_apps", &message)
        };
        self.event_bus.command_executed(device_id.as_uuid().clone(), result);

        Ok(())
    }
}
