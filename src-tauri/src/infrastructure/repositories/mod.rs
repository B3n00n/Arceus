// Repository implementations
//
// Concrete implementations of repository traits.

mod in_memory_device_repo;
mod sqlite_device_name_repo;
mod fs_apk_repo;
mod fs_client_apk_repo;
mod fs_game_version_repo;
mod sqlite_game_cache_repo;

// Re-export repository implementations
pub use in_memory_device_repo::InMemoryDeviceRepository;
pub use sqlite_device_name_repo::SqliteDeviceNameRepository;
pub use fs_apk_repo::FsApkRepository;
pub use fs_client_apk_repo::FsClientApkRepository;
pub use fs_game_version_repo::FsGameVersionRepository;
pub use sqlite_game_cache_repo::SqliteGameCacheRepository;
