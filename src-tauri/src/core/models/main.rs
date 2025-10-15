///
/// These models represent the domain entities and are designed to be:
/// - Immutable where possible (using builder patterns for modification)
/// - Serializable for Tauri IPC
/// - Thread-safe
/// - Self-documenting


// Re-export update types

///
/// These models represent the domain entities and are designed to be:
/// - Immutable where possible (using builder patterns for modification)
/// - Serializable for Tauri IPC
/// - Thread-safe
/// - Self-documenting

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Information about a Quest device
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceInfo {
    /// Unique identifier for this device session
    pub id: Uuid,

    /// Device model (e.g., "Quest 2", "Quest 3", "Quest Pro")
    pub model: String,

    /// Device serial number (unique hardware identifier)
    pub serial: String,

    /// IP address of the device
    pub ip: String,

    /// When the device connected to the server
    pub connected_at: DateTime<Utc>,

    /// Last time the device sent any message
    pub last_seen: DateTime<Utc>,

    /// Custom name assigned by user (optional)
    pub custom_name: Option<String>,
}

impl DeviceInfo {
    /// Create a new DeviceInfo instance
    pub fn new(model: String, serial: String, ip: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            model,
            serial,
            ip,
            connected_at: now,
            last_seen: now,
            custom_name: None,
        }
    }

    /// Get the display name (custom name if set, otherwise model)
    pub fn display_name(&self) -> &str {
        self.custom_name.as_deref().unwrap_or(&self.model)
    }

    /// Update last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    }

    /// Set custom name
    pub fn set_custom_name(&mut self, name: Option<String>) {
        self.custom_name = name;
    }
}

/// Battery information for a Quest device
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatteryInfo {
    /// Battery level percentage (0-100)
    pub headset_level: u8,

    /// Whether the device is currently charging
    pub is_charging: bool,

    /// When this battery info was last updated
    pub last_updated: DateTime<Utc>,
}

impl BatteryInfo {
    /// Create new BatteryInfo
    pub fn new(headset_level: u8, is_charging: bool) -> Self {
        Self {
            headset_level: headset_level.min(100), // Cap at 100%
            is_charging,
            last_updated: Utc::now(),
        }
    }

    /// Get battery level as a float (0.0 - 1.0) for UI rendering
    pub fn level_normalized(&self) -> f32 {
        self.headset_level as f32 / 100.0
    }

    /// Check if battery is low (< 20%)
    pub fn is_low(&self) -> bool {
        self.headset_level < 20
    }

    /// Check if battery is critical (< 10%)
    pub fn is_critical(&self) -> bool {
        self.headset_level < 10
    }
}

/// Volume information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VolumeInfo {
    /// Current volume level (0-100)
    pub level: u8,

    /// When this volume info was last updated
    pub last_updated: DateTime<Utc>,
}

impl VolumeInfo {
    pub fn new(level: u8) -> Self {
        Self {
            level: level.min(100),
            last_updated: Utc::now(),
        }
    }
}

/// Result of a command execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandResult {
    /// When the command was executed
    pub timestamp: DateTime<Utc>,

    /// Type of command (e.g., "LaunchApp", "InstallApk")
    pub command_type: String,

    /// Whether the command succeeded
    pub success: bool,

    /// Result message or error description
    pub message: String,
}

impl CommandResult {
    pub fn success(command_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            command_type: command_type.into(),
            success: true,
            message: message.into(),
        }
    }

    pub fn failure(command_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            command_type: command_type.into(),
            success: false,
            message: message.into(),
        }
    }
}

/// Complete state of a device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceState {
    /// Basic device information
    pub info: DeviceInfo,

    /// Current battery information (if available)
    pub battery: Option<BatteryInfo>,

    /// Current volume information (if available)
    pub volume: Option<VolumeInfo>,

    /// History of recent commands (limited to last N entries)
    pub command_history: Vec<CommandResult>,

    /// Whether the device is currently connected
    pub is_connected: bool,
}

impl DeviceState {
    /// Create new DeviceState from DeviceInfo
    pub fn new(info: DeviceInfo) -> Self {
        Self {
            info,
            battery: None,
            volume: None,
            command_history: Vec::new(),
            is_connected: true,
        }
    }

    /// Add a command result to history (maintains max size)
    pub fn add_command_result(&mut self, result: CommandResult) {
        const MAX_HISTORY_SIZE: usize = 50;

        self.command_history.insert(0, result);
        if self.command_history.len() > MAX_HISTORY_SIZE {
            self.command_history.truncate(MAX_HISTORY_SIZE);
        }
    }

    /// Get recent command history (last N commands)
    pub fn recent_commands(&self, count: usize) -> &[CommandResult] {
        let end = self.command_history.len().min(count);
        &self.command_history[..end]
    }

    /// Update battery information
    pub fn update_battery(&mut self, battery: BatteryInfo) {
        self.battery = Some(battery);
    }

    /// Update volume information
    pub fn update_volume(&mut self, volume: VolumeInfo) {
        self.volume = Some(volume);
    }

    /// Mark device as disconnected
    pub fn mark_disconnected(&mut self) {
        self.is_connected = false;
    }
}

/// Information about an APK file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApkFile {
    /// Filename of the APK
    pub filename: String,

    /// Size in bytes
    pub size_bytes: u64,

    /// Local HTTP URL where this APK can be accessed
    pub url: String,
}

impl ApkFile {
    pub fn new(filename: String, size_bytes: u64, url: String) -> Self {
        Self {
            filename,
            size_bytes,
            url,
        }
    }

    /// Format size as human-readable string
    pub fn size_formatted(&self) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if self.size_bytes >= GB {
            format!("{:.2} GB", self.size_bytes as f64 / GB as f64)
        } else if self.size_bytes >= MB {
            format!("{:.2} MB", self.size_bytes as f64 / MB as f64)
        } else if self.size_bytes >= KB {
            format!("{:.2} KB", self.size_bytes as f64 / KB as f64)
        } else {
            format!("{} bytes", self.size_bytes)
        }
    }
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// TCP server host (usually "0.0.0.0" for all interfaces)
    pub tcp_host: String,

    /// TCP server port (default: 8888)
    pub tcp_port: u16,

    /// HTTP server port for APK hosting (default: 8889)
    pub http_port: u16,

    /// Maximum number of concurrent device connections
    pub max_connections: usize,

    /// Battery update interval in seconds
    pub battery_update_interval: u64,

    /// Heartbeat timeout in seconds
    pub heartbeat_timeout: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            tcp_host: "0.0.0.0".to_string(),
            tcp_port: 8888,
            http_port: 8889,
            max_connections: 100,
            battery_update_interval: 60,
            heartbeat_timeout: 30,
        }
    }
}

