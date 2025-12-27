//! Property-based tests for rustboot-cache
//!
//! These tests verify fundamental cache properties:
//! - Set-Get consistency: Value set can be retrieved
//! - Removal: Removed keys are not retrievable
//! - TTL expiration: Values expire after TTL
//! - Clear: Clear removes all entries
//! - Isolation: Different keys don't interfere

use proptest::prelude::*;
use dev_engineeringlabs_rustboot_cache::{Cache, InMemoryCache};
use std::time::Duration;

// Property: Set then Get returns the value
proptest! {
    #[test]
    fn test_set_get_consistency_string(key in "[a-z]{1,20}", value in ".*") {
        let cache = InMemoryCache::new();
        cache.set(key.clone(), value.clone()).unwrap();
        let result = cache.get(&key).unwrap();
        prop_assert_eq!(result, Some(value));
    }

    #[test]
    fn test_set_get_consistency_i32(key in "[a-z]{1,20}", value in any::<i32>()) {
        let cache = InMemoryCache::new();
        cache.set(key.clone(), value).unwrap();
        let result = cache.get(&key).unwrap();
        prop_assert_eq!(result, Some(value));
    }

    #[test]
    fn test_set_get_consistency_vec(key in "[a-z]{1,20}", value in prop::collection::vec(any::<i32>(), 0..100)) {
        let cache = InMemoryCache::new();
        cache.set(key.clone(), value.clone()).unwrap();
        let result = cache.get(&key).unwrap();
        prop_assert_eq!(result, Some(value));
    }
}

// Property: Get on non-existent key returns None
proptest! {
    #[test]
    fn test_get_nonexistent(key in "[a-z]{1,20}") {
        let cache: InMemoryCache<String, String> = InMemoryCache::new();
        let result = cache.get(&key).unwrap();
        prop_assert_eq!(result, None);
    }
}

// Property: Remove then Get returns None
proptest! {
    #[test]
    fn test_remove_then_get(key in "[a-z]{1,20}", value in ".*") {
        let cache = InMemoryCache::new();
        cache.set(key.clone(), value).unwrap();
        cache.remove(&key).unwrap();
        let result = cache.get(&key).unwrap();
        prop_assert_eq!(result, None);
    }
}

// Property: Overwrite - setting same key twice uses latest value
proptest! {
    #[test]
    fn test_overwrite(key in "[a-z]{1,20}", value1 in ".*", value2 in ".*") {
        let cache = InMemoryCache::new();
        cache.set(key.clone(), value1).unwrap();
        cache.set(key.clone(), value2.clone()).unwrap();
        let result = cache.get(&key).unwrap();
        prop_assert_eq!(result, Some(value2));
    }
}

// Property: Multiple keys are independent
proptest! {
    #[test]
    fn test_multiple_keys_independence(
        key1 in "[a-z]{1,10}",
        key2 in "[A-Z]{1,10}",
        value1 in any::<i32>(),
        value2 in any::<i32>()
    ) {
        prop_assume!(key1 != key2.to_lowercase());
        let cache = InMemoryCache::new();

        cache.set(key1.clone(), value1).unwrap();
        cache.set(key2.clone(), value2).unwrap();

        prop_assert_eq!(cache.get(&key1).unwrap(), Some(value1));
        prop_assert_eq!(cache.get(&key2).unwrap(), Some(value2));
    }
}

// Property: Contains returns true for existing keys, false for non-existent
proptest! {
    #[test]
    fn test_contains_existing(key in "[a-z]{1,20}", value in any::<i32>()) {
        let cache = InMemoryCache::new();
        prop_assert!(!cache.contains(&key));
        cache.set(key.clone(), value).unwrap();
        prop_assert!(cache.contains(&key));
    }

    #[test]
    fn test_contains_after_remove(key in "[a-z]{1,20}", value in any::<i32>()) {
        let cache = InMemoryCache::new();
        cache.set(key.clone(), value).unwrap();
        prop_assert!(cache.contains(&key));
        cache.remove(&key).unwrap();
        prop_assert!(!cache.contains(&key));
    }
}

// Property: Clear removes all entries
proptest! {
    #[test]
    fn test_clear_removes_all(
        entries in prop::collection::vec(("[a-z]{1,10}", any::<i32>()), 1..20)
    ) {
        let cache = InMemoryCache::new();
        let (keys, values): (Vec<_>, Vec<_>) = entries.into_iter().unzip();

        // Set all key-value pairs
        for (key, value) in keys.iter().zip(values.iter()) {
            cache.set(key.clone(), *value).unwrap();
        }

        // Verify they exist
        for key in &keys {
            prop_assert!(cache.contains(key));
        }

        // Clear cache
        cache.clear().unwrap();

        // Verify all are gone
        for key in &keys {
            prop_assert!(!cache.contains(key));
            prop_assert_eq!(cache.get(key).unwrap(), None);
        }
    }
}

