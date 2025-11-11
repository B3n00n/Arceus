/// Application orchestration layer
/// Manages app-level concerns: config, lifecycle, events
pub mod config;
pub mod error;
pub mod events;
pub mod lifecycle;
pub mod models;
pub mod server_manager;
pub mod signal_handler;

pub use config::AppConfig;
pub use error::Result;
pub use events::EventBus;
pub use lifecycle::AppState;
pub use models::{ApkFile, ServerConfig};
pub use server_manager::ServerManager;
pub use signal_handler::setup_signal_handlers;
