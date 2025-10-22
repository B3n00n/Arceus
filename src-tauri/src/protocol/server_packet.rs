use crate::net::{io::*, PacketLength, ProtocolReadExt, ProtocolWriteExt};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

// =============================================================================
// SERVER → CLIENT PACKETS (Commands sent to client devices)
// =============================================================================

/// LaunchApp - Launch an application by package name
/// Opcode: 0x40
/// Payload: [package_name: String]
#[derive(Debug, Clone)]
pub struct LaunchApp {
    pub package_name: String,
}

impl ProtocolReadable for LaunchApp {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let package_name = src.read_string()?;
        Ok(Self { package_name })
    }
}

impl ProtocolWritable for LaunchApp {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_string(&self.package_name)?;
        Ok(())
    }
}

/// ExecuteShell - Execute a shell command
/// Opcode: 0x41
/// Payload: [command: String]
#[derive(Debug, Clone)]
pub struct ExecuteShell {
    pub command: String,
}

impl ProtocolReadable for ExecuteShell {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let command = src.read_string()?;
        Ok(Self { command })
    }
}

impl ProtocolWritable for ExecuteShell {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_string(&self.command)?;
        Ok(())
    }
}

/// RequestBattery - Request battery status
/// Opcode: 0x42
/// Payload: empty
#[derive(Debug, Clone)]
pub struct RequestBattery;

impl ProtocolReadable for RequestBattery {
    fn read<T>(_src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        Ok(Self)
    }
}

impl ProtocolWritable for RequestBattery {
    fn write<T>(&self, _dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        Ok(())
    }
}

/// RequestInstalledApps - Request list of installed applications
/// Opcode: 0x43
/// Payload: empty
#[derive(Debug, Clone)]
pub struct RequestInstalledApps;

impl ProtocolReadable for RequestInstalledApps {
    fn read<T>(_src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        Ok(Self)
    }
}

impl ProtocolWritable for RequestInstalledApps {
    fn write<T>(&self, _dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        Ok(())
    }
}

/// RequestDeviceInfo - Request device information
/// Opcode: 0x44
/// Payload: empty
#[derive(Debug, Clone)]
pub struct RequestDeviceInfo;

impl ProtocolReadable for RequestDeviceInfo {
    fn read<T>(_src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        Ok(Self)
    }
}

impl ProtocolWritable for RequestDeviceInfo {
    fn write<T>(&self, _dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        Ok(())
    }
}

/// Ping - Test connectivity with timestamp
/// Opcode: 0x45
/// Payload: [timestamp: u64]
#[derive(Debug, Clone)]
pub struct Ping {
    pub timestamp: u64,
}

impl ProtocolReadable for Ping {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let timestamp = src.read_u64::<byteorder::BigEndian>()?;
        Ok(Self { timestamp })
    }
}

impl ProtocolWritable for Ping {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_u64::<byteorder::BigEndian>(self.timestamp)?;
        Ok(())
    }
}

/// InstallApk - Install APK from URL
/// Opcode: 0x46
/// Payload: [url: String]
#[derive(Debug, Clone)]
pub struct InstallApk {
    pub url: String,
}

impl ProtocolReadable for InstallApk {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let url = src.read_string()?;
        Ok(Self { url })
    }
}

impl ProtocolWritable for InstallApk {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_string(&self.url)?;
        Ok(())
    }
}

/// InstallLocalApk - Install APK from local server storage
/// Opcode: 0x47
/// Payload: [filename: String]
#[derive(Debug, Clone)]
pub struct InstallLocalApk {
    pub filename: String,
}

impl ProtocolReadable for InstallLocalApk {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let filename = src.read_string()?;
        Ok(Self { filename })
    }
}

impl ProtocolWritable for InstallLocalApk {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_string(&self.filename)?;
        Ok(())
    }
}

/// Shutdown - Shutdown/restart device
/// Opcode: 0x48
/// Payload: empty
#[derive(Debug, Clone)]
pub struct Shutdown;

impl ProtocolReadable for Shutdown {
    fn read<T>(_src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        Ok(Self)
    }
}

impl ProtocolWritable for Shutdown {
    fn write<T>(&self, _dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        Ok(())
    }
}

/// UninstallApp - Uninstall an application
/// Opcode: 0x49
/// Payload: [package_name: String]
#[derive(Debug, Clone)]
pub struct UninstallApp {
    pub package_name: String,
}

impl ProtocolReadable for UninstallApp {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let package_name = src.read_string()?;
        Ok(Self { package_name })
    }
}

impl ProtocolWritable for UninstallApp {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_string(&self.package_name)?;
        Ok(())
    }
}

/// SetVolume - Set device volume level
/// Opcode: 0x4A
/// Payload: [level: u8]
#[derive(Debug, Clone)]
pub struct SetVolume {
    pub level: u8,
}

impl ProtocolReadable for SetVolume {
    fn read<T>(src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        let level = src.read_u8()?;
        Ok(Self { level })
    }
}

impl ProtocolWritable for SetVolume {
    fn write<T>(&self, dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        dst.write_u8(self.level)?;
        Ok(())
    }
}

/// GetVolume - Request current volume level
/// Opcode: 0x4B
/// Payload: empty
#[derive(Debug, Clone)]
pub struct GetVolume;

impl ProtocolReadable for GetVolume {
    fn read<T>(_src: &mut T) -> Result<Self, PacketReadError>
    where
        T: Read + ReadBytesExt,
    {
        Ok(Self)
    }
}

impl ProtocolWritable for GetVolume {
    fn write<T>(&self, _dst: &mut T) -> anyhow::Result<()>
    where
        T: Write + WriteBytesExt,
    {
        Ok(())
    }
}

// =============================================================================
// PACKET ENUM - All server packets
// =============================================================================

use crate::packets;

packets! {
    ServerPacket {
        LaunchApp => (opcode: 0x40, PacketLength::VariableShort),
        ExecuteShell => (opcode: 0x41, PacketLength::VariableShort),
        RequestBattery => (opcode: 0x42, PacketLength::VariableShort),
        RequestInstalledApps => (opcode: 0x43, PacketLength::VariableShort),
        RequestDeviceInfo => (opcode: 0x44, PacketLength::VariableShort),
        Ping => (opcode: 0x45, PacketLength::VariableShort),
        InstallApk => (opcode: 0x46, PacketLength::VariableShort),
        InstallLocalApk => (opcode: 0x47, PacketLength::VariableShort),
        Shutdown => (opcode: 0x48, PacketLength::VariableShort),
        UninstallApp => (opcode: 0x49, PacketLength::VariableShort),
        SetVolume => (opcode: 0x4A, PacketLength::VariableShort),
        GetVolume => (opcode: 0x4B, PacketLength::VariableShort),
    }
}
