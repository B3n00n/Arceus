use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::ClientId;
use crate::domain::entities::Client;

/// Request to create a new client
#[derive(Debug, Clone, Deserialize)]
pub struct CreateClientRequest {
    pub name: String,
}

/// Request to update an existing client
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateClientRequest {
    pub name: Option<String>,
}

/// Client response DTO (excludes sensitive API key)
#[derive(Debug, Clone, Serialize)]
pub struct ClientResponse {
    pub id: ClientId,
    pub name: String,
    pub last_seen: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<Client> for ClientResponse {
    fn from(client: Client) -> Self {
        Self {
            id: client.id,
            name: client.name,
            last_seen: client.last_seen,
            created_at: client.created_at,
        }
    }
}

/// Client response with API key (only for creation)
#[derive(Debug, Clone, Serialize)]
pub struct ClientWithKeyResponse {
    pub id: ClientId,
    pub name: String,
    pub api_key: String,
    pub created_at: DateTime<Utc>,
}

impl From<Client> for ClientWithKeyResponse {
    fn from(client: Client) -> Self {
        Self {
            id: client.id,
            name: client.name,
            api_key: client.api_key,
            created_at: client.created_at,
        }
    }
}
