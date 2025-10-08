use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    pub version: String,
    pub current_version: String,
    pub body: Option<String>,
    pub date: Option<String>,
    pub is_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum UpdateStatus {
    Checking,
    NoUpdate,
    UpdateAvailable(UpdateInfo),
    Downloading { progress: f64, #[serde(rename = "bytesDownloaded")] bytes_downloaded: u64, #[serde(rename = "totalBytes")] total_bytes: u64 },
    Installing,
    Complete,
    Error { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProgress {
    pub chunk_len: u64,
    pub content_len: Option<u64>,
    pub downloaded: u64,
    pub percentage: Option<f64>,
}

impl UpdateProgress {
    pub fn new(chunk_len: u64, content_len: Option<u64>, downloaded: u64) -> Self {
        let percentage = content_len.map(|total| {
            if total > 0 {
                (downloaded as f64 / total as f64) * 100.0
            } else {
                0.0
            }
        });

        Self {
            chunk_len,
            content_len,
            downloaded,
            percentage,
        }
    }
}