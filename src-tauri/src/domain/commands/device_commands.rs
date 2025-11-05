/// Concrete device command implementations
/// These commands correspond to the protocol opcodes for device operations.
use crate::domain::commands::Command;
use crate::domain::models::PackageName;
use crate::net::io::ProtocolWriteExt;
use crate::protocol::opcodes::*;
use byteorder::WriteBytesExt;

/// Launch an application on a device
#[derive(Debug, Clone)]
pub struct LaunchAppCommand {
    pub package_name: PackageName,
}

impl LaunchAppCommand {
    pub fn new(package_name: PackageName) -> Self {
        Self { package_name }
    }
}

impl Command for LaunchAppCommand {
    fn opcode(&self) -> u8 {
        LAUNCH_APP
    }

    fn name(&self) -> &'static str {
        "launch_app"
    }

    fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        buffer.write_string(self.package_name.as_str())?;
        Ok(buffer)
    }

    fn validate(&self) -> Result<(), String> {
        // PackageName is already validated in its constructor
        Ok(())
    }
}

/// Execute a shell command on a device
#[derive(Debug, Clone)]
pub struct ExecuteShellCommand {
    pub command: String,
}

impl ExecuteShellCommand {
    pub fn new(command: String) -> Self {
        Self { command }
    }
}

impl Command for ExecuteShellCommand {
    fn opcode(&self) -> u8 {
        EXECUTE_SHELL
    }

    fn name(&self) -> &'static str {
        "execute_shell"
    }

    fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        buffer.write_string(&self.command)?;
        Ok(buffer)
    }

    fn validate(&self) -> Result<(), String> {
        if self.command.is_empty() {
            return Err("Shell command cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Request battery status from a device
#[derive(Debug, Clone)]
pub struct RequestBatteryCommand;

impl Command for RequestBatteryCommand {
    fn opcode(&self) -> u8 {
        REQUEST_BATTERY
    }

    fn name(&self) -> &'static str {
        "request_battery"
    }

    fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        // No payload for battery request
        Ok(Vec::new())
    }
}

/// Request installed applications list from a device
#[derive(Debug, Clone)]
pub struct GetInstalledAppsCommand;

impl Command for GetInstalledAppsCommand {
    fn opcode(&self) -> u8 {
        REQUEST_INSTALLED_APPS
    }

    fn name(&self) -> &'static str {
        "get_installed_apps"
    }

    fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        // No payload
        Ok(Vec::new())
    }
}

/// Send a ping to a device
#[derive(Debug, Clone)]
pub struct PingCommand;

impl Command for PingCommand {
    fn opcode(&self) -> u8 {
        PING
    }

    fn name(&self) -> &'static str {
        "ping"
    }

    fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        use byteorder::{BigEndian, WriteBytesExt};
        

        // Send current timestamp in milliseconds as u64
        let timestamp_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let mut buf = Vec::new();
        buf.write_u64::<BigEndian>(timestamp_ms)?;
        Ok(buf)
    }
}

/// Install an APK from a URL
#[derive(Debug, Clone)]
pub struct InstallApkCommand {
    pub url: String,
}

impl InstallApkCommand {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

impl Command for InstallApkCommand {
    fn opcode(&self) -> u8 {
        INSTALL_APK
    }

    fn name(&self) -> &'static str {
        "install_apk"
    }

    fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        buffer.write_string(&self.url)?;
        Ok(buffer)
    }

    fn validate(&self) -> Result<(), String> {
        if self.url.is_empty() {
            return Err("APK URL cannot be empty".to_string());
        }
        if !self.url.starts_with("http://") && !self.url.starts_with("https://") {
            return Err("APK URL must be a valid HTTP/HTTPS URL".to_string());
        }
        Ok(())
    }
}

/// Uninstall an application from a device
#[derive(Debug, Clone)]
pub struct UninstallAppCommand {
    pub package_name: PackageName,
}

impl UninstallAppCommand {
    pub fn new(package_name: PackageName) -> Self {
        Self { package_name }
    }
}

impl Command for UninstallAppCommand {
    fn opcode(&self) -> u8 {
        UNINSTALL_APP
    }

    fn name(&self) -> &'static str {
        "uninstall_app"
    }

    fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        buffer.write_string(self.package_name.as_str())?;
        Ok(buffer)
    }
}

/// Set device volume
#[derive(Debug, Clone)]
pub struct SetVolumeCommand {
    pub level: u8,
}

impl SetVolumeCommand {
    pub fn new(level: u8) -> Result<Self, String> {
        if level > 100 {
            return Err(format!("Volume level must be 0-100, got {}", level));
        }
        Ok(Self { level })
    }
}

impl Command for SetVolumeCommand {
    fn opcode(&self) -> u8 {
        SET_VOLUME
    }

    fn name(&self) -> &'static str {
        "set_volume"
    }

    fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();
        buffer.write_u8(self.level)?;
        Ok(buffer)
    }

    fn validate(&self) -> Result<(), String> {
        if self.level > 100 {
            return Err(format!("Volume level must be 0-100, got {}", self.level));
        }
        Ok(())
    }
}

/// Request current volume level from a device
#[derive(Debug, Clone)]
pub struct GetVolumeCommand;

impl Command for GetVolumeCommand {
    fn opcode(&self) -> u8 {
        GET_VOLUME
    }

    fn name(&self) -> &'static str {
        "get_volume"
    }

    fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        // No payload
        Ok(Vec::new())
    }
}

/// Restart a device
#[derive(Debug, Clone)]
pub struct RestartDeviceCommand;

impl Command for RestartDeviceCommand {
    fn opcode(&self) -> u8 {
        SHUTDOWN
    }

    fn name(&self) -> &'static str {
        "restart_device"
    }

    fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        // No payload
        Ok(Vec::new())
    }
}

/// Close all running applications on a device
#[derive(Debug, Clone)]
pub struct CloseAllAppsCommand;

impl Command for CloseAllAppsCommand {
    fn opcode(&self) -> u8 {
        CLOSE_ALL_APPS
    }

    fn name(&self) -> &'static str {
        "close_all_apps"
    }

    fn serialize(&self) -> Result<Vec<u8>, std::io::Error> {
        // No payload
        Ok(Vec::new())
    }
}
