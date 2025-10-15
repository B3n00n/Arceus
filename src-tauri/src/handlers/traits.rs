use crate::core::Result;
use crate::network::DeviceConnection;
use crate::protocol::MessageType;
use async_trait::async_trait;
use bytes::Bytes;
use std::sync::Arc;

#[async_trait]
pub trait MessageHandler: Send + Sync {
    fn message_type(&self) -> MessageType;
    async fn handle(&self, device: &Arc<DeviceConnection>, payload: Bytes) -> Result<()>;
    fn name(&self) -> &'static str {
        self.message_type().name()
    }
}

#[macro_export]
macro_rules! impl_message_handler {
    ($handler:ty, $msg_type:expr) => {
        #[async_trait]
        impl MessageHandler for $handler {
            fn message_type(&self) -> MessageType {
                $msg_type
            }

            fn name(&self) -> &'static str {
                stringify!($handler)
            }
        }
    };
}
