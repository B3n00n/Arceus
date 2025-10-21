use super::message_type::MessageType;
use crate::core::error::Result;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

// Protocol size constants
const MESSAGE_TYPE_SIZE: usize = 1;
const U8_SIZE: usize = 1;
const U32_SIZE: usize = 4;
const VOLUME_PAYLOAD_SIZE: usize = 1; // volume level byte
const BATTERY_PAYLOAD_SIZE: usize = 2; // headset_level + is_charging
const VOLUME_STATUS_SIZE: usize = 3; // volume_percentage + current + max

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
        if src.is_empty() {
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
                src.advance(MESSAGE_TYPE_SIZE);
                Ok(Some(Message::empty(msg_type)))
            }

            MessageType::SetVolume => {
                let required = MESSAGE_TYPE_SIZE + VOLUME_PAYLOAD_SIZE;
                if src.len() < required {
                    return Ok(None);
                }
                src.advance(MESSAGE_TYPE_SIZE);
                let payload_byte = src.get_u8();
                Ok(Some(Message::from_vec(msg_type, vec![payload_byte])))
            }

            MessageType::BatteryStatus => {
                let required = MESSAGE_TYPE_SIZE + BATTERY_PAYLOAD_SIZE;
                if src.len() < required {
                    return Ok(None);
                }
                src.advance(MESSAGE_TYPE_SIZE);
                let headset_level = src.get_u8();
                let is_charging = src.get_u8();
                Ok(Some(Message::from_vec(msg_type, vec![headset_level, is_charging])))
            }

            MessageType::VolumeStatus => {
                let required = MESSAGE_TYPE_SIZE + VOLUME_STATUS_SIZE;
                if src.len() < required {
                    return Ok(None);
                }
                src.advance(MESSAGE_TYPE_SIZE);
                let volume_percentage = src.get_u8();
                let current_volume = src.get_u8();
                let max_volume = src.get_u8();
                Ok(Some(Message::from_vec(
                    msg_type,
                    vec![volume_percentage, current_volume, max_volume],
                )))
            }

            MessageType::DeviceConnected => {
                let min_required = MESSAGE_TYPE_SIZE + U32_SIZE; // type + first length
                if src.len() < min_required {
                    return Ok(None);
                }

                let len1 = u32::from_be_bytes([src[1], src[2], src[3], src[4]]) as usize;

                let after_first_string = MESSAGE_TYPE_SIZE + U32_SIZE + len1 + U32_SIZE;
                if src.len() < after_first_string {
                    return Ok(None);
                }

                let len2_offset = MESSAGE_TYPE_SIZE + U32_SIZE + len1;
                let len2 = u32::from_be_bytes([
                    src[len2_offset],
                    src[len2_offset + 1],
                    src[len2_offset + 2],
                    src[len2_offset + 3],
                ]) as usize;

                let total_needed = MESSAGE_TYPE_SIZE + U32_SIZE + len1 + U32_SIZE + len2;

                if src.len() < total_needed {
                    src.reserve(total_needed - src.len());
                    return Ok(None);
                }

                src.advance(MESSAGE_TYPE_SIZE);

                let payload = src.split_to(U32_SIZE + len1 + U32_SIZE + len2).freeze();

                Ok(Some(Message::new(msg_type, payload)))
            }

            MessageType::CommandResponse => {
                let min_required = MESSAGE_TYPE_SIZE + U8_SIZE + U32_SIZE; // type + success byte + string length
                if src.len() < min_required {
                    return Ok(None);
                }

                let str_len = u32::from_be_bytes([src[2], src[3], src[4], src[5]]) as usize;
                let total_needed = MESSAGE_TYPE_SIZE + U8_SIZE + U32_SIZE + str_len;

                if src.len() < total_needed {
                    src.reserve(total_needed - src.len());
                    return Ok(None);
                }

                src.advance(MESSAGE_TYPE_SIZE);

                let payload = src.split_to(U8_SIZE + U32_SIZE + str_len).freeze();

                Ok(Some(Message::new(msg_type, payload)))
            }

            MessageType::Error => {
                let min_required = MESSAGE_TYPE_SIZE + U32_SIZE; // type + string length
                if src.len() < min_required {
                    return Ok(None);
                }

                let str_len = u32::from_be_bytes([src[1], src[2], src[3], src[4]]) as usize;
                let total_needed = MESSAGE_TYPE_SIZE + U32_SIZE + str_len;

                if src.len() < total_needed {
                    src.reserve(total_needed - src.len());
                    return Ok(None);
                }

                src.advance(MESSAGE_TYPE_SIZE);

                let payload = src.split_to(U32_SIZE + str_len).freeze();

                Ok(Some(Message::new(msg_type, payload)))
            }

            MessageType::LaunchApp
            | MessageType::ExecuteShell
            | MessageType::DownloadAndInstallApk
            | MessageType::UninstallApp
            | MessageType::InstallLocalApk => {
                let min_required = MESSAGE_TYPE_SIZE + U32_SIZE; // type + string length
                if src.len() < min_required {
                    return Ok(None);
                }

                let length = u32::from_be_bytes([src[1], src[2], src[3], src[4]]) as usize;
                let total_needed = MESSAGE_TYPE_SIZE + U32_SIZE + length;

                if src.len() < total_needed {
                    src.reserve(total_needed - src.len());
                    return Ok(None);
                }

                src.advance(MESSAGE_TYPE_SIZE + U32_SIZE);

                let payload = src.split_to(length).freeze();

                Ok(Some(Message::new(msg_type, payload)))
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

