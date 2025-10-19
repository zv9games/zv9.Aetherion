// aetherion_cache/src/lib.rs
//! In-memory caching layer for storing and retrieving procedural generation data (Chunks).

use aetherion_math::coordinate_system::ChunkKey;
use aetherion_sync::AtomicResource;
use glam::IVec3;
use std::collections::HashMap;
use tracing::info;

// --- 1. Placeholder Data Structure (for Phase 2) ---

/// A minimal placeholder for the actual Chunk Data (Phase 3).
/// In Phase 2, we only track the creation/retrieval status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkDataPlaceholder {
    pub key: ChunkKey,
    pub version: u32,
}

// --- 2. Cache Structure ---

// Type alias for the thread-safe core map
type CacheMap = HashMap<ChunkKey, ChunkDataPlaceholder>;

/// The thread-safe, in-memory cache for generated Chunk data.
/// Uses AtomicResource to allow safe concurrent read/write access across worker threads.
#[derive(Debug, Clone)]
pub struct Cache {
    storage: AtomicResource<CacheMap>,
}

impl Cache {
    /// Creates a new, empty, thread-safe cache instance.
    pub fn new() -> Self {
        info!("Aetherion Cache initialized: Ready for thread-safe storage.");
        Cache {
            storage: AtomicResource::new(HashMap::new()),
        }
    }

    /// Attempts to retrieve a ChunkDataPlaceholder by its ChunkKey.
    pub fn get(&self, key: ChunkKey) -> Option<ChunkDataPlaceholder> {
        let map = self.storage.read();
        map.get(&key).cloned()
    }

    /// Inserts a new ChunkDataPlaceholder into the cache.
    /// Returns true if the insertion was successful (i.e., the key was new).
    pub fn insert(&self, data: ChunkDataPlaceholder) -> bool {
        let mut map = self.storage.write();
        let key = data.key;
        map.insert(key, data).is_none()
    }

    /// Reports the current number of chunks stored in the cache.
    pub fn len(&self) -> usize {
        self.storage.read().len()
    }

    /// Clears all entries from the cache.
    pub fn clear(&self) {
        self.storage.write().clear();
        info!("Aetherion Cache cleared.");
    }

    /// Checks if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ---------------------------
// IMPL: Unit Tests (Phase 2 Validation)
// ---------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_cache_basic_insert_and_get() {
        let cache = Cache::new();
        let key = ChunkKey(IVec3::new(1, 2, 3));
        let data = ChunkDataPlaceholder { key, version: 1 };

        assert!(cache.is_empty());
        
        // Test insertion
        // Must clone here, as `insert` consumes the `data` value.
        assert!(cache.insert(data.clone()), "Insertion should succeed for a new key.");
        assert_eq!(cache.len(), 1, "Cache length should be 1 after insertion.");
        
        // Test retrieval
        let retrieved = cache.get(key);
        // FIX E0382: Clone `data` here to use it for comparison without moving the original `data` variable.
        assert_eq!(retrieved, Some(data.clone()), "Retrieved data must match inserted data.");

        // Test insertion of same key (The original `data` is now available for the final consuming call)
        assert!(!cache.insert(data), "Insertion should fail (return false) for an existing key.");
        assert_eq!(cache.len(), 1, "Cache length should remain 1.");
    }

    #[test]
    fn test_cache_concurrency_safety() {
        let cache = Cache::new();
        let num_threads = 10;
        let num_inserts_per_thread = 100;

        // Clone the cache for each thread
        let handles: Vec<_> = (0..num_threads)
            .map(|i| {
                let cache_clone = cache.clone();
                thread::spawn(move || {
                    for j in 0..num_inserts_per_thread {
                        let key = ChunkKey(IVec3::new(i as i32, j as i32, 0));
                        let data = ChunkDataPlaceholder { key, version: 1 };
                        cache_clone.insert(data);
                    }
                })
            })
            .collect();

        // Wait for all threads to finish
        for h in handles {
            h.join().unwrap();
        }

        // Verify the final count
        let expected_total = num_threads * num_inserts_per_thread;
        assert_eq!(cache.len(), expected_total, "Total items in cache should match total successful inserts.");

        // Verify concurrent retrieval
        let retrieved_count = cache.storage.read().values().count();
        assert_eq!(retrieved_count, expected_total, "Concurrent retrieval count failed.");
    }

    #[test]
    fn test_cache_clear() {
        let cache = Cache::new();
        let key = ChunkKey(IVec3::new(1, 1, 1));
        cache.insert(ChunkDataPlaceholder { key, version: 1 });
        
        assert!(!cache.is_empty());

        cache.clear();

        assert!(cache.is_empty());
    }
}