/// Simple response handlers using macro for code generation
/// These handlers follow a common pattern: read success byte, emit event

use crate::app::EventBus;
use crate::application::dto::CommandResultDto;
use crate::domain::models::DeviceId;
use crate::infrastructure::protocol::opcodes;
use async_trait::async_trait;
use byteorder::ReadBytesExt;
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
