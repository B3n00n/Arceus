use crate::core::{
    BatteryInfo, CommandResult, DeviceInfo, DeviceState, EventBus, Result, VolumeInfo,
};
use crate::protocol::{RawPacket, RawPacketCodec};
use crate::storage::DeviceNamesStore;
use futures::{SinkExt, StreamExt};
use parking_lot::RwLock;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_util::codec::Framed;
use uuid::Uuid;

/// Represents a connected Quest device
pub struct DeviceConnection {
    id: Uuid,
    read_stream: Arc<Mutex<futures::stream::SplitStream<Framed<TcpStream, RawPacketCodec>>>>,
    write_stream:
        Arc<Mutex<futures::stream::SplitSink<Framed<TcpStream, RawPacketCodec>, RawPacket>>>,
    state: Arc<RwLock<DeviceState>>,
    event_bus: Arc<EventBus>,
    device_names_store: Arc<DeviceNamesStore>,
    addr: SocketAddr,
}

impl DeviceConnection {
    pub fn new(
        stream: TcpStream,
        addr: SocketAddr,
        event_bus: Arc<EventBus>,
        device_names_store: Arc<DeviceNamesStore>,
    ) -> Self {
        let id = Uuid::new_v4();
        let framed = Framed::new(stream, RawPacketCodec);

        let (write, read) = framed.split();

        let device_info = DeviceInfo::with_id(
            id,
            "Unknown".to_string(),
            id.to_string(),
            addr.ip().to_string(),
        );

        let state = DeviceState::new(device_info);

        Self {
            id,
            read_stream: Arc::new(Mutex::new(read)),
            write_stream: Arc::new(Mutex::new(write)),
            state: Arc::new(RwLock::new(state)),
            event_bus,
            device_names_store,
            addr,
        }
    }

    // =============================================================================
    // Public getters
    // =============================================================================

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn serial(&self) -> String {
        self.state.read().info.serial.clone()
    }

    pub fn get_state(&self) -> DeviceState {
        self.state.read().clone()
    }

    // =============================================================================
    // State update methods
    // =============================================================================

    pub fn update_device_info(&self, model: String, serial: String) {
        let mut state = self.state.write();
        state.info.model = model;
        state.info.serial = serial.clone();
        state.info.update_last_seen();

        // Load custom name from database if it exists
        if let Some(custom_name) = self.device_names_store.get_name(&serial) {
            state.info.custom_name = Some(custom_name);
        }

        tracing::info!(
            "Device {} ({}) connected from {}",
            state.info.model,
            state.info.serial,
            self.addr
        );

        // Emit device connected event with custom name included
        let device_state = state.clone();
        drop(state); // Release the write lock before emitting
        self.event_bus.device_connected(device_state);
    }

    pub fn update_last_seen(&self) {
        let mut state = self.state.write();
        state.info.update_last_seen();
    }

    pub fn update_battery(&self, battery: BatteryInfo) {
        let mut state = self.state.write();
        state.update_battery(battery.clone());

        tracing::debug!(
            "Device {} battery updated: {}%{}",
            state.info.serial,
            battery.headset_level,
            if battery.is_charging { " (charging)" } else { "" }
        );

        self.event_bus.battery_updated(self.id, battery);
    }

    pub fn update_volume(&self, volume: VolumeInfo) {
        let mut state = self.state.write();
        state.update_volume(volume.clone());

        tracing::debug!(
            "Device {} volume updated: {}% (current: {}, max: {})",
            state.info.serial,
            volume.volume_percentage,
            volume.current_volume,
            volume.max_volume
        );
        self.event_bus.volume_updated(self.id, volume);
    }

    pub fn update_installed_apps(&self, apps: Vec<String>) {
        tracing::debug!(
            "Device {} updated installed apps: {} apps",
            self.serial(),
            apps.len()
        );

        // Emit event to frontend
        self.event_bus.installed_apps_received(self.id, apps);
    }

    pub fn add_command_result(&self, result: CommandResult) {
        let mut state = self.state.write();
        state.add_command_result(result.clone());

        tracing::debug!(
            "Device {} command {} {}: {}",
            state.info.serial,
            result.command_type,
            if result.success { "succeeded" } else { "failed" },
            result.message
        );

        self.event_bus.command_executed(self.id, result);
    }

    pub fn set_custom_name(&self, name: Option<String>) {
        let mut state = self.state.write();
        state.info.set_custom_name(name.clone());

        tracing::info!(
            "Device {} custom name set to: {:?}",
            state.info.serial,
            name
        );

        self.event_bus
            .device_name_changed(self.id, state.info.serial.clone(), name);
    }

    pub fn mark_disconnected(&self) {
        let mut state = self.state.write();
        state.mark_disconnected();

        tracing::info!("Device {} disconnected", state.info.serial);

        self.event_bus
            .device_disconnected(self.id, state.info.serial.clone());
    }

    // =============================================================================
    // Low-level send/receive methods
    // =============================================================================

    /// Send a raw packet to the device
    pub async fn send_raw_packet(&self, packet: RawPacket) -> Result<()> {
        let mut write_stream = self.write_stream.lock().await;

        tracing::debug!(
            opcode = packet.opcode,
            device_id = %self.id,
            serial = %self.serial(),
            "Sending packet"
        );

        write_stream.send(packet).await.map_err(|e| {
            tracing::error!(
                device_id = %self.id,
                error = %e,
                "Failed to send packet"
            );
            e
        })?;

        write_stream.flush().await.map_err(|e| {
            tracing::error!(
                device_id = %self.id,
                error = %e,
                "Failed to flush"
            );
            e
        })?;

        tracing::debug!(device_id = %self.id, "Packet sent successfully");
        Ok(())
    }

    /// Receive a raw packet from the device
    pub async fn receive_raw_packet(&self) -> Result<Option<RawPacket>> {
        tracing::trace!(device_id = %self.id, "Waiting for packet");
        let mut read_stream = self.read_stream.lock().await;
        let result = read_stream.next().await.transpose().map_err(|e| {
            tracing::error!(
                device_id = %self.id,
                error = %e,
                "Failed to receive packet"
            );
            e
        });
        if result.is_ok() {
            tracing::trace!(device_id = %self.id, "Packet received");
        }
        result
    }

    // =============================================================================
    // High-level command methods (using raw packets!)
    // =============================================================================

    pub async fn launch_app(&self, package_name: &str) -> Result<()> {
        use crate::net::ProtocolWriteExt;
        use crate::protocol::opcodes;

        let mut payload = Vec::new();
        payload.write_string(package_name)?;

        self.send_raw_packet(RawPacket::new(opcodes::LAUNCH_APP, payload))
            .await
    }

    pub async fn execute_shell(&self, command: &str) -> Result<()> {
        use crate::net::ProtocolWriteExt;
        use crate::protocol::opcodes;

        let mut payload = Vec::new();
        payload.write_string(command)?;

        self.send_raw_packet(RawPacket::new(opcodes::EXECUTE_SHELL, payload))
            .await
    }

    pub async fn request_battery(&self) -> Result<()> {
        use crate::protocol::opcodes;
        self.send_raw_packet(RawPacket::empty(opcodes::REQUEST_BATTERY))
            .await
    }

    pub async fn request_installed_apps(&self) -> Result<()> {
        use crate::protocol::opcodes;
        self.send_raw_packet(RawPacket::empty(opcodes::REQUEST_INSTALLED_APPS))
            .await
    }

    pub async fn ping(&self) -> Result<()> {
        use crate::protocol::opcodes;
        use byteorder::WriteBytesExt;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let mut payload = Vec::new();
        payload.write_u64::<byteorder::BigEndian>(timestamp)?;

        self.send_raw_packet(RawPacket::new(opcodes::PING, payload))
            .await
    }

    pub async fn install_apk(&self, url: &str) -> Result<()> {
        use crate::net::ProtocolWriteExt;
        use crate::protocol::opcodes;

        let mut payload = Vec::new();
        payload.write_string(url)?;

        self.send_raw_packet(RawPacket::new(opcodes::INSTALL_APK, payload))
            .await
    }

    pub async fn install_local_apk(&self, filename: &str) -> Result<()> {
        use crate::net::ProtocolWriteExt;
        use crate::protocol::opcodes;

        let mut payload = Vec::new();
        payload.write_string(filename)?;

        self.send_raw_packet(RawPacket::new(opcodes::INSTALL_LOCAL_APK, payload))
            .await
    }

    pub async fn shutdown(&self) -> Result<()> {
        use crate::protocol::opcodes;
        self.send_raw_packet(RawPacket::empty(opcodes::SHUTDOWN))
            .await
    }

    pub async fn uninstall_app(&self, package_name: &str) -> Result<()> {
        use crate::net::ProtocolWriteExt;
        use crate::protocol::opcodes;

        let mut payload = Vec::new();
        payload.write_string(package_name)?;

        self.send_raw_packet(RawPacket::new(opcodes::UNINSTALL_APP, payload))
            .await
    }

    pub async fn set_volume(&self, level: u8) -> Result<()> {
        use crate::protocol::opcodes;
        use byteorder::WriteBytesExt;

        let mut payload = Vec::new();
        payload.write_u8(level.min(100))?;

        self.send_raw_packet(RawPacket::new(opcodes::SET_VOLUME, payload))
            .await
    }

    pub async fn get_volume(&self) -> Result<()> {
        use crate::protocol::opcodes;
        self.send_raw_packet(RawPacket::empty(opcodes::GET_VOLUME))
            .await
    }

    // Legacy alias methods for compatibility with service layer
    pub async fn get_installed_apps(&self) -> Result<()> {
        self.request_installed_apps().await
    }

    pub async fn restart(&self) -> Result<()> {
        self.shutdown().await
    }

    pub async fn install_remote_apk(&self, url: &str) -> Result<()> {
        self.install_apk(url).await
    }

    pub async fn request_volume(&self) -> Result<()> {
        self.get_volume().await
    }
}
