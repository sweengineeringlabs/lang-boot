//go:build ignore

// Package main demonstrates the cache module usage.
package main

import (
	"context"
	"fmt"
	"time"

	"dev.engineeringlabs/goboot/cache"
)

func main() {
	fmt.Println("=== Goboot Cache Module Example ===\n")
	ctx := context.Background()

	// Example 1: Memory cache
	fmt.Println("1. Memory Cache:")
	memCache := cache.NewMemoryCache()

	// Set values
	memCache.Set(ctx, "user:1", map[string]any{"name": "John", "age": 30}, 5*time.Minute)
	memCache.Set(ctx, "user:2", map[string]any{"name": "Jane", "age": 25}, 5*time.Minute)

	// Get value
	if value, found, _ := memCache.Get(ctx, "user:1"); found {
		fmt.Printf("   user:1 = %v\n", value)
	}

	// Check existence
	exists, _ := memCache.Exists(ctx, "user:1")
	fmt.Printf("   user:1 exists: %v\n", exists)

	// Get keys
	keys, _ := memCache.Keys(ctx, "user:*")
	fmt.Printf("   Keys matching 'user:*': %v\n", keys)

	// Stats
	stats := memCache.Stats()
	fmt.Printf("   Stats: hits=%d, misses=%d, size=%d\n", stats.Hits, stats.Misses, stats.Size)

	// Example 2: TTL expiration
	fmt.Println("\n2. TTL Expiration:")
	memCache.Set(ctx, "temp", "expires soon", 50*time.Millisecond)

	if _, found, _ := memCache.Get(ctx, "temp"); found {
		fmt.Println("   Before expiry: found")
	}

	time.Sleep(100 * time.Millisecond)

	if _, found, _ := memCache.Get(ctx, "temp"); !found {
		fmt.Println("   After expiry: not found")
	}

	// Example 3: LRU Cache
	fmt.Println("\n3. LRU Cache:")
	lruCache := cache.NewLRUCache(3) // Max 3 items

	lruCache.Set(ctx, "a", 1, 0)
	lruCache.Set(ctx, "b", 2, 0)
	lruCache.Set(ctx, "c", 3, 0)
	fmt.Println("   Added a, b, c")

	// Access 'a' to make it recently used
	lruCache.Get(ctx, "a")

	// Add 'd' - should evict 'b' (least recently used)
	lruCache.Set(ctx, "d", 4, 0)
	fmt.Println("   Added d (should evict b)")

	keys, _ = lruCache.Keys(ctx, "*")
	fmt.Printf("   Current keys: %v\n", keys)

	if _, found, _ := lruCache.Get(ctx, "b"); !found {
		fmt.Println("   'b' was evicted (as expected)")
	}

	// Example 4: Delete and Clear
	fmt.Println("\n4. Delete and Clear:")
	memCache2 := cache.NewMemoryCache()
	memCache2.Set(ctx, "key1", "value1", 0)
	memCache2.Set(ctx, "key2", "value2", 0)
	memCache2.Set(ctx, "key3", "value3", 0)

	stats = memCache2.Stats()
	fmt.Printf("   Before delete: size=%d\n", stats.Size)

	memCache2.Delete(ctx, "key1")
	stats = memCache2.Stats()
	fmt.Printf("   After delete key1: size=%d\n", stats.Size)

	memCache2.Clear(ctx)
	stats = memCache2.Stats()
	fmt.Printf("   After clear: size=%d\n", stats.Size)
}
