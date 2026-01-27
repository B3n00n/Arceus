use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub tcp_host: String,
    pub tcp_port: u16,
    pub http_port: u16,
    pub max_connections: usize,
    pub battery_update_interval: u64,
    pub heartbeat_timeout: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            tcp_host: "0.0.0.0".to_string(),
            tcp_port: 43572,
            http_port: 43573,
            max_connections: 100,
            battery_update_interval: 60,
            heartbeat_timeout: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlakazamConfig {
    pub base_url: String,
    pub snorlax_endpoint: String,
}

impl Default for AlakazamConfig {
    fn default() -> Self {
        Self {
            base_url: "https://alakazam-yexfczgpca-uc.a.run.app".to_string(),
            //base_url: "http://localhost:43571".to_string(),
            snorlax_endpoint: "/api/arcade/snorlax/latest".to_string(),
        }
    }
}
