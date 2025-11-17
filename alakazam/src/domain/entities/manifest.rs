use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// File information within a manifest
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileInfo {
    pub hash: String,
    pub size: u64,
}

impl FileInfo {
    pub fn new(hash: String, size: u64) -> Self {
        Self { hash, size }
    }
}

/// Game manifest containing file hashes for delta updates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameManifest {
    pub version: String,
    pub files: HashMap<String, FileInfo>,
}

impl GameManifest {
    pub fn new(version: String) -> Self {
        Self {
            version,
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, path: String, hash: String, size: u64) {
        self.files.insert(path, FileInfo::new(hash, size));
    }

    /// Compare this manifest with another to find changed files
    /// Returns list of file paths that are new or changed in `other`
    pub fn diff(&self, other: &GameManifest) -> Vec<String> {
        let mut changed_files = Vec::new();

        for (path, info) in &other.files {
            match self.files.get(path) {
                Some(old_info) if old_info.hash == info.hash => {
                    // File unchanged
                }
                _ => {
                    // File is new or changed
                    changed_files.push(path.clone());
                }
            }
        }

        changed_files
    }

    /// Get files that were removed in `other`
    pub fn removed_files(&self, other: &GameManifest) -> Vec<String> {
        self.files
            .keys()
            .filter(|path| !other.files.contains_key(*path))
            .cloned()
            .collect()
    }

    /// Total size of all files
    pub fn total_size(&self) -> u64 {
        self.files.values().map(|f| f.size).sum()
    }

    /// Number of files
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_diff() {
        let mut old = GameManifest::new("1.0.0".into());
        old.add_file("a.txt".into(), "hash_a".into(), 100);
        old.add_file("b.txt".into(), "hash_b".into(), 200);

        let mut new = GameManifest::new("1.1.0".into());
        new.add_file("a.txt".into(), "hash_a".into(), 100); // unchanged
        new.add_file("b.txt".into(), "hash_b_new".into(), 250); // changed
        new.add_file("c.txt".into(), "hash_c".into(), 300); // new

        let changed = old.diff(&new);
        assert_eq!(changed.len(), 2);
        assert!(changed.contains(&"b.txt".into()));
        assert!(changed.contains(&"c.txt".into()));
    }

    #[test]
    fn test_removed_files() {
        let mut old = GameManifest::new("1.0.0".into());
        old.add_file("a.txt".into(), "hash_a".into(), 100);
        old.add_file("b.txt".into(), "hash_b".into(), 200);

        let mut new = GameManifest::new("1.1.0".into());
        new.add_file("a.txt".into(), "hash_a".into(), 100);

        let removed = old.removed_files(&new);
        assert_eq!(removed, vec!["b.txt".to_string()]);
    }
}
