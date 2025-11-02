/// Protocol opcodes for all packet types.
///
/// Wire format: [Opcode: u8][Length: u16 BE][Payload]

// =============================================================================
// CLIENT → SERVER (Client-initiated) - 0x01-0x05
// =============================================================================

pub const DEVICE_CONNECTED: u8 = 0x01;
pub const HEARTBEAT: u8 = 0x02;
pub const BATTERY_STATUS: u8 = 0x03;
pub const VOLUME_STATUS: u8 = 0x04;

// =============================================================================
// CLIENT → SERVER (Responses to server commands) - 0x10-0x16
// =============================================================================

pub const LAUNCH_APP_RESPONSE: u8 = 0x10;
pub const SHELL_EXECUTION_RESPONSE: u8 = 0x11;
pub const INSTALLED_APPS_RESPONSE: u8 = 0x12;
pub const PING_RESPONSE: u8 = 0x13;
pub const APK_INSTALL_RESPONSE: u8 = 0x14;
pub const UNINSTALL_APP_RESPONSE: u8 = 0x15;
pub const VOLUME_SET_RESPONSE: u8 = 0x16;
pub const APK_DOWNLOAD_STARTED: u8 = 0x17;

// =============================================================================
// SERVER → CLIENT (Commands from server) - 0x40-0x4B
// =============================================================================

pub const LAUNCH_APP: u8 = 0x40;
pub const EXECUTE_SHELL: u8 = 0x41;
pub const REQUEST_BATTERY: u8 = 0x42;
pub const REQUEST_INSTALLED_APPS: u8 = 0x43;
pub const PING: u8 = 0x45;
pub const INSTALL_APK: u8 = 0x46;
pub const SHUTDOWN: u8 = 0x48;
pub const UNINSTALL_APP: u8 = 0x49;
pub const SET_VOLUME: u8 = 0x4A;
pub const GET_VOLUME: u8 = 0x4B;
