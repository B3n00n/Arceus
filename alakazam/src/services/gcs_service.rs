use crate::error::{AppError, Result};
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};
use serde::Deserialize;
use sha2::Digest;
use std::sync::Arc;

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
struct GcsListResponse {
    items: Option<Vec<GcsObject>>,
}

#[derive(Debug, Deserialize)]
struct GcsObject {
    name: String,
}

pub struct GcsService {
    bucket_name: String,
    url_duration_secs: u32,
    token_provider: Arc<dyn gcp_auth::TokenProvider>,
}

impl GcsService {
    pub async fn new(bucket_name: String, duration_secs: u32) -> Result<Self> {
        // Initialize Application Default Credentials
        let token_provider = gcp_auth::provider()
            .await
            .map_err(|e| {
                AppError::Internal(format!("Failed to initialize GCP authentication: {}", e))
            })?;

        Ok(Self {
            bucket_name,
            url_duration_secs: duration_secs,
            token_provider,
        })
    }

    /// Get the URL duration in seconds
    pub fn get_url_duration_secs(&self) -> u32 {
        self.url_duration_secs
    }

    /// Generate a signed URL for downloading an object (v4 signing)
    pub async fn generate_signed_download_url(&self, object_path: &str) -> Result<String> {
        self.generate_signed_url(object_path, "GET", self.url_duration_secs).await
    }

    /// Generate a signed URL for uploading an object (v4 signing)
    pub async fn generate_signed_upload_url(&self, object_path: &str, duration_secs: u32) -> Result<String> {
        self.generate_signed_url(object_path, "PUT", duration_secs).await
    }

    /// Internal method to generate signed URLs for both upload and download
    async fn generate_signed_url(&self, object_path: &str, method: &str, expiration: u32) -> Result<String> {
        use chrono::Utc;

        let now = Utc::now();
        let timestamp = now.format("%Y%m%dT%H%M%SZ").to_string();
        let datestamp = now.format("%Y%m%d").to_string();

        let encoded_path = object_path
            .split('/')
            .map(|segment| percent_encode(segment.as_bytes(), QUERY_ENCODE_SET).to_string())
            .collect::<Vec<_>>()
            .join("/");

        let canonical_uri = format!("/{}/{}", self.bucket_name, encoded_path);
        let credential_scope = format!("{}/auto/storage/goog4_request", datestamp);

        // Get service account email from GCP metadata server
        let client_email = self.get_service_account_email().await?;

        let credential = format!("{}/{}", client_email, credential_scope);

        let encoded_credential = percent_encode(credential.as_bytes(), QUERY_ENCODE_SET).to_string();
        let canonical_query_string = format!(
            "X-Goog-Algorithm=GOOG4-RSA-SHA256&X-Goog-Credential={}&X-Goog-Date={}&X-Goog-Expires={}&X-Goog-SignedHeaders=host",
            encoded_credential,
            timestamp,
            expiration
        );

        let canonical_headers = "host:storage.googleapis.com\n";
        let signed_headers = "host";

        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\nUNSIGNED-PAYLOAD",
            method, canonical_uri, canonical_query_string, canonical_headers, signed_headers
        );

        let canonical_request_hash = format!("{:x}", sha2::Sha256::digest(canonical_request.as_bytes()));
        let string_to_sign = format!(
            "GOOG4-RSA-SHA256\n{}\n{}\n{}",
            timestamp, credential_scope, canonical_request_hash
        );

        // Sign the string (using either local key or IAM signBlob API)
        let signature = self.sign_string(&string_to_sign, &client_email).await?;

        // Build final URL
        let url = format!(
            "https://storage.googleapis.com{canonical_uri}?{canonical_query_string}&X-Goog-Signature={signature}"
        );

