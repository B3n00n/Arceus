use crate::core::{error::StorageError, Result};
use sled::Db;
use std::collections::HashMap;
use std::path::Path;

pub struct DeviceNamesStore {
    db: Db,
}

impl DeviceNamesStore {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let db = sled::open(path).map_err(|e| {
            StorageError::Database(format!("Failed to open database: {}", e))
        })?;

        Ok(Self { db })
    }

    pub fn set_name(&self, serial: String, name: String) -> Result<()> {
        self.db
            .insert(serial.as_bytes(), name.as_bytes())
            .map_err(|e| StorageError::WriteFailed(format!("Failed to set name: {}", e)))?;

        self.db
            .flush()
            .map_err(|e| StorageError::WriteFailed(format!("Failed to flush database: {}", e)))?;

        tracing::debug!("Set custom name for device {}: {}", serial, name);

        Ok(())
    }

    pub fn get_name(&self, serial: &str) -> Option<String> {
        self.db
            .get(serial.as_bytes())
            .ok()?
            .map(|bytes| String::from_utf8_lossy(&bytes).to_string())
    }

    pub fn remove_name(&self, serial: &str) -> Result<()> {
        self.db
            .remove(serial.as_bytes())
            .map_err(|e| StorageError::WriteFailed(format!("Failed to remove name: {}", e)))?;

        self.db
            .flush()
            .map_err(|e| StorageError::WriteFailed(format!("Failed to flush database: {}", e)))?;

        tracing::debug!("Removed custom name for device {}", serial);

        Ok(())
    }

    pub fn get_all_names(&self) -> HashMap<String, String> {
        self.db
            .iter()
            .filter_map(|result| result.ok())
            .map(|(key, value)| {
                let serial = String::from_utf8_lossy(&key).to_string();
                let name = String::from_utf8_lossy(&value).to_string();
                (serial, name)
            })
            .collect()
    }
}
