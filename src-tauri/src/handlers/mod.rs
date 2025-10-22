// Core handler infrastructure
pub mod registry;
pub mod traits;

// Response packet handlers
pub mod apk_install_handler;
pub mod installed_apps_handler;
pub mod launch_app_handler;
pub mod ping_handler;
pub mod shell_execution_handler;
pub mod shutdown_handler;
pub mod uninstall_app_handler;
pub mod volume_set_handler;

// Re-exports
pub use apk_install_handler::ApkInstallHandler;
pub use installed_apps_handler::InstalledAppsHandler;
pub use launch_app_handler::LaunchAppHandler;
pub use ping_handler::PingHandler;
pub use registry::HandlerRegistry;
pub use shell_execution_handler::ShellExecutionHandler;
pub use shutdown_handler::ShutdownHandler;
pub use traits::PacketHandler;
pub use uninstall_app_handler::UninstallAppHandler;
pub use volume_set_handler::VolumeSetHandler;
