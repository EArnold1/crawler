//! Content deduplication logic.

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// Trait for content deduplication strategies.
pub trait ContentDeduplicator: Send + Sync {
    /// Returns true if the content hash has been seen before.
    fn is_seen(&self, hash: &[u8]) -> bool;
    /// Marks the content hash as seen.
    fn mark_seen(&self, hash: Vec<u8>);
}

/// Default in-memory deduplicator using a HashSet.
pub struct InMemoryDeduplicator {
    seen: Arc<Mutex<HashSet<Vec<u8>>>>,
}

impl Default for InMemoryDeduplicator {
    fn default() -> Self {
        Self {
            seen: Arc::new(Mutex::new(HashSet::new())),
        }
    }
}

impl ContentDeduplicator for InMemoryDeduplicator {
    fn is_seen(&self, hash: &[u8]) -> bool {
        let seen = self.seen.lock().unwrap();
        seen.contains(hash)
    }

    fn mark_seen(&self, hash: Vec<u8>) {
        let mut seen = self.seen.lock().unwrap();
        seen.insert(hash);
    }
}
