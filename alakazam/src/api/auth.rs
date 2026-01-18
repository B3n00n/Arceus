use crate::error::AppError;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};

pub struct MacKey(pub String);

impl<S> FromRequestParts<S> for MacKey
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let mac_address = parts
            .headers
            .get("X-MAC-Address")
            .and_then(|value| value.to_str().ok())
            .ok_or(AppError::InvalidMacAddress)?;

        Ok(MacKey(mac_address.to_string()))
    }
}

// Allow extracting the inner String directly
impl From<MacKey> for String {
    fn from(key: MacKey) -> Self {
        key.0
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