// Property: TTL expiration - value not available after TTL
proptest! {
    #[test]
    fn test_ttl_expiration(key in "[a-z]{1,20}", value in any::<i32>(), ttl_ms in 10u64..50) {
        let cache = InMemoryCache::new();
        cache.set_with_ttl(key.clone(), value, Duration::from_millis(ttl_ms)).unwrap();

        // Should be available immediately
        prop_assert_eq!(cache.get(&key).unwrap(), Some(value));

        // Wait for expiration
        std::thread::sleep(Duration::from_millis(ttl_ms + 20));

        // Should be gone
        prop_assert_eq!(cache.get(&key).unwrap(), None);
        prop_assert!(!cache.contains(&key));
    }
}

// Property: TTL - value available before expiration
proptest! {
    #[test]
    fn test_ttl_before_expiration(key in "[a-z]{1,20}", value in any::<i32>()) {
        let cache = InMemoryCache::new();
        cache.set_with_ttl(key.clone(), value, Duration::from_millis(1000)).unwrap();

        // Should be available immediately
        prop_assert_eq!(cache.get(&key).unwrap(), Some(value));

        // Should still be available shortly after
        std::thread::sleep(Duration::from_millis(10));
        prop_assert_eq!(cache.get(&key).unwrap(), Some(value));
    }
}

// Property: Set without TTL persists indefinitely
proptest! {
    #[test]
    fn test_no_ttl_persists(key in "[a-z]{1,20}", value in any::<i32>()) {
        let cache = InMemoryCache::new();
        cache.set(key.clone(), value).unwrap();

        // Wait a bit
        std::thread::sleep(Duration::from_millis(50));

        // Should still be available
        prop_assert_eq!(cache.get(&key).unwrap(), Some(value));
    }
}

// Property: Overwriting with TTL replaces expiration time
proptest! {
    #[test]
    fn test_ttl_overwrite_extends(key in "[a-z]{1,20}", value1 in any::<i32>(), value2 in any::<i32>()) {
        let cache = InMemoryCache::new();

        // Set with short TTL
        cache.set_with_ttl(key.clone(), value1, Duration::from_millis(30)).unwrap();

        // Wait a bit
        std::thread::sleep(Duration::from_millis(20));

        // Overwrite with longer TTL
        cache.set_with_ttl(key.clone(), value2, Duration::from_millis(100)).unwrap();

        // Wait past original expiration
        std::thread::sleep(Duration::from_millis(20));

        // Should still be available with new value
        prop_assert_eq!(cache.get(&key).unwrap(), Some(value2));
    }
}

// Property: Overwriting TTL entry with non-TTL entry removes expiration
proptest! {
    #[test]
    fn test_ttl_to_no_ttl(key in "[a-z]{1,20}", value1 in any::<i32>(), value2 in any::<i32>()) {
        let cache = InMemoryCache::new();

        // Set with TTL
        cache.set_with_ttl(key.clone(), value1, Duration::from_millis(50)).unwrap();

        // Overwrite without TTL
        cache.set(key.clone(), value2).unwrap();

        // Wait past original expiration
        std::thread::sleep(Duration::from_millis(60));

        // Should still be available (no expiration)
        prop_assert_eq!(cache.get(&key).unwrap(), Some(value2));
    }
}

// Property: Multiple entries with different TTLs
proptest! {
    #[test]
    fn test_multiple_ttls(
        key1 in "[a-z]{1,10}",
        key2 in "[A-Z]{1,10}",
        value1 in any::<i32>(),
        value2 in any::<i32>()
    ) {
        prop_assume!(key1 != key2.to_lowercase());
        let cache = InMemoryCache::new();

        // Set with different TTLs
        cache.set_with_ttl(key1.clone(), value1, Duration::from_millis(30)).unwrap();
        cache.set_with_ttl(key2.clone(), value2, Duration::from_millis(100)).unwrap();

        // Both available immediately
        prop_assert_eq!(cache.get(&key1).unwrap(), Some(value1));
        prop_assert_eq!(cache.get(&key2).unwrap(), Some(value2));

        // Wait for first to expire
        std::thread::sleep(Duration::from_millis(40));

        // First expired, second still available
        prop_assert_eq!(cache.get(&key1).unwrap(), None);
        prop_assert_eq!(cache.get(&key2).unwrap(), Some(value2));
    }
}

// Property: Remove non-existent key is idempotent
proptest! {
    #[test]
    fn test_remove_nonexistent_idempotent(key in "[a-z]{1,20}") {
        let cache: InMemoryCache<String, i32> = InMemoryCache::new();

        // Remove non-existent key should succeed
        cache.remove(&key).unwrap();
        cache.remove(&key).unwrap();

        // Should still not exist
        prop_assert_eq!(cache.get(&key).unwrap(), None);
    }
}

