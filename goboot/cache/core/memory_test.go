package core

import (
	"context"
	"testing"
	"time"
)

func TestMemoryCache_SetGet(t *testing.T) {
	cache := NewMemoryCache()
	ctx := context.Background()

	// Set value
	err := cache.Set(ctx, "key1", "value1", 0)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	// Get value
	val, found, err := cache.Get(ctx, "key1")
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if !found {
		t.Error("Expected to find key")
	}
	if val != "value1" {
		t.Errorf("Expected 'value1', got %v", val)
	}
}

func TestMemoryCache_GetNotFound(t *testing.T) {
	cache := NewMemoryCache()
	ctx := context.Background()

	val, found, err := cache.Get(ctx, "nonexistent")
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if found {
		t.Error("Should not find nonexistent key")
	}
	if val != nil {
		t.Error("Value should be nil")
	}
}

func TestMemoryCache_TTL(t *testing.T) {
	cache := NewMemoryCache()
	ctx := context.Background()

	// Set with short TTL
	cache.Set(ctx, "expiring", "value", 50*time.Millisecond)

	// Should exist initially
	_, found, _ := cache.Get(ctx, "expiring")
	if !found {
		t.Error("Should find key before expiry")
	}

	// Wait for expiry
	time.Sleep(100 * time.Millisecond)

	// Should be gone
	_, found, _ = cache.Get(ctx, "expiring")
	if found {
		t.Error("Should not find expired key")
	}
}

func TestMemoryCache_Delete(t *testing.T) {
	cache := NewMemoryCache()
	ctx := context.Background()

	cache.Set(ctx, "key", "value", 0)

	exists, _ := cache.Exists(ctx, "key")
	if !exists {
		t.Error("Key should exist")
	}

	cache.Delete(ctx, "key")

	exists, _ = cache.Exists(ctx, "key")
	if exists {
		t.Error("Key should not exist after delete")
	}
}

func TestMemoryCache_Clear(t *testing.T) {
	cache := NewMemoryCache()
	ctx := context.Background()

	cache.Set(ctx, "key1", "value1", 0)
	cache.Set(ctx, "key2", "value2", 0)
	cache.Set(ctx, "key3", "value3", 0)

	cache.Clear(ctx)

	keys, _ := cache.Keys(ctx, "*")
	if len(keys) != 0 {
		t.Errorf("Expected 0 keys after clear, got %d", len(keys))
	}
}

func TestMemoryCache_Keys(t *testing.T) {
	cache := NewMemoryCache()
	ctx := context.Background()

	cache.Set(ctx, "user:1", "a", 0)
	cache.Set(ctx, "user:2", "b", 0)
	cache.Set(ctx, "order:1", "c", 0)

	// All keys
	keys, _ := cache.Keys(ctx, "*")
	if len(keys) != 3 {
		t.Errorf("Expected 3 keys, got %d", len(keys))
	}

	// Prefix pattern
	keys, _ = cache.Keys(ctx, "user:*")
	if len(keys) != 2 {
		t.Errorf("Expected 2 user keys, got %d", len(keys))
	}
}

func TestMemoryCache_Stats(t *testing.T) {
	cache := NewMemoryCache()
	ctx := context.Background()

	cache.Set(ctx, "key", "value", 0)

	// Hit
	cache.Get(ctx, "key")
	cache.Get(ctx, "key")

	// Miss
	cache.Get(ctx, "nonexistent")

	stats := cache.Stats()
	if stats.Hits != 2 {
		t.Errorf("Expected 2 hits, got %d", stats.Hits)
	}
	if stats.Misses != 1 {
		t.Errorf("Expected 1 miss, got %d", stats.Misses)
	}
	if stats.Size != 1 {
		t.Errorf("Expected size 1, got %d", stats.Size)
	}
}

func TestLRUCache_Eviction(t *testing.T) {
	cache := NewLRUCache(3)
	ctx := context.Background()

	cache.Set(ctx, "a", 1, 0)
	cache.Set(ctx, "b", 2, 0)
	cache.Set(ctx, "c", 3, 0)

	// Access 'a' to make it recently used
	cache.Get(ctx, "a")

	// Add 'd' - should evict 'b' (least recently used)
	cache.Set(ctx, "d", 4, 0)

	// 'a', 'c', 'd' should exist
	exists, _ := cache.Exists(ctx, "a")
	if !exists {
		t.Error("'a' should exist")
	}
	exists, _ = cache.Exists(ctx, "c")
	if !exists {
		t.Error("'c' should exist")
	}
	exists, _ = cache.Exists(ctx, "d")
	if !exists {
		t.Error("'d' should exist")
	}

	// 'b' should be evicted
	exists, _ = cache.Exists(ctx, "b")
	if exists {
		t.Error("'b' should be evicted")
	}
}

func TestLRUCache_UpdatesOrder(t *testing.T) {
	cache := NewLRUCache(2)
	ctx := context.Background()

	cache.Set(ctx, "a", 1, 0)
	cache.Set(ctx, "b", 2, 0)

	// Access 'a' to make it most recently used
	cache.Get(ctx, "a")

	// Add 'c' - should evict 'b'
	cache.Set(ctx, "c", 3, 0)

	exists, _ := cache.Exists(ctx, "a")
	if !exists {
		t.Error("'a' should exist")
	}
	exists, _ = cache.Exists(ctx, "b")
	if exists {
		t.Error("'b' should be evicted")
	}
}
