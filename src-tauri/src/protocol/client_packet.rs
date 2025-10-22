use crate::net::{io::*, PacketLength, ProtocolReadExt, ProtocolWriteExt};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

// =============================================================================
// CLIENT → SERVER PACKETS (Initiated by client)
// =============================================================================

/// DeviceConnected - Sent when client first connects
/// Opcode: 0x01
/// Payload: [model: String, serial: String]
#[derive(Debug, Clone)]
pub struct DeviceConnected {
    pub model: String,
    pub serial: String,
}

impl ProtocolReadable for DeviceConnected {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let model = src.read_string()?;
        let serial = src.read_string()?;
        Ok(Self { model, serial })
    }
}

impl ProtocolWritable for DeviceConnected {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_string(&self.model)?;
        dst.write_string(&self.serial)?;
        Ok(())
    }
}

/// Heartbeat - Keep-alive message
/// Opcode: 0x02
/// Payload: empty
#[derive(Debug, Clone)]
pub struct Heartbeat;

impl ProtocolReadable for Heartbeat {
    fn read<T>(_src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        Ok(Self)
    }
}

impl ProtocolWritable for Heartbeat {
    fn write<T>(&self, _dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        Ok(())
    }
}

/// BatteryStatus - Battery level and charging state
/// Opcode: 0x03
/// Payload: [level: u8, is_charging: bool]
#[derive(Debug, Clone)]
pub struct BatteryStatus {
    pub level: u8,
    pub is_charging: bool,
}

impl ProtocolReadable for BatteryStatus {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let level = src.read_u8()?;
        let is_charging = src.read_u8()? != 0;
        Ok(Self { level, is_charging })
    }
}

impl ProtocolWritable for BatteryStatus {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_u8(self.level)?;
        dst.write_u8(if self.is_charging { 1 } else { 0 })?;
        Ok(())
    }
}

/// VolumeStatus - Current volume information
/// Opcode: 0x04
/// Payload: [percentage: u8, current: u8, max: u8]
#[derive(Debug, Clone)]
pub struct VolumeStatus {
    pub percentage: u8,
    pub current: u8,
    pub max: u8,
}

impl ProtocolReadable for VolumeStatus {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let percentage = src.read_u8()?;
        let current = src.read_u8()?;
        let max = src.read_u8()?;
        Ok(Self {
            percentage,
            current,
            max,
        })
    }
}

impl ProtocolWritable for VolumeStatus {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_u8(self.percentage)?;
        dst.write_u8(self.current)?;
        dst.write_u8(self.max)?;
        Ok(())
    }
}

/// Error - Error message from client
/// Opcode: 0x05
/// Payload: [message: String]
#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl ProtocolReadable for Error {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let message = src.read_string()?;
        Ok(Self { message })
    }
}

impl ProtocolWritable for Error {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_string(&self.message)?;
        Ok(())
    }
}

// =============================================================================
// CLIENT → SERVER PACKETS (Responses to server commands)
// =============================================================================

/// LaunchAppResponse - Response to LaunchApp command
/// Opcode: 0x10
/// Payload: [success: bool, message: String]
#[derive(Debug, Clone)]
pub struct LaunchAppResponse {
    pub success: bool,
    pub message: String,
}

impl ProtocolReadable for LaunchAppResponse {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let success = src.read_u8()? != 0;
        let message = src.read_string()?;
        Ok(Self { success, message })
    }
}

impl ProtocolWritable for LaunchAppResponse {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_u8(if self.success { 1 } else { 0 })?;
        dst.write_string(&self.message)?;
        Ok(())
    }
}

/// ShellExecutionResponse - Response to ExecuteShell command
/// Opcode: 0x11
/// Payload: [success: bool, output: String, exit_code: i32]
#[derive(Debug, Clone)]
pub struct ShellExecutionResponse {
    pub success: bool,
    pub output: String,
    pub exit_code: i32,
}

impl ProtocolReadable for ShellExecutionResponse {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let success = src.read_u8()? != 0;
        let output = src.read_string()?;
        let exit_code = src.read_i32::<byteorder::BigEndian>()?;
        Ok(Self {
            success,
            output,
            exit_code,
        })
    }
}

impl ProtocolWritable for ShellExecutionResponse {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_u8(if self.success { 1 } else { 0 })?;
        dst.write_string(&self.output)?;
        dst.write_i32::<byteorder::BigEndian>(self.exit_code)?;
        Ok(())
    }
}

/// InstalledAppsResponse - Response to RequestInstalledApps command
/// Opcode: 0x12
/// Payload: [count: u32, apps: Vec<String>]
#[derive(Debug, Clone)]
pub struct InstalledAppsResponse {
    pub apps: Vec<String>,
}

