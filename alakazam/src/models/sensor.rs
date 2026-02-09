use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Sensor tracked in the database
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Sensor {
    pub id: i32,
    pub serial_number: String,
    pub mac_address: Option<String>,
    pub firmware_version: Option<String>,
    pub arcade_id: Option<i32>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Sensor with arcade name joined
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct SensorWithArcade {
    pub id: i32,
    pub serial_number: String,
    pub mac_address: Option<String>,
    pub firmware_version: Option<String>,
    pub arcade_id: Option<i32>,
    pub arcade_name: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
