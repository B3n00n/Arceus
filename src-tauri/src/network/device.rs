use crate::core::{
    BatteryInfo, CommandResult, DeviceInfo, DeviceState, EventBus, Result, VolumeInfo,
};
use crate::protocol::{ClientPacket, ClientPacketCodec, ServerPacket};
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
    read_stream: Arc<Mutex<futures::stream::SplitStream<Framed<TcpStream, ClientPacketCodec>>>>,
    write_stream:
        Arc<Mutex<futures::stream::SplitSink<Framed<TcpStream, ClientPacketCodec>, ServerPacket>>>,
    state: Arc<RwLock<DeviceState>>,
    event_bus: Arc<EventBus>,
    addr: SocketAddr,
}

impl DeviceConnection {
    pub fn new(stream: TcpStream, addr: SocketAddr, event_bus: Arc<EventBus>) -> Self {
        let id = Uuid::new_v4();
        let framed = Framed::new(stream, ClientPacketCodec);

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
            addr,
        }
    }

    // =============================================================================
    // Public getters
    // =============================================================================

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn event_bus(&self) -> &Arc<EventBus> {
        &self.event_bus
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
        state.info.serial = serial;
        state.info.update_last_seen();

        tracing::info!(
            "Device {} ({}) connected from {}",
            state.info.model,
            state.info.serial,
            self.addr
        );
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

    /// Send a server packet to the device
    pub async fn send_packet(&self, packet: ServerPacket) -> Result<()> {
        let mut write_stream = self.write_stream.lock().await;

        tracing::debug!(
            opcode = packet.opcode(),
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

    /// Receive a client packet from the device
    pub async fn receive_packet(&self) -> Result<Option<ClientPacket>> {
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
    // High-level command methods (type-safe!)
    // =============================================================================

    pub async fn launch_app(&self, package_name: &str) -> Result<()> {
        use crate::protocol::server_packet::LaunchApp;
        self.send_packet(ServerPacket::LaunchApp(LaunchApp {
            package_name: package_name.to_string(),
        }))
        .await
    }

    pub async fn execute_shell(&self, command: &str) -> Result<()> {
        use crate::protocol::server_packet::ExecuteShell;
        self.send_packet(ServerPacket::ExecuteShell(ExecuteShell {
            command: command.to_string(),
        }))
        .await
    }

    pub async fn request_battery(&self) -> Result<()> {
        use crate::protocol::server_packet::RequestBattery;
        self.send_packet(ServerPacket::RequestBattery(RequestBattery))
            .await
    }

    pub async fn request_installed_apps(&self) -> Result<()> {
        use crate::protocol::server_packet::RequestInstalledApps;
        self.send_packet(ServerPacket::RequestInstalledApps(
            RequestInstalledApps,
        ))
        .await
    }

    pub async fn request_device_info(&self) -> Result<()> {
        use crate::protocol::server_packet::RequestDeviceInfo;
        self.send_packet(ServerPacket::RequestDeviceInfo(RequestDeviceInfo))
            .await
    }

    pub async fn ping(&self) -> Result<()> {
        use crate::protocol::server_packet::Ping;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        self.send_packet(ServerPacket::Ping(Ping { timestamp }))
            .await
    }

    pub async fn install_apk(&self, url: &str) -> Result<()> {
        use crate::protocol::server_packet::InstallApk;
        self.send_packet(ServerPacket::InstallApk(InstallApk {
            url: url.to_string(),
        }))
        .await
    }

    pub async fn install_local_apk(&self, filename: &str) -> Result<()> {
        use crate::protocol::server_packet::InstallLocalApk;
        self.send_packet(ServerPacket::InstallLocalApk(InstallLocalApk {
            filename: filename.to_string(),
        }))
        .await
    }

    pub async fn shutdown(&self) -> Result<()> {
        use crate::protocol::server_packet::Shutdown;
        self.send_packet(ServerPacket::Shutdown(Shutdown)).await
    }

    pub async fn uninstall_app(&self, package_name: &str) -> Result<()> {
        use crate::protocol::server_packet::UninstallApp;
        self.send_packet(ServerPacket::UninstallApp(UninstallApp {
            package_name: package_name.to_string(),
        }))
        .await
    }

    pub async fn set_volume(&self, level: u8) -> Result<()> {
        use crate::protocol::server_packet::SetVolume;
        self.send_packet(ServerPacket::SetVolume(SetVolume {
            level: level.min(100),
        }))
        .await
    }

    pub async fn get_volume(&self) -> Result<()> {
        use crate::protocol::server_packet::GetVolume;
        self.send_packet(ServerPacket::GetVolume(GetVolume)).await
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
