use serde::{Deserialize, Serialize};

use crate::domain::value_objects::ClientId;

/// Token claims for authenticated clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: ClientId,
    pub iat: usize,
}

/// Request to exchange API key for token
#[derive(Debug, Clone, Deserialize)]
pub struct AuthRequest {
    pub api_key: String,
}

/// Response containing the session token
#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub client_id: ClientId,
}
