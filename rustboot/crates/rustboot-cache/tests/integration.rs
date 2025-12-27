//! Integration tests for rustboot-cache

use dev_engineeringlabs_rustboot_cache::*;
use std::time::Duration;

#[test]
fn test_cache_basic_operations() {
    let cache: InMemoryCache<String, String> = InMemoryCache::new();
    
    // Set value with TTL
    cache.set_with_ttl("user:123".to_string(), "John Doe".to_string(), Duration::from_secs(60)).unwrap();
    
    // Get value
    let value = cache.get(&"user:123".to_string()).unwrap();
    assert_eq!(value, Some("John Doe".to_string()));
    
    // Get non-existent
    let missing = cache.get(&"user:999".to_string()).unwrap();
    assert_eq!(missing, None);
}

#[test]
fn test_cache_expiration() {
    let cache: InMemoryCache<String, String> = InMemoryCache::new();
    
    // Set with short TTL
    cache.set_with_ttl("temp".to_string(), "value".to_string(), Duration::from_millis(100)).unwrap();
    
    // Should exist immediately
    assert!(cache.get(&"temp".to_string()).unwrap().is_some());
    
    // Wait for expiration
    std::thread::sleep(Duration::from_millis(150));
    
    // Should be expired
    assert!(cache.get(&"temp".to_string()).unwrap().is_none());
}

#[test]
fn test_cache_update() {
    let cache: InMemoryCache<String, String> = InMemoryCache::new();
    
    cache.set_with_ttl("config".to_string(), "v1".to_string(), Duration::from_secs(60)).unwrap();
    assert_eq!(cache.get(&"config".to_string()).unwrap(), Some("v1".to_string()));
    
    // Update value
    cache.set_with_ttl("config".to_string(), "v2".to_string(), Duration::from_secs(60)).unwrap();
    assert_eq!(cache.get(&"config".to_string()).unwrap(), Some("v2".to_string()));
}

#[test]
fn test_cache_remove() {
    let cache: InMemoryCache<String, i32> = InMemoryCache::new();
    
    cache.set_with_ttl("temp".to_string(), 42, Duration::from_secs(60)).unwrap();
    assert!(cache.get(&"temp".to_string()).unwrap().is_some());
    
    cache.remove(&"temp".to_string()).unwrap();
    assert!(cache.get(&"temp".to_string()).unwrap().is_none());
}

#[test]
fn test_cache_clear() {
    let cache: InMemoryCache<String, i32> = InMemoryCache::new();
    
    cache.set_with_ttl("key1".to_string(), 1, Duration::from_secs(60)).unwrap();
    cache.set_with_ttl("key2".to_string(), 2, Duration::from_secs(60)).unwrap();
    
    cache.clear().unwrap();
    
    assert!(cache.get(&"key1".to_string()).unwrap().is_none());
    assert!(cache.get(&"key2".to_string()).unwrap().is_none());
}

#[test]
fn test_cache_set_without_ttl() {
    let cache: InMemoryCache<String, String> = InMemoryCache::new();
    
    // Set without TTL (never expires)
    cache.set("permanent".to_string(), "value".to_string()).unwrap();
    
    assert_eq!(cache.get(&"permanent".to_string()).unwrap(), Some("value".to_string()));
    
    // Verify it still exists after sleep
    std::thread::sleep(Duration::from_millis(50));
    assert_eq!(cache.get(&"permanent".to_string()).unwrap(), Some("value".to_string()));
}

#[test]
fn test_cache_contains() {
    let cache: InMemoryCache<String, i32> = InMemoryCache::new();
    
    cache.set("exists".to_string(), 123).unwrap();
    
    assert!(cache.contains(&"exists".to_string()));
    assert!(!cache.contains(&"missing".to_string()));
}
