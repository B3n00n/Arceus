/// Simple response handlers using macro for code generation
/// These handlers follow a common pattern: read success byte, emit event

use crate::app::EventBus;
use crate::application::dto::{CommandResultDto, OperationProgressDto};
use crate::domain::models::DeviceId;
use crate::domain::repositories::DeviceRepository;
use crate::infrastructure::protocol::opcodes;
use async_trait::async_trait;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use std::sync::Arc;

use super::super::super::{PacketHandler, Result};

macro_rules! simple_response_handler {
    ($handler_name:ident, $opcode:expr, $command_name:expr, $success_msg:expr, $failure_msg:expr) => {
        pub struct $handler_name {
            event_bus: Arc<EventBus>,
        }

        impl $handler_name {
            pub fn new(event_bus: Arc<EventBus>) -> Self {
                Self { event_bus }
            }
        }

        #[async_trait]
        impl PacketHandler for $handler_name {
            fn opcode(&self) -> u8 {
                $opcode
            }

            async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
                let mut cursor = Cursor::new(payload);
                let success = cursor.read_u8()? != 0;

                tracing::debug!(device_id = %device_id, success, concat!($command_name, " response"));

                let result = if success {
                    CommandResultDto::success($command_name, $success_msg)
                } else {
                    CommandResultDto::failure($command_name, $failure_msg)
                };
                self.event_bus.command_executed(device_id.as_uuid().clone(), result);

                Ok(())
            }
        }
    };
}

// Handles LAUNCH_APP_RESPONSE (0x10) packets
simple_response_handler!(
    LaunchAppResponseHandler,
    opcodes::LAUNCH_APP_RESPONSE,
    "launch_app",
    "App launched successfully",
    "Failed to launch app"
);

// Handles APK_INSTALL_RESPONSE (0x14) packets
simple_response_handler!(
    ApkInstallResponseHandler,
    opcodes::APK_INSTALL_RESPONSE,
    "apk_install",
    "APK installed successfully",
    "Failed to install APK"
);

// Handles UNINSTALL_APP_RESPONSE (0x15) packets
simple_response_handler!(
    UninstallAppResponseHandler,
    opcodes::UNINSTALL_APP_RESPONSE,
    "uninstall_app",
    "App uninstalled successfully",
    "Failed to uninstall app"
);

/// Handles PING_RESPONSE (0x13) packets
pub struct PingResponseHandler {
    event_bus: Arc<EventBus>,
}

impl PingResponseHandler {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait]
impl PacketHandler for PingResponseHandler {
    fn opcode(&self) -> u8 {
        crate::infrastructure::protocol::opcodes::PING_RESPONSE
    }

    async fn handle(&self, device_id: DeviceId, _payload: Vec<u8>) -> Result<()> {
        tracing::debug!(device_id = %device_id, "Ping response received");

        // Emit event to frontend
        let result = CommandResultDto::success("ping", "Ping successful");
        self.event_bus.command_executed(device_id.as_uuid().clone(), result);

        Ok(())
    }
}

/// Handles APK_DOWNLOAD_STARTED (0x17) packets
pub struct ApkDownloadStartedHandler {
    event_bus: Arc<EventBus>,
}

impl ApkDownloadStartedHandler {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }
}

#[async_trait]
impl PacketHandler for ApkDownloadStartedHandler {
    fn opcode(&self) -> u8 {
        crate::infrastructure::protocol::opcodes::APK_DOWNLOAD_STARTED
    }

    async fn handle(&self, device_id: DeviceId, _payload: Vec<u8>) -> Result<()> {
        tracing::info!(device_id = %device_id, "APK download started on device");

        let result = CommandResultDto::success("apk_download", "APK download started");
        self.event_bus.command_executed(device_id.as_uuid().clone(), result);

        Ok(())
    }
}

async fn handle_progress_packet(
    device_id: DeviceId,
    payload: Vec<u8>,
    operation_type: crate::application::dto::OperationType,
    operation_label: &str,
    event_bus: &Arc<EventBus>,
    device_repository: &Arc<dyn DeviceRepository>,
) -> Result<()> {
    let mut cursor = Cursor::new(payload);

    // Read operation ID (16 bytes UUID)
    let mut uuid_bytes = [0u8; 16];
    std::io::Read::read_exact(&mut cursor, &mut uuid_bytes)?;
    let operation_id = uuid::Uuid::from_bytes(uuid_bytes).to_string();

    // Read stage (0=Started, 1=InProgress, 2=Completed, 3=Failed)
    let stage_byte = cursor.read_u8()?;
    let stage = match stage_byte {
        0 => crate::application::dto::OperationStage::Started,
        1 => crate::application::dto::OperationStage::InProgress,
        2 => crate::application::dto::OperationStage::Completed,
        3 => crate::application::dto::OperationStage::Failed,
        _ => crate::application::dto::OperationStage::InProgress,
    };

    // Read percentage
    let percentage = cursor.read_f32::<BigEndian>()?;

    tracing::debug!(
        device_id = %device_id,
        operation_id = %operation_id,
        stage = ?stage,
        percentage,
        "APK {} progress",
        operation_label
    );

    // Get device name
    let device_name = match device_repository.find_by_id(device_id).await? {
        Some(device) => device.custom_name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| device.model().to_string()),
        None => format!("Device {}", device_id.as_uuid()),
    };

    // Create and emit progress event
    let progress = OperationProgressDto::new(operation_type, operation_id, stage, percentage);
    event_bus.operation_progress(device_id.as_uuid().clone(), device_name, progress);

    Ok(())
}

/// Handles APK_DOWNLOAD_PROGRESS (0x19) packets
/// Payload format: [operation_id: 16 bytes UUID][stage: u8][percentage: f32 BE]
pub struct ApkDownloadProgressHandler {
    event_bus: Arc<EventBus>,
    device_repository: Arc<dyn DeviceRepository>,
}

impl ApkDownloadProgressHandler {
    pub fn new(event_bus: Arc<EventBus>, device_repository: Arc<dyn DeviceRepository>) -> Self {
        Self { event_bus, device_repository }
    }
}

#[async_trait]
impl PacketHandler for ApkDownloadProgressHandler {
    fn opcode(&self) -> u8 {
        opcodes::APK_DOWNLOAD_PROGRESS
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        handle_progress_packet(
            device_id,
            payload,
            crate::application::dto::OperationType::Download,
            "download",
            &self.event_bus,
            &self.device_repository,
        ).await
    }
}

/// Handles APK_INSTALL_PROGRESS (0x1A) packets
/// Payload format: [operation_id: 16 bytes UUID][stage: u8][percentage: f32 BE]
pub struct ApkInstallProgressHandler {
    event_bus: Arc<EventBus>,
    device_repository: Arc<dyn DeviceRepository>,
}

impl ApkInstallProgressHandler {
    pub fn new(event_bus: Arc<EventBus>, device_repository: Arc<dyn DeviceRepository>) -> Self {
        Self { event_bus, device_repository }
    }
}

#[async_trait]
impl PacketHandler for ApkInstallProgressHandler {
    fn opcode(&self) -> u8 {
        opcodes::APK_INSTALL_PROGRESS
    }

    async fn handle(&self, device_id: DeviceId, payload: Vec<u8>) -> Result<()> {
        handle_progress_packet(
            device_id,
            payload,
            crate::application::dto::OperationType::Install,
            "install",
            &self.event_bus,
            &self.device_repository,
        ).await
    }
}
