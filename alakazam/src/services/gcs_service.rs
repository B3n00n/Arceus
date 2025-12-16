use crate::error::{AppError, Result};
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fs;

// Based on RFC 3986, encode everything except unreserved characters (A-Z, a-z, 0-9, -, ., _, ~)
const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'?')
    .add(b'{')
    .add(b'}')
    .add(b'/')
    .add(b'@')
    .add(b':')
    .add(b'[')
    .add(b']')
    .add(b'=')
    .add(b'&')
    .add(b'%')
    .add(b'+');

#[derive(Debug, Deserialize)]
struct ServiceAccountKey {
    client_email: String,
    private_key: String,
}

pub struct GcsService {
    bucket_name: String,
    client_email: String,
    private_key_pem: String,
    url_duration_secs: u32,
    snorlax_version: String,
}

impl GcsService {
    pub async fn new(
        bucket_name: String,
        service_account_path: String,
        duration_secs: u32,
        snorlax_version: String,
    ) -> Result<Self> {
        let key_json = fs::read_to_string(&service_account_path).map_err(|e| {
            AppError::Internal(format!("Failed to read service account key: {}", e))
        })?;

        let key: ServiceAccountKey = serde_json::from_str(&key_json).map_err(|e| {
            AppError::Internal(format!("Failed to parse service account key: {}", e))
        })?;

        Ok(Self {
            bucket_name,
            client_email: key.client_email,
            private_key_pem: key.private_key,
            url_duration_secs: duration_secs,
            snorlax_version,
        })
    }

    /// Get the URL duration in seconds
    pub fn get_url_duration_secs(&self) -> u32 {
        self.url_duration_secs
    }

    /// Get the Snorlax APK version
    pub fn get_snorlax_version(&self) -> &str {
        &self.snorlax_version
    }

    /// Generate a signed URL for downloading an object (v4 signing)
    pub async fn generate_signed_download_url(&self, object_path: &str) -> Result<String> {
        use chrono::Utc;

        let now = Utc::now();
        let expiration = self.url_duration_secs;
        let timestamp = now.format("%Y%m%dT%H%M%SZ").to_string();
        let datestamp = now.format("%Y%m%d").to_string();

        // Canonical request components
        let method = "GET";
        let canonical_uri = format!("/{}/{}", self.bucket_name, object_path);
        let credential_scope = format!("{}/auto/storage/goog4_request", datestamp);
        let credential = format!("{}/{}", self.client_email, credential_scope);

        // Query parameters (must be in this exact order, not sorted)
        let encoded_credential = percent_encode(credential.as_bytes(), QUERY_ENCODE_SET).to_string();
        let canonical_query_string = format!(
            "X-Goog-Algorithm=GOOG4-RSA-SHA256&X-Goog-Credential={}&X-Goog-Date={}&X-Goog-Expires={}&X-Goog-SignedHeaders=host",
            encoded_credential,
            timestamp,
            expiration
        );

        // Canonical headers
        let canonical_headers = "host:storage.googleapis.com\n";
        let signed_headers = "host";

        // Create canonical request
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\nUNSIGNED-PAYLOAD",
            method, canonical_uri, canonical_query_string, canonical_headers, signed_headers
        );

        // Create string to sign
        let canonical_request_hash = format!("{:x}", sha2::Sha256::digest(canonical_request.as_bytes()));
        let string_to_sign = format!(
            "GOOG4-RSA-SHA256\n{}\n{}\n{}",
            timestamp, credential_scope, canonical_request_hash
        );

        // Sign using RSA-SHA256 with private key
        let signature = self.sign_string(&string_to_sign)?;

        // Build final URL
        let url = format!(
            "https://storage.googleapis.com{canonical_uri}?{canonical_query_string}&X-Goog-Signature={signature}"
        );

        Ok(url)
    }

    /// Sign a string using RSA-SHA256 with the private key
    fn sign_string(&self, message: &str) -> Result<String> {
        use rsa::pkcs1v15::SigningKey;
        use rsa::pkcs8::DecodePrivateKey;
        use rsa::signature::{SignatureEncoding, Signer};
        use rsa::RsaPrivateKey;

        // Parse the PEM private key
        let private_key = RsaPrivateKey::from_pkcs8_pem(&self.private_key_pem)
            .map_err(|e| AppError::Internal(format!("Failed to parse private key: {}", e)))?;

        let signing_key = SigningKey::<Sha256>::new(private_key);
        let signature = signing_key
            .sign(message.as_bytes())
            .to_bytes();

        // Encode signature as hex
        Ok(hex::encode(signature))
    }
}
