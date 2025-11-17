use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::ClientId;

/// Represents an Arceus client installation at a VR arcade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: ClientId,
    pub api_key: String,
    pub name: String,
    pub last_seen: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Client {
    pub fn new(name: String, api_key: String) -> Self {
        let now = Utc::now();
        Self {
            id: ClientId::new(),
            api_key,
            name,
            last_seen: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn touch(&mut self) {
        self.last_seen = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}
