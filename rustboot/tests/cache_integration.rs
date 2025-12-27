//! Integration tests for Rustboot cache module

use rustboot::cache::*;
use std::thread;
use std::time::Duration;

#[test]
fn test_cache_basic_operations() {
    let cache = InMemoryCache::new();
    
    cache.set("key1", "value1").unwrap();
    assert_eq!(cache.get("key1").unwrap().as_deref(), Some(&"value1"));
    
    cache.delete("key1").unwrap();
    assert!(cache.get("key1").unwrap().is_none());
}

#[test]
fn test_cache_ttl_expiration() {
    let cache = InMemoryCache::new();
    
    cache.set_with_ttl("temp_key", "temp_value", Duration::from_millis(100)).unwrap();
    assert!(cache.get("temp_key").unwrap().is_some());
    
    thread::sleep(Duration::from_millis(150));
    assert!(cache.get("temp_key").unwrap().is_none());
}

#[test]
fn test_cache_overwrite() {
    let cache = InMemoryCache::new();
    
    cache.set("key", "value1").unwrap();
    cache.set("key", "value2").unwrap();
    
    assert_eq!(cache.get("key").unwrap().as_deref(), Some(&"value2"));
}

#[test]
fn test_cache_multiple_keys() {
    let cache = InMemoryCache::new();
    
    for i in 0..100 {
        cache.set(&format!("key{}", i), format!("value{}", i)).unwrap();
    }
    
    for i in 0..100 {
        let value = cache.get(&format!("key{}", i)).unwrap();
        assert_eq!(value.as_deref(), Some(&format!("value{}", i)));
    }
}

#[test]
#[should_panic(expected = "not found")]
fn test_cache_missing_key_panic() {
    let cache = InMemoryCache::new();
    cache.get("nonexistent").unwrap().expect("Should panic");
}

#[test]
fn test_cache_clear() {
    let cache = InMemoryCache::new();
    
    cache.set("key1", "value1").unwrap();
    cache.set("key2", "value2").unwrap();
    
    cache.clear().unwrap();
    
    assert!(cache.get("key1").unwrap().is_none());
    assert!(cache.get("key2").unwrap().is_none());
}
