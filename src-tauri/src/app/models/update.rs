use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateInfo {
    pub version: String,
    pub current_version: String,
    pub notes: Option<String>,
    pub body: Option<String>,
    pub date: Option<String>,
    pub pub_date: Option<String>,
    pub is_available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateProgress {
    pub chunk_size: u64,
    pub content_length: Option<u64>,
    pub downloaded: u64,
}

impl UpdateProgress {
    pub fn new(chunk_size: u64, content_length: Option<u64>, downloaded: u64) -> Self {
        Self {
            chunk_size,
            content_length,
            downloaded,
        }
    }

    pub fn percentage(&self) -> Option<f64> {
        self.content_length.map(|total| {
            if total > 0 {
                (self.downloaded as f64 / total as f64) * 100.0
            } else {
                0.0
            }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum UpdateStatus {
    Checking,
    UpdateAvailable(UpdateInfo),
    NoUpdate,
    Downloading {
        progress: f64,
        bytes_downloaded: u64,
        total_bytes: u64,
    },
    Downloaded,
    Installing,
    Installed,
    Complete,
    Error {
        message: String,
    },
}
