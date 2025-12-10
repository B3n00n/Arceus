/// Packet handler implementations organized by category

pub mod connection;
pub mod status;
pub mod app;
pub mod responses;

pub use connection::{DeviceConnectedHandler, HeartbeatHandler, VersionCheckHandler};
pub use status::{BatteryStatusHandler, VolumeStatusHandler};
pub use app::ForegroundAppChangedHandler;
pub use responses::*;
