use super::message_type::MessageType;
use crate::core::error::{Result};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug, Clone)]
pub struct Message {
    pub msg_type: MessageType,
    pub payload: Bytes,
}

impl Message {
    pub fn new(msg_type: MessageType, payload: Bytes) -> Self {
        Self { msg_type, payload }
    }

    pub fn empty(msg_type: MessageType) -> Self {
        Self {
            msg_type,
            payload: Bytes::new(),
        }
    }

    pub fn from_vec(msg_type: MessageType, payload: Vec<u8>) -> Self {
        Self {
            msg_type,
            payload: Bytes::from(payload),
        }
    }

    pub fn len(&self) -> usize {
        1 + self.payload.len()
    }

    pub fn is_empty(&self) -> bool {
        self.payload.is_empty()
    }
}

pub struct MessageCodec;

impl MessageCodec {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MessageCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder for MessageCodec {
    type Item = Message;
    type Error = crate::core::error::ArceusError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>> {
        if src.len() < 1 {
            return Ok(None);
        }

        let msg_type_byte = src[0];
        let msg_type = MessageType::from_u8(msg_type_byte)?;

        match msg_type {
            MessageType::Heartbeat
            | MessageType::RequestBattery
            | MessageType::GetInstalledApps
            | MessageType::GetDeviceInfo
            | MessageType::Ping
            | MessageType::ShutdownDevice
            | MessageType::GetVolume => {
                src.advance(1);
                Ok(Some(Message::empty(msg_type)))
            }

            MessageType::BatteryStatus | MessageType::SetVolume => {
                if src.len() < 2 {
                    return Ok(None);
                }
                src.advance(1);
                let payload_byte = src.get_u8();
                Ok(Some(Message::from_vec(msg_type, vec![payload_byte])))
            }

            MessageType::VolumeStatus => {
                if src.len() < 4 {
                    return Ok(None);
                }
                src.advance(1);
                let volume_percentage = src.get_u8();
                let current_volume = src.get_u8();
                let max_volume = src.get_u8();
                Ok(Some(Message::from_vec(
                    msg_type,
                    vec![volume_percentage, current_volume, max_volume],
                )))
            }

            MessageType::DeviceConnected
            | MessageType::LaunchApp
            | MessageType::ExecuteShell
            | MessageType::DownloadAndInstallApk
            | MessageType::UninstallApp
            | MessageType::InstallLocalApk
            | MessageType::CommandResponse
            | MessageType::Error => {
                if src.len() < 5 {
                    return Ok(None);
                }

                let length = u32::from_be_bytes([src[1], src[2], src[3], src[4]]) as usize;

                let total_needed = 1 + 4 + length;

                if matches!(
                    msg_type,
                    MessageType::DeviceConnected | MessageType::CommandResponse
                ) {
                    if src.len() < total_needed {
                        src.reserve(total_needed - src.len());
                        return Ok(None);
                    }
                }

                if src.len() < total_needed {
                    src.reserve(total_needed - src.len());
                    return Ok(None);
                }

                src.advance(1);

                let mut payload_bytes = vec![0u8; src.len()];
                src.copy_to_slice(&mut payload_bytes);

                Ok(Some(Message::from_vec(msg_type, payload_bytes)))
            }
        }
    }
}

impl Encoder<Message> for MessageCodec {
    type Error = crate::core::error::ArceusError;

    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<()> {
        dst.reserve(item.len());
        dst.put_u8(item.msg_type.to_u8());
        dst.put_slice(&item.payload);

        Ok(())
    }
}

