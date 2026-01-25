use crate::error::AppError;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};

pub struct MachineId(pub String);

impl<S> FromRequestParts<S> for MachineId
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let machine_id = parts
            .headers
            .get("X-Machine-ID")
            .and_then(|value| value.to_str().ok())
            .ok_or(AppError::InvalidMachineId)?;

        // Normalize machine ID by stripping hyphens (support both formats)
        let normalized = machine_id.replace("-", "");

        Ok(MachineId(normalized))
    }
}

// Allow extracting the inner String directly
impl From<MachineId> for String {
    fn from(id: MachineId) -> Self {
        id.0
    }
}

/// IAP (Identity-Aware Proxy) authenticated user
/// Extracts user email from Google Cloud IAP headers
pub struct IapUser {
    #[allow(dead_code)]
    pub email: String,
}

impl<S> FromRequestParts<S> for IapUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract email from IAP header
        let email = parts
            .headers
            .get("X-Goog-Authenticated-User-Email")
            .and_then(|value| value.to_str().ok());

        // If no IAP header, use a default email (for development with IP whitelisting)
        let email = email
            .unwrap_or("admin@localhost")
            .to_string();

        // IAP prefixes emails with "accounts.google.com:" - strip it
        let email = email
            .strip_prefix("accounts.google.com:")
            .unwrap_or(&email)
            .to_string();

        Ok(IapUser { email })
    }
}
