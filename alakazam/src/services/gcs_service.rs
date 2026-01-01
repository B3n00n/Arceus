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
    client_email: String,
    private_key_pem: String,
    url_duration_secs: u32,
}

impl GcsService {
    pub async fn new(
        bucket_name: String,
        service_account_path: String,
        duration_secs: u32,
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
        })
    }

    /// Get the URL duration in seconds
    pub fn get_url_duration_secs(&self) -> u32 {
        self.url_duration_secs
    }

    /// Generate a signed URL for downloading an object (v4 signing)
    pub async fn generate_signed_download_url(&self, object_path: &str) -> Result<String> {
        use chrono::Utc;

        let now = Utc::now();
        let expiration = self.url_duration_secs;
        let timestamp = now.format("%Y%m%dT%H%M%SZ").to_string();
        let datestamp = now.format("%Y%m%d").to_string();

        let encoded_path = object_path
            .split('/')
            .map(|segment| percent_encode(segment.as_bytes(), QUERY_ENCODE_SET).to_string())
            .collect::<Vec<_>>()
            .join("/");

        let method = "GET";
        let canonical_uri = format!("/{}/{}", self.bucket_name, encoded_path);
        let credential_scope = format!("{}/auto/storage/goog4_request", datestamp);
        let credential = format!("{}/{}", self.client_email, credential_scope);

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

        // Sign using RSA-SHA256 with private key
        let signature = self.sign_string(&string_to_sign)?;

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

    /// Get OAuth2 access token using service account
    async fn get_access_token(&self) -> Result<String> {
        use chrono::Utc;
        use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize)]
        struct Claims {
            iss: String,
            scope: String,
            aud: String,
            exp: i64,
            iat: i64,
        }

        let now = Utc::now().timestamp();
        let claims = Claims {
            iss: self.client_email.clone(),
            scope: "https://www.googleapis.com/auth/devstorage.full_control".to_string(),
            aud: "https://oauth2.googleapis.com/token".to_string(),
            exp: now + 3600,
            iat: now,
        };

        let encoding_key = EncodingKey::from_rsa_pem(self.private_key_pem.as_bytes())
            .map_err(|e| AppError::Internal(format!("Failed to create encoding key: {}", e)))?;

        let jwt = encode(&Header::new(Algorithm::RS256), &claims, &encoding_key)
            .map_err(|e| AppError::Internal(format!("Failed to encode JWT: {}", e)))?;

        // Exchange JWT for access token
        let client = reqwest::Client::new();
        let response = client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ])
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to get access token: {}", e)))?;

        #[derive(Deserialize)]
        struct TokenResponse {
            access_token: String,
        }

        let token_response: TokenResponse = response
            .json()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse token response: {}", e)))?;

        Ok(token_response.access_token)
    }

    /// Upload a file to GCS (for small files, up to 100MB)
    pub async fn upload_file(
        &self,
        object_path: &str,
        content_type: &str,
        data: Vec<u8>,
    ) -> Result<()> {
        use reqwest::Client;

        let token = self.get_access_token().await?;
        let upload_url = format!(
            "https://storage.googleapis.com/upload/storage/v1/b/{}/o?uploadType=media&name={}",
            self.bucket_name,
            percent_encoding::utf8_percent_encode(object_path, &percent_encoding::NON_ALPHANUMERIC)
        );

        let client = Client::new();
        let response = client
            .post(&upload_url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", content_type)
            .header("Content-Length", data.len())
            .body(data)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to upload to GCS: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "GCS upload failed with status {}: {}",
                status, error_text
            )));
        }

        Ok(())
    }

    /// Upload a large file to GCS using resumable upload protocol
    /// This method receives the full file data and uploads it in chunks to GCS
    pub async fn upload_file_resumable(
        &self,
        object_path: &str,
        content_type: &str,
        data: Vec<u8>,
    ) -> Result<()> {
        use reqwest::Client;

        // Step 1: Initiate resumable upload session
        let token = self.get_access_token().await?;
        let initiate_url = format!(
            "https://storage.googleapis.com/upload/storage/v1/b/{}/o?uploadType=resumable&name={}",
            self.bucket_name,
            percent_encoding::utf8_percent_encode(object_path, &percent_encoding::NON_ALPHANUMERIC)
        );

        let client = Client::new();
        let initiate_response = client
            .post(&initiate_url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .header("Content-Length", "0")
            .header("X-Upload-Content-Type", content_type)
            .header("X-Upload-Content-Length", data.len().to_string())
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to initiate resumable upload: {}", e)))?;

        if !initiate_response.status().is_success() {
            let status = initiate_response.status();
            let error_text = initiate_response.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "GCS resumable upload initiation failed with status {}: {}",
                status, error_text
            )));
        }

        // Extract session URL from Location header
        let session_url = initiate_response
            .headers()
            .get("Location")
            .ok_or_else(|| AppError::Internal("Missing Location header in resumable upload response".to_string()))?
            .to_str()
            .map_err(|e| AppError::Internal(format!("Invalid Location header: {}", e)))?
            .to_string();

        // Step 2: Upload the file data to the session URL
        let upload_response = client
            .put(&session_url)
            .header("Content-Type", content_type)
            .header("Content-Length", data.len())
            .body(data)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to upload data to GCS: {}", e)))?;

        if !upload_response.status().is_success() {
            let status = upload_response.status();
            let error_text = upload_response.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!(
                "GCS resumable upload failed with status {}: {}",
                status, error_text
            )));
        }

        Ok(())
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
