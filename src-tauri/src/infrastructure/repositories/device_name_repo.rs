use crate::domain::models::Serial;
use crate::domain::repositories::device_name_repository::{DeviceNameRepository, Result};
use async_trait::async_trait;
use std::path::Path;

pub struct SledDeviceNameRepository {
    db: sled::Db,
}

impl SledDeviceNameRepository {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = sled::open(path)
            .map_err(|e| crate::domain::repositories::RepositoryError::DatabaseError(e.to_string()))?;

        Ok(Self { db })
    }

    /// Create repository from an existing Sled database instance
    pub fn from_db(db: sled::Db) -> Self {
        Self { db }
    }

    fn serial_to_key(serial: &Serial) -> Vec<u8> {
        format!("device_name:{}", serial.as_str()).into_bytes()
    }
}

#[async_trait]
impl DeviceNameRepository for SledDeviceNameRepository {
    async fn get_name(&self, serial: &Serial) -> Result<Option<String>> {
        let key = Self::serial_to_key(serial);

        match self.db.get(&key) {
            Ok(Some(value)) => {
                let name = String::from_utf8(value.to_vec())
                    .map_err(|e| crate::domain::repositories::RepositoryError::SerializationError(e.to_string()))?;
                Ok(Some(name))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(crate::domain::repositories::RepositoryError::DatabaseError(e.to_string())),
        }
    }

    async fn set_name(&self, serial: &Serial, name: Option<String>) -> Result<()> {
        let key = Self::serial_to_key(serial);

        match name {
            Some(name) => {
                self.db
                    .insert(&key, name.as_bytes())
                    .map_err(|e| crate::domain::repositories::RepositoryError::DatabaseError(e.to_string()))?;
            }
            None => {
                self.db
                    .remove(&key)
                    .map_err(|e| crate::domain::repositories::RepositoryError::DatabaseError(e.to_string()))?;
            }
        }

        Ok(())
    }
}