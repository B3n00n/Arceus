/// Package Name value object
/// Represents an Android package name with validation.
/// Package names follow the format: com.company.app

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PackageName(String);

#[derive(Debug, thiserror::Error)]
pub enum PackageNameError {
    #[error("Package name cannot be empty")]
    Empty,

    #[error("Package name must have at least two segments (e.g., com.example)")]
    TooFewSegments,

    #[error("Package segment '{0}' is invalid: {1}")]
    InvalidSegment(String, String),
}

impl PackageName {
    pub fn new(value: String) -> Result<Self, PackageNameError> {
        if value.is_empty() {
            return Err(PackageNameError::Empty);
        }

        Self::validate(&value)?;

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn validate(name: &str) -> Result<(), PackageNameError> {
        let segments: Vec<&str> = name.split('.').collect();

        if segments.len() < 2 {
            return Err(PackageNameError::TooFewSegments);
        }

        for segment in segments {
            if segment.is_empty() {
                return Err(PackageNameError::InvalidSegment(
                    segment.to_string(),
                    "segment cannot be empty".to_string(),
                ));
            }

            if !segment.chars().next().unwrap().is_ascii_alphabetic() {
                return Err(PackageNameError::InvalidSegment(
                    segment.to_string(),
                    "must start with a letter".to_string(),
                ));
            }

            for ch in segment.chars() {
                if !ch.is_ascii_alphanumeric() && ch != '_' {
                    return Err(PackageNameError::InvalidSegment(
                        segment.to_string(),
                        format!("invalid character '{}'", ch),
                    ));
                }
            }
        }

        Ok(())
    }
}

impl fmt::Display for PackageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for PackageName {
    type Error = PackageNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for PackageName {
    type Error = PackageNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_string())
    }
}

impl AsRef<str> for PackageName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
