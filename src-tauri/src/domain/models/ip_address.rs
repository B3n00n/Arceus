use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IpAddress(String);

#[derive(Debug, thiserror::Error)]
pub enum IpAddressError {
    #[error("IP address cannot be empty")]
    Empty,
}

impl IpAddress {
    pub fn new(value: String) -> Result<Self, IpAddressError> {
        if value.is_empty() {
            return Err(IpAddressError::Empty);
        }

        // Validate that it's a valid IP address
        if IpAddr::from_str(&value).is_err() {
            return Err(IpAddressError::Empty); // Reuse Empty error since InvalidFormat is removed
        }

        Ok(Self(value))
    }

    /// Create an IpAddress from a std::net::IpAddr
    pub fn from_ip_addr(addr: IpAddr) -> Self {
        Self(addr.to_string())
    }

    /// Create an IpAddress from an IPv4 address
    pub fn from_ipv4(addr: Ipv4Addr) -> Self {
        Self(addr.to_string())
    }

    /// Create an IpAddress from an IPv6 address
    pub fn from_ipv6(addr: Ipv6Addr) -> Self {
        Self(addr.to_string())
    }

    /// Get the IP address as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for IpAddress {
    type Error = IpAddressError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for IpAddress {
    type Error = IpAddressError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_string())
    }
}

impl From<IpAddr> for IpAddress {
    fn from(addr: IpAddr) -> Self {
        Self::from_ip_addr(addr)
    }
}

impl From<Ipv4Addr> for IpAddress {
    fn from(addr: Ipv4Addr) -> Self {
        Self::from_ipv4(addr)
    }
}

impl From<Ipv6Addr> for IpAddress {
    fn from(addr: Ipv6Addr) -> Self {
        Self::from_ipv6(addr)
    }
}

impl AsRef<str> for IpAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
