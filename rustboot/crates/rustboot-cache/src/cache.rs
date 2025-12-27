//! Caching abstraction (L4: Core - Caching).
//!
//! Trait-based caching with multiple backend support.

use std::hash::Hash;
use std::time::Duration;

/// Cache error types.
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    /// Key not found in cache.
    #[error("Key not found")]
    NotFound,
    
    /// Serialization error.
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    /// Backend error.
    #[error("Backend error: {0}")]
    Backend(String),
}

/// Result type for cache operations.
pub type CacheResult<T> = Result<T, CacheError>;

/// Cache trait for storing key-value pairs.
pub trait Cache<K, V>: Send + Sync
where
    K: Hash + Eq + Send + Sync,
    V: Send + Sync,
{
    /// Get a value from the cache.
    fn get(&self, key: &K) -> CacheResult<Option<V>>;
    
    /// Set a value in the cache.
    fn set(&self, key: K, value: V) -> CacheResult<()>;
    
    /// Set a value with expiration.
    fn set_with_ttl(&self, key: K, value: V, ttl: Duration) -> CacheResult<()>;
    
    /// Remove a value from the cache.
    fn remove(&self, key: &K) -> CacheResult<()>;
    
    /// Check if a key exists.
    fn contains(&self, key: &K) -> bool {
        self.get(key).map(|v| v.is_some()).unwrap_or(false)
    }
    
    /// Clear all entries.
    fn clear(&self) -> CacheResult<()>;
}

/// In-memory cache implementation.
pub struct InMemoryCache<K, V> {
    store: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<K, CacheEntry<V>>>>,
}

struct CacheEntry<V> {
    value: V,
    expires_at: Option<std::time::Instant>,
}

impl<K, V> InMemoryCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new in-memory cache.
    pub fn new() -> Self {
        Self {
            store: std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }
    
    fn is_expired(entry: &CacheEntry<V>) -> bool {
        entry.expires_at.map(|exp| exp < std::time::Instant::now()).unwrap_or(false)
    }
}

impl<K, V> Default for InMemoryCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Cache<K, V> for InMemoryCache<K, V>
where
    K: Hash + Eq + Clone + Send + Sync,
    V: Clone + Send + Sync,
{
    fn get(&self, key: &K) -> CacheResult<Option<V>> {
        let mut store = self.store.lock().unwrap();
        
        if let Some(entry) = store.get(key) {
            if Self::is_expired(entry) {
                store.remove(key);
                Ok(None)
            } else {
                Ok(Some(entry.value.clone()))
            }
        } else {
            Ok(None)
        }
    }
    
    fn set(&self, key: K, value: V) -> CacheResult<()> {
        let mut store = self.store.lock().unwrap();
        store.insert(key, CacheEntry {
            value,
            expires_at: None,
        });
        Ok(())
    }
    
    fn set_with_ttl(&self, key: K, value: V, ttl: Duration) -> CacheResult<()> {
        let mut store = self.store.lock().unwrap();
        store.insert(key, CacheEntry {
            value,
            expires_at: Some(std::time::Instant::now() + ttl),
        });
        Ok(())
    }
    
    fn remove(&self, key: &K) -> CacheResult<()> {
        let mut store = self.store.lock().unwrap();
        store.remove(key);
        Ok(())
    }
    
    fn clear(&self) -> CacheResult<()> {
        let mut store = self.store.lock().unwrap();
        store.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_cache() {
        let cache = InMemoryCache::new();
        
        cache.set("key1".to_string(), "value1".to_string()).unwrap();
        assert_eq!(cache.get(&"key1".to_string()).unwrap(), Some("value1".to_string()));
        
        cache.remove(&"key1".to_string()).unwrap();
        assert_eq!(cache.get(&"key1".to_string()).unwrap(), None);
    }

    #[test]
    fn test_ttl() {
        let cache = InMemoryCache::new();
        
        cache.set_with_ttl(
            "key1".to_string(),
            "value1".to_string(),
            Duration::from_millis(50)
        ).unwrap();
        
        assert!(cache.contains(&"key1".to_string()));
        
        std::thread::sleep(Duration::from_millis(100));
        assert!(!cache.contains(&"key1".to_string()));
    }
}
