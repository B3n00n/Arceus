/// Serial number value object
/// Represents a device serial number.
/// Provides validation and normalization.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A validated device serial number
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Serial(String);

#[derive(Debug, thiserror::Error)]
pub enum SerialError {
    #[error("Invalid serial number format: {0}. Must contain only alphanumeric characters, colons, hyphens, or underscores")]
    InvalidFormat(String),

    #[error("Serial number cannot be empty")]
    Empty,

    #[error("Serial number too long: {0}. Maximum length is 64 characters")]
    TooLong(usize),
}

impl Serial {
    /// Create a new Serial from a string, validating the format
    pub fn new(value: String) -> Result<Self, SerialError> {
        if value.is_empty() {
            return Err(SerialError::Empty);
        }

        if value.len() > 64 {
            return Err(SerialError::TooLong(value.len()));
        }

        if !Self::is_valid_format(&value) {
            return Err(SerialError::InvalidFormat(value));
        }

        Ok(Self(value.to_lowercase()))
    }

    /// Get the serial number as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if a string is a valid serial number format
    fn is_valid_format(s: &str) -> bool {
        s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == ':' || c == '-' || c == '_')
    }
}

impl fmt::Display for Serial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for Serial {
    type Error = SerialError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Serial {
    type Error = SerialError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_string())
    }
}

impl AsRef<str> for Serial {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
