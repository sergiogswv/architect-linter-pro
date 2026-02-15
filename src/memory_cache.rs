use lru::LruCache;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Thread-safe LRU memory cache for file analysis results
#[derive(Clone)]
pub struct MemoryCache {
    cache: Arc<Mutex<LruCache<PathBuf, String>>>,
}

impl MemoryCache {
    /// Create a new memory cache with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(LruCache::new(
                NonZeroUsize::new(capacity).expect("Capacity must be > 0"),
            ))),
        }
    }

    /// Get a cached value
    pub fn get(&self, path: &PathBuf) -> Option<String> {
        let mut cache = self.cache.lock().unwrap();
        cache.get(path).cloned()
    }

    /// Put a value in the cache
    pub fn put(&self, path: PathBuf, content: String) {
        let mut cache = self.cache.lock().unwrap();
        cache.put(path, content);
    }

    /// Clear the cache
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get cache size
    pub fn len(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_and_get() {
        let cache = MemoryCache::new(2);
        let path = PathBuf::from("test.ts");
        cache.put(path.clone(), "content".to_string());
        assert_eq!(cache.get(&path), Some("content".to_string()));
    }
}
