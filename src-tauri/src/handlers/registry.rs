use super::traits::MessageHandler;
use crate::core::{error::HandlerError, Result};
use crate::network::DeviceConnection;
use crate::protocol::MessageType;
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;

pub struct HandlerRegistry {
    handlers: HashMap<MessageType, Arc<dyn MessageHandler>>,
}

impl HandlerRegistry {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<H>(&mut self, handler: H) -> &mut Self
    where
        H: MessageHandler + 'static,
    {
        let msg_type = handler.message_type();
        self.handlers.insert(msg_type, Arc::new(handler));
        tracing::debug!("Registered handler for {}", msg_type);
        self
    }

    pub fn get(&self, msg_type: MessageType) -> Option<Arc<dyn MessageHandler>> {
        self.handlers.get(&msg_type).map(Arc::clone)
    }

    pub async fn handle(
        &self,
        msg_type: MessageType,
        device: &Arc<DeviceConnection>,
        payload: Bytes,
    ) -> Result<()> {
        let handler = self
            .get(msg_type)
            .ok_or_else(|| HandlerError::HandlerNotRegistered(msg_type.to_u8()))?;

        tracing::trace!(
            "Routing {} message from device {} to handler",
            msg_type,
            device.id()
        );

        handler.handle(device, payload).await.map_err(|e| {
            tracing::error!(
                "Handler {} failed for device {}: {}",
                handler.name(),
                device.id(),
                e
            );
            e
        })
    }
}

impl Default for HandlerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

