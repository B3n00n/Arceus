use crate::application::dto::game_version::{CachedGameEntry, GameAssignment, LocalGameMetadata};
use crate::domain::repositories::RepositoryError;
use std::path::Path;

pub struct SledGameCacheRepository {
    db: sled::Db,
}

impl SledGameCacheRepository {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, RepositoryError> {
        let db = sled::open(path)
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(Self { db })
    }

    /// Create repository from an existing Sled database instance
    pub fn from_db(db: sled::Db) -> Self {
        Self { db }
    }

    /// Generate cache key for a game
    fn game_key(game_id: i32) -> Vec<u8> {
        format!("game_cache:{}", game_id).into_bytes()
    }

    /// Generate name index key for reverse lookup
    fn name_index_key(game_name: &str) -> Vec<u8> {
        format!("game_name_index:{}", game_name).into_bytes()
    }

    /// Get a cached game entry by ID
    pub async fn get_entry(&self, game_id: i32) -> Result<Option<CachedGameEntry>, RepositoryError> {
        let key = Self::game_key(game_id);
        match self.db.get(&key)? {
            Some(value) => {
                let entry: CachedGameEntry = serde_json::from_slice(&value)?;
                Ok(Some(entry))
            }
            None => Ok(None),
        }
    }

    /// Get a cached game entry by name using the name index
    pub async fn get_entry_by_name(&self, game_name: &str) -> Result<Option<CachedGameEntry>, RepositoryError> {
        let index_key = Self::name_index_key(game_name);

        if let Some(game_id_bytes) = self.db.get(&index_key)? {
            let game_id: i32 = String::from_utf8(game_id_bytes.to_vec())
                .map_err(|e| RepositoryError::SerializationError(format!("Invalid UTF-8 in game_id: {}", e)))?
                .parse()
                .map_err(|e| RepositoryError::SerializationError(format!("Invalid game_id: {}", e)))?;
            self.get_entry(game_id).await
        } else {
            Ok(None)
        }
    }

    /// Get all cached game entries
    pub async fn get_all_entries(&self) -> Result<Vec<CachedGameEntry>, RepositoryError> {
        let prefix = b"game_cache:";
        let mut entries = Vec::new();

        for result in self.db.scan_prefix(prefix) {
            let (_key, value) = result?;
            let entry: CachedGameEntry = serde_json::from_slice(&value)?;
            entries.push(entry);
        }

        Ok(entries)
    }

    /// Set (insert or update) a cached game entry
    pub async fn set_entry(&self, entry: &CachedGameEntry) -> Result<(), RepositoryError> {
        let key = Self::game_key(entry.game_id);
        let value = serde_json::to_vec(entry)?;

        // Store main entry
        self.db.insert(&key, value)?;

        // Update name index
        let index_key = Self::name_index_key(&entry.game_name);
        self.db.insert(&index_key, entry.game_id.to_string().as_bytes())?;

        Ok(())
    }

    /// Update only the local metadata portion of a cached entry
    pub async fn update_local_metadata(
        &self,
        game_id: i32,
        metadata: LocalGameMetadata,
    ) -> Result<(), RepositoryError> {
        if let Some(mut entry) = self.get_entry(game_id).await? {
            entry.update_local_metadata(metadata);
            self.set_entry(&entry).await?;
        }
        Ok(())
    }

    /// Sync cache from Alakazam assignments with local metadata lookup
    pub async fn sync_from_assignments<F>(
        &self,
        assignments: Vec<GameAssignment>,
        local_metadata_fn: F,
    ) -> Result<(), RepositoryError>
    where
        F: Fn(&str) -> Option<LocalGameMetadata>,
    {
        for assignment in assignments {
            let local_metadata = local_metadata_fn(&assignment.game_name);
            let entry = CachedGameEntry::from_assignment_and_metadata(assignment, local_metadata);
            self.set_entry(&entry).await?;
        }
        Ok(())
    }

    /// Check if the cache is empty
    pub async fn is_empty(&self) -> Result<bool, RepositoryError> {
        let prefix = b"game_cache:";
        Ok(self.db.scan_prefix(prefix).next().is_none())
    }

    /// Clear all cached entries (for recovery)
    pub async fn clear_all(&self) -> Result<(), RepositoryError> {
        let prefix = b"game_cache:";
        let keys: Vec<_> = self.db.scan_prefix(prefix)
            .filter_map(|r| r.ok().map(|(k, _)| k))
            .collect();

        for key in keys {
            self.db.remove(&key)?;
        }

        // Also clear name indices
        let index_prefix = b"game_name_index:";
        let index_keys: Vec<_> = self.db.scan_prefix(index_prefix)
            .filter_map(|r| r.ok().map(|(k, _)| k))
            .collect();

        for key in index_keys {
            self.db.remove(&key)?;
        }

        Ok(())
    }

    /// Remove a cached entry and its name index
    pub async fn remove_entry(&self, game_id: i32) -> Result<(), RepositoryError> {
        if let Some(entry) = self.get_entry(game_id).await? {
            // Remove name index
            let index_key = Self::name_index_key(&entry.game_name);
            self.db.remove(&index_key)?;

            // Remove main entry
            let key = Self::game_key(game_id);
            self.db.remove(&key)?;
        }
        Ok(())
    }
}
