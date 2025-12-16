use crate::error::AppError;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};

/// Extractor for API key from X-API-Key header
pub struct ApiKey(pub String);

impl<S> FromRequestParts<S> for ApiKey
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let api_key = parts
            .headers
            .get("X-API-Key")
            .and_then(|value| value.to_str().ok())
            .ok_or(AppError::InvalidApiKey)?;

        Ok(ApiKey(api_key.to_string()))
    }
}

// Allow extracting the inner String directly
impl From<ApiKey> for String {
    fn from(key: ApiKey) -> Self {
        key.0
    }
}
