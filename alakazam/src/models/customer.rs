use serde::Serialize;
use sqlx::types::chrono::{DateTime, Utc};

/// Customer entity from database
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
}