        Ok(url)
    }

    /// List all files in a GCS folder and generate signed URLs for each
    /// Returns a list of relative paths and their signed download URLs
    pub async fn list_and_sign_folder(&self, folder_path: &str) -> Result<Vec<crate::api::handlers::GameFile>> {
        use reqwest::Client;

        // Get OAuth2 token for GCS API access
        let token = self.get_access_token().await?;

        // List objects with the folder prefix
        let list_url = format!(
            "https://storage.googleapis.com/storage/v1/b/{}/o?prefix={}",
            self.bucket_name,
            percent_encoding::utf8_percent_encode(folder_path, &percent_encoding::NON_ALPHANUMERIC)
        );

        let client = Client::new();
        let response = client
            .get(&list_url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to list GCS objects: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Internal(format!(
                "GCS list failed with status {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let list_response: GcsListResponse = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse GCS list response: {}", e)))?;

        // Generate signed URLs for each file
        let mut files = Vec::new();
        for item in list_response.items.unwrap_or_default() {
            // Skip directories (objects ending with /)
            if item.name.ends_with('/') {
                continue;
            }

            let download_url = self.generate_signed_download_url(&item.name).await?;

            // Get relative path (remove the folder prefix)
            let relative_path = item.name
                .strip_prefix(&format!("{}/", folder_path))
                .unwrap_or(&item.name)
                .to_string();

            files.push(crate::api::handlers::GameFile {
                path: relative_path,
                download_url,
            });
        }

        Ok(files)
    }

    /// Get service account email from GCP metadata server or environment variable
    async fn get_service_account_email(&self) -> Result<String> {
        // Try environment variable first (for local development)
        if let Ok(email) = std::env::var("GCP_SERVICE_ACCOUNT_EMAIL") {
            return Ok(email);
        }

        // Try GCP metadata server (for Cloud Run/GCE)
        // The /email endpoint returns plain text, not JSON
        let client = reqwest::Client::new();
        let response = client
            .get("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/email")
            .header("Metadata-Flavor", "Google")
            .send()
            .await
            .map_err(|e| {
                AppError::Internal(format!(
                    "Failed to get service account email. For local development, set GCP_SERVICE_ACCOUNT_EMAIL environment variable. Error: {}",
                    e
                ))
            })?;

        if !response.status().is_success() {
            return Err(AppError::Internal(format!(
                "Metadata server returned status {}: {}. For local development, set GCP_SERVICE_ACCOUNT_EMAIL environment variable.",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        // The response is plain text (just the email address)
        let email = response
            .text()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to read service account email: {}", e)))?
            .trim()
            .to_string();

        Ok(email)
    }

    /// Get OAuth2 access token using Application Default Credentials
    async fn get_access_token(&self) -> Result<String> {
        let scopes = &["https://www.googleapis.com/auth/devstorage.full_control"];
        let token = self.token_provider
            .token(scopes)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to get token from ADC: {}", e)))?;
        Ok(token.as_str().to_string())
    }

    /// Delete a file from GCS
    pub async fn delete_file(&self, object_path: &str) -> Result<()> {
        use reqwest::Client;

        let token = self.get_access_token().await?;
        let delete_url = format!(
            "https://storage.googleapis.com/storage/v1/b/{}/o/{}",
            self.bucket_name,
            percent_encoding::utf8_percent_encode(object_path, &percent_encoding::NON_ALPHANUMERIC)
        );

        let client = Client::new();
        let response = client
            .delete(&delete_url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete from GCS: {}", e)))?;

        if !response.status().is_success() && response.status().as_u16() != 404 {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "GCS delete failed with status {}: {}",
                status, error_text
            )));
        }

        Ok(())
    }

    /// Delete all files with a specific prefix (folder)
    pub async fn delete_folder(&self, folder_path: &str) -> Result<()> {
        use reqwest::Client;

        // List all objects with the prefix
        let token = self.get_access_token().await?;
        let list_url = format!(
            "https://storage.googleapis.com/storage/v1/b/{}/o?prefix={}",
            self.bucket_name,
            percent_encoding::utf8_percent_encode(folder_path, &percent_encoding::NON_ALPHANUMERIC)
        );

        let client = Client::new();
        let response = client
            .get(&list_url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to list GCS objects: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Internal(format!(
                "GCS list failed with status {}: {}",
                response.status(),
                response.text().await.unwrap_or_default()
            )));
        }

        let list_response: GcsListResponse = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse GCS list response: {}", e)))?;

        // Delete each file
        for item in list_response.items.unwrap_or_default() {
            self.delete_file(&item.name).await?;
        }

        Ok(())
    }

    /// Sign a string using IAM signBlob API
    async fn sign_string(&self, message: &str, service_account_email: &str) -> Result<String> {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize)]
        struct SignBlobRequest {
            payload: String,
        }

        #[derive(Deserialize)]
        #[allow(dead_code)]
        struct SignBlobResponse {
            #[serde(rename = "keyId")]
            key_id: String,
            #[serde(rename = "signedBlob")]
            signed_blob: String,
        }

        let token = self.get_access_token().await?;
        let sign_url = format!(
            "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/{}:signBlob",
            service_account_email
        );

        let payload_base64 = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            message.as_bytes()
        );

        let request_body = SignBlobRequest {
            payload: payload_base64,
        };

        let client = reqwest::Client::new();
        let response = client
            .post(&sign_url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to call signBlob API: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "signBlob API failed with status {}: {}",
                status, error_text
            )));
        }

        let sign_response: SignBlobResponse = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse signBlob response: {}", e)))?;

        // Decode the base64 signature and encode as hex
        let signature_bytes = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            sign_response.signed_blob
        ).map_err(|e| AppError::Internal(format!("Failed to decode signature: {}", e)))?;

        Ok(hex::encode(signature_bytes))
    }
}