// Property: Cache operations are thread-safe (basic concurrency test)
proptest! {
    #[test]
    fn test_concurrent_operations(
        entries in prop::collection::vec(("[A-Z]{1,10}", any::<i32>()), 5..10)
    ) {
        // Use HashSet to ensure unique keys for concurrent test
        let unique_entries: std::collections::HashMap<String, i32> = entries.into_iter().collect();
        prop_assume!(unique_entries.len() >= 5); // Ensure we have enough unique keys

        let cache = std::sync::Arc::new(InMemoryCache::new());
        let entries_vec: Vec<_> = unique_entries.into_iter().collect();

        let handles: Vec<_> = entries_vec.iter()
            .map(|(key, value)| {
                let cache_clone = cache.clone();
                let key_clone = key.clone();
                let value_clone = *value;
                std::thread::spawn(move || {
                    cache_clone.set(key_clone, value_clone).unwrap();
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all values are set
        for (key, value) in entries_vec.iter() {
            prop_assert_eq!(cache.get(key).unwrap(), Some(*value));
        }
    }
}

// Property: Clear is idempotent
proptest! {
    #[test]
    fn test_clear_idempotent(
        entries in prop::collection::vec(("[a-z]{1,10}", any::<i32>()), 1..10)
    ) {
        let cache = InMemoryCache::new();
        let (keys, values): (Vec<_>, Vec<_>) = entries.into_iter().unzip();

        for (key, value) in keys.iter().zip(values.iter()) {
            cache.set(key.clone(), *value).unwrap();
        }

        cache.clear().unwrap();
        cache.clear().unwrap();
        cache.clear().unwrap();

        // Should still be empty
        for key in &keys {
            prop_assert_eq!(cache.get(key).unwrap(), None);
        }
    }
}

// Property: Type safety - different value types
proptest! {
    #[test]
    fn test_cache_with_complex_types(key in "[a-z]{1,20}") {
        #[derive(Debug, Clone, PartialEq)]
        struct ComplexValue {
            id: i32,
            data: Vec<String>,
        }

        let cache = InMemoryCache::new();
        let value = ComplexValue {
            id: 42,
            data: vec!["test".to_string(), "data".to_string()],
        };

        cache.set(key.clone(), value.clone()).unwrap();
        let result = cache.get(&key).unwrap();
        prop_assert_eq!(result, Some(value));
    }
}

// Property: Empty cache operations
proptest! {
    #[test]
    fn test_operations_on_empty_cache(key in "[a-z]{1,20}") {
        let cache: InMemoryCache<String, i32> = InMemoryCache::new();

        // Get on empty cache
        prop_assert_eq!(cache.get(&key).unwrap(), None);

        // Contains on empty cache
        prop_assert!(!cache.contains(&key));

        // Remove on empty cache
        cache.remove(&key).unwrap();

        // Clear on empty cache
        cache.clear().unwrap();
    }
}

// Property: Stress test - many operations
proptest! {
    #[test]
    fn test_many_operations(
        entries in prop::collection::vec(("[a-z]{1,10}", any::<i32>()), 50..100)
    ) {
        let cache = InMemoryCache::new();

        // Use HashMap to track final expected values (handles overwrites)
        let mut expected: std::collections::HashMap<String, i32> = std::collections::HashMap::new();

        // Set all
        for (key, value) in &entries {
            cache.set(key.clone(), *value).unwrap();
            expected.insert(key.clone(), *value);
        }

        // Verify all using expected values
        for (key, value) in &expected {
            prop_assert_eq!(cache.get(key).unwrap(), Some(*value));
        }

        // Remove half of unique keys
        let keys_to_remove: Vec<_> = expected.keys().take(expected.len() / 2).cloned().collect();
        for key in &keys_to_remove {
            cache.remove(key).unwrap();
            expected.remove(key);
        }

        // Verify removed keys are gone
        for key in &keys_to_remove {
            prop_assert_eq!(cache.get(key).unwrap(), None);
        }

        // Verify remaining keys still exist
        for (key, value) in &expected {
            prop_assert_eq!(cache.get(key).unwrap(), Some(*value));
        }
    }
}

// Property: Key uniqueness with similar strings
proptest! {
    #[test]
    fn test_key_uniqueness(base in "[a-z]{5,10}", suffix1 in "1", suffix2 in "2") {
        let cache = InMemoryCache::new();
        let key1 = format!("{}{}", base, suffix1);
        let key2 = format!("{}{}", base, suffix2);

        cache.set(key1.clone(), 100).unwrap();
        cache.set(key2.clone(), 200).unwrap();

        prop_assert_eq!(cache.get(&key1).unwrap(), Some(100));
        prop_assert_eq!(cache.get(&key2).unwrap(), Some(200));
    }
}