impl ProtocolReadable for InstalledAppsResponse {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let count = src.read_u32::<byteorder::BigEndian>()?;
        let mut apps = Vec::with_capacity(count as usize);
        for _ in 0..count {
            apps.push(src.read_string()?);
        }
        Ok(Self { apps })
    }
}

impl ProtocolWritable for InstalledAppsResponse {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_u32::<byteorder::BigEndian>(self.apps.len() as u32)?;
        for app in &self.apps {
            dst.write_string(app)?;
        }
        Ok(())
    }
}

/// PingResponse - Response to Ping command
/// Opcode: 0x13
/// Payload: [timestamp: u64]
#[derive(Debug, Clone)]
pub struct PingResponse {
    pub timestamp: u64,
}

impl ProtocolReadable for PingResponse {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let timestamp = src.read_u64::<byteorder::BigEndian>()?;
        Ok(Self { timestamp })
    }
}

impl ProtocolWritable for PingResponse {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_u64::<byteorder::BigEndian>(self.timestamp)?;
        Ok(())
    }
}

/// ApkInstallResponse - Response to InstallApk command
/// Opcode: 0x14
/// Payload: [success: bool, message: String]
#[derive(Debug, Clone)]
pub struct ApkInstallResponse {
    pub success: bool,
    pub message: String,
}

impl ProtocolReadable for ApkInstallResponse {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let success = src.read_u8()? != 0;
        let message = src.read_string()?;
        Ok(Self { success, message })
    }
}

impl ProtocolWritable for ApkInstallResponse {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_u8(if self.success { 1 } else { 0 })?;
        dst.write_string(&self.message)?;
        Ok(())
    }
}

/// UninstallAppResponse - Response to UninstallApp command
/// Opcode: 0x15
/// Payload: [success: bool, message: String]
#[derive(Debug, Clone)]
pub struct UninstallAppResponse {
    pub success: bool,
    pub message: String,
}

impl ProtocolReadable for UninstallAppResponse {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let success = src.read_u8()? != 0;
        let message = src.read_string()?;
        Ok(Self { success, message })
    }
}

impl ProtocolWritable for UninstallAppResponse {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_u8(if self.success { 1 } else { 0 })?;
        dst.write_string(&self.message)?;
        Ok(())
    }
}

/// ShutdownResponse - Response to Shutdown command
/// Opcode: 0x16
/// Payload: empty
#[derive(Debug, Clone)]
pub struct ShutdownResponse;

impl ProtocolReadable for ShutdownResponse {
    fn read<T>(_src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        Ok(Self)
    }
}

impl ProtocolWritable for ShutdownResponse {
    fn write<T>(&self, _dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        Ok(())
    }
}

/// VolumeSetResponse - Response to SetVolume command
/// Opcode: 0x17
/// Payload: [success: bool, actual_level: u8]
#[derive(Debug, Clone)]
pub struct VolumeSetResponse {
    pub success: bool,
    pub actual_level: u8,
}

impl ProtocolReadable for VolumeSetResponse {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let success = src.read_u8()? != 0;
        let actual_level = src.read_u8()?;
        Ok(Self {
            success,
            actual_level,
        })
    }
}

impl ProtocolWritable for VolumeSetResponse {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_u8(if self.success { 1 } else { 0 })?;
        dst.write_u8(self.actual_level)?;
        Ok(())
    }
}

// =============================================================================
// PACKET ENUM - All client packets
// =============================================================================

use crate::packets;

packets! {
    ClientPacket {
        // Client-initiated (0x01-0x05)
        DeviceConnected => (opcode: 0x01, PacketLength::VariableShort),
        Heartbeat => (opcode: 0x02, PacketLength::Fixed(0)),
        BatteryStatus => (opcode: 0x03, PacketLength::Fixed(2)),
        VolumeStatus => (opcode: 0x04, PacketLength::Fixed(3)),
        Error => (opcode: 0x05, PacketLength::VariableShort),

        // Response packets (0x10-0x17)
        LaunchAppResponse => (opcode: 0x10, PacketLength::VariableShort),
        ShellExecutionResponse => (opcode: 0x11, PacketLength::VariableShort),
        InstalledAppsResponse => (opcode: 0x12, PacketLength::VariableShort),
        PingResponse => (opcode: 0x13, PacketLength::Fixed(8)),
        ApkInstallResponse => (opcode: 0x14, PacketLength::VariableShort),
        UninstallAppResponse => (opcode: 0x15, PacketLength::VariableShort),
        ShutdownResponse => (opcode: 0x16, PacketLength::Fixed(0)),
        VolumeSetResponse => (opcode: 0x17, PacketLength::Fixed(2)),
    }
}
