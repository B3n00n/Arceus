use crate::core::{
    BatteryInfo, CommandResult, DeviceInfo, DeviceState, EventBus, Result, VolumeInfo,
};
use crate::protocol::{Message, MessageCodec, MessageType, PacketWriter};
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use parking_lot::RwLock;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_util::codec::Framed;
use uuid::Uuid;

pub struct DeviceConnection {
    id: Uuid,
    read_stream: Arc<Mutex<futures::stream::SplitStream<Framed<TcpStream, MessageCodec>>>>,
    write_stream: Arc<Mutex<futures::stream::SplitSink<Framed<TcpStream, MessageCodec>, Message>>>,
    state: Arc<RwLock<DeviceState>>,
    event_bus: Arc<EventBus>,
    addr: SocketAddr,
}

impl DeviceConnection {
    pub fn new(
        stream: TcpStream,
        addr: SocketAddr,
        event_bus: Arc<EventBus>,
    ) -> Self {
        let id = Uuid::new_v4();
        let framed = Framed::new(stream, MessageCodec::new());

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

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn event_bus(&self) -> &Arc<EventBus> {
        &self.event_bus
    }

    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn serial(&self) -> String {
        self.state.read().info.serial.clone()
    }

    pub fn get_state(&self) -> DeviceState {
        self.state.read().clone()
    }

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

        self.event_bus.device_name_changed(
            self.id,
            state.info.serial.clone(),
            name,
        );
    }

    pub async fn send_message(&self, msg_type: MessageType, payload: Bytes) -> Result<()> {
        let message = Message::new(msg_type, payload.clone());
        let mut write_stream = self.write_stream.lock().await;

        tracing::debug!(
            msg_type = %msg_type,
            device_id = %self.id,
            serial = %self.serial(),
            payload_size = payload.len(),
            "Sending message"
        );

        write_stream.send(message).await.map_err(|e| {
            tracing::error!(
                msg_type = %msg_type,
                device_id = %self.id,
                error = %e,
                "Failed to send message"
            );
            e
        })?;

        write_stream.flush().await.map_err(|e| {
            tracing::error!(
                msg_type = %msg_type,
                device_id = %self.id,
                error = %e,
                "Failed to flush"
            );
            e
        })?;

        tracing::debug!(
            msg_type = %msg_type,
            device_id = %self.id,
            "Message sent"
        );
        Ok(())
    }

    pub async fn send_empty_message(&self, msg_type: MessageType) -> Result<()> {
        self.send_message(msg_type, Bytes::new()).await
    }

    pub async fn send_string_command(&self, msg_type: MessageType, payload: &str) -> Result<()> {
        let mut writer = PacketWriter::new();
        writer.write_string(payload);
        self.send_message(msg_type, writer.freeze()).await
    }

    pub async fn send_u8_command(&self, msg_type: MessageType, value: u8) -> Result<()> {
        let mut writer = PacketWriter::new();
        writer.write_u8(value);
        self.send_message(msg_type, writer.freeze()).await
    }

    pub async fn receive_message(&self) -> Result<Option<Message>> {
        tracing::trace!(device_id = %self.id, "Waiting for message");
        let mut read_stream = self.read_stream.lock().await;
        let result = read_stream.next().await.transpose().map_err(|e| {
            tracing::error!(
                device_id = %self.id,
                error = %e,
                "Failed to receive message"
            );
            e
        });
        if result.is_ok() {
            tracing::trace!(device_id = %self.id, "Message received");
        }
        result
    }

    pub fn mark_disconnected(&self) {
        let mut state = self.state.write();
        state.mark_disconnected();

        tracing::info!("Device {} disconnected", state.info.serial);

        self.event_bus.device_disconnected(self.id, state.info.serial.clone());
    }

    pub async fn request_battery(&self) -> Result<()> {
        self.send_empty_message(MessageType::RequestBattery).await
    }

    pub async fn request_volume(&self) -> Result<()> {
        self.send_empty_message(MessageType::GetVolume).await
    }

    pub async fn launch_app(&self, package_name: &str) -> Result<()> {
        self.send_string_command(MessageType::LaunchApp, package_name)
            .await
    }

    pub async fn uninstall_app(&self, package_name: &str) -> Result<()> {
        self.send_string_command(MessageType::UninstallApp, package_name)
            .await
    }

    pub async fn execute_shell(&self, command: &str) -> Result<()> {
        self.send_string_command(MessageType::ExecuteShell, command)
            .await
    }

    pub async fn get_installed_apps(&self) -> Result<()> {
        self.send_empty_message(MessageType::GetInstalledApps).await
    }

    pub async fn ping(&self) -> Result<()> {
        self.send_empty_message(MessageType::Ping).await
    }

    pub async fn set_volume(&self, level: u8) -> Result<()> {
        self.send_u8_command(MessageType::SetVolume, level.min(100))
            .await
    }

    pub async fn restart(&self) -> Result<()> {
        self.send_string_command(MessageType::ShutdownDevice, "restart")
            .await
    }

    pub async fn install_remote_apk(&self, url: &str) -> Result<()> {
        self.send_string_command(MessageType::DownloadAndInstallApk, url)
            .await
    }

    pub async fn install_local_apk(&self, url: &str) -> Result<()> {
        self.send_string_command(MessageType::InstallLocalApk, url)
            .await
    }
}

