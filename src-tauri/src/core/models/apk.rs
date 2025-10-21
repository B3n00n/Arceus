use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApkFile {
    pub filename: String,
    pub size_bytes: u64,
    pub url: String,
}

impl ApkFile {
    pub fn new(filename: String, size_bytes: u64, url: String) -> Self {
        Self {
            filename,
            size_bytes,
            url,
        }
    }
}
