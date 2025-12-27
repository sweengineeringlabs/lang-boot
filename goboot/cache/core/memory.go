// Package core contains the implementation details for the cache module.
package core

import (
	"context"
	"strings"
	"sync"
	"sync/atomic"
	"time"

	"dev.engineeringlabs/goboot/cache/api"
)

// MemoryCache is an in-memory cache implementation.
type MemoryCache struct {
	entries   map[string]*api.CacheEntry
	mu        sync.RWMutex
	hits      int64
	misses    int64
	evictions int64
}

// NewMemoryCache creates a new MemoryCache.
func NewMemoryCache() *MemoryCache {
	cache := &MemoryCache{
		entries: make(map[string]*api.CacheEntry),
	}
	// Start cleanup goroutine
	go cache.cleanup()
	return cache
}

// Get retrieves a value from the cache.
func (c *MemoryCache) Get(ctx context.Context, key string) (any, bool, error) {
	c.mu.RLock()
	entry, exists := c.entries[key]
	c.mu.RUnlock()

	if !exists {
		atomic.AddInt64(&c.misses, 1)
		return nil, false, nil
	}

	if entry.IsExpired() {
		atomic.AddInt64(&c.misses, 1)
		c.Delete(ctx, key)
		return nil, false, nil
	}

	atomic.AddInt64(&c.hits, 1)
	return entry.Value, true, nil
}

// Set stores a value in the cache.
func (c *MemoryCache) Set(ctx context.Context, key string, value any, ttl time.Duration) error {
	c.mu.Lock()
	defer c.mu.Unlock()

	entry := &api.CacheEntry{
		Key:       key,
		Value:     value,
		CreatedAt: time.Now(),
	}

	if ttl > 0 {
		entry.ExpiresAt = time.Now().Add(ttl)
	}

	c.entries[key] = entry
	return nil
}

// Delete removes a value from the cache.
func (c *MemoryCache) Delete(ctx context.Context, key string) error {
	c.mu.Lock()
	defer c.mu.Unlock()
	delete(c.entries, key)
	return nil
}

// Exists checks if a key exists in the cache.
func (c *MemoryCache) Exists(ctx context.Context, key string) (bool, error) {
	c.mu.RLock()
	entry, exists := c.entries[key]
	c.mu.RUnlock()

	if !exists {
		return false, nil
	}

	if entry.IsExpired() {
		return false, nil
	}

	return true, nil
}

// Clear removes all items from the cache.
func (c *MemoryCache) Clear(ctx context.Context) error {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.entries = make(map[string]*api.CacheEntry)
	return nil
}

// Keys returns all keys matching the pattern.
// Supports simple wildcard patterns with *.
func (c *MemoryCache) Keys(ctx context.Context, pattern string) ([]string, error) {
	c.mu.RLock()
	defer c.mu.RUnlock()

	var keys []string
	for key, entry := range c.entries {
		if entry.IsExpired() {
			continue
		}
		if matchPattern(pattern, key) {
			keys = append(keys, key)
		}
	}
	return keys, nil
}

// Stats returns cache statistics.
func (c *MemoryCache) Stats() api.CacheStats {
	c.mu.RLock()
	size := int64(len(c.entries))
	c.mu.RUnlock()

	return api.CacheStats{
		Hits:      atomic.LoadInt64(&c.hits),
		Misses:    atomic.LoadInt64(&c.misses),
		Size:      size,
		Evictions: atomic.LoadInt64(&c.evictions),
	}
}

func (c *MemoryCache) cleanup() {
	ticker := time.NewTicker(time.Minute)
	defer ticker.Stop()

	for range ticker.C {
		c.removeExpired()
	}
}

func (c *MemoryCache) removeExpired() {
	c.mu.Lock()
	defer c.mu.Unlock()

	now := time.Now()
	for key, entry := range c.entries {
		if !entry.ExpiresAt.IsZero() && now.After(entry.ExpiresAt) {
			delete(c.entries, key)
			atomic.AddInt64(&c.evictions, 1)
		}
	}
}

func matchPattern(pattern, s string) bool {
	if pattern == "*" {
		return true
	}
	if strings.HasPrefix(pattern, "*") && strings.HasSuffix(pattern, "*") {
		return strings.Contains(s, pattern[1:len(pattern)-1])
	}
	if strings.HasPrefix(pattern, "*") {
		return strings.HasSuffix(s, pattern[1:])
	}
	if strings.HasSuffix(pattern, "*") {
		return strings.HasPrefix(s, pattern[:len(pattern)-1])
	}
	return s == pattern
}

// LRUCache is an LRU cache implementation with a maximum size.
type LRUCache struct {
	*MemoryCache
	maxSize int
	order   []string // For LRU tracking
}

// NewLRUCache creates a new LRUCache.
func NewLRUCache(maxSize int) *LRUCache {
	return &LRUCache{
		MemoryCache: NewMemoryCache(),
		maxSize:     maxSize,
		order:       make([]string, 0, maxSize),
	}
}

// Set stores a value in the cache, evicting LRU items if necessary.
func (c *LRUCache) Set(ctx context.Context, key string, value any, ttl time.Duration) error {
	c.mu.Lock()
	defer c.mu.Unlock()

	// Remove from order if exists
	for i, k := range c.order {
		if k == key {
			c.order = append(c.order[:i], c.order[i+1:]...)
			break
		}
	}

	// Evict if at capacity
	for len(c.order) >= c.maxSize && c.maxSize > 0 {
		evictKey := c.order[0]
		c.order = c.order[1:]
		delete(c.entries, evictKey)
		atomic.AddInt64(&c.evictions, 1)
	}

	// Add to cache
	entry := &api.CacheEntry{
		Key:       key,
		Value:     value,
		CreatedAt: time.Now(),
	}
	if ttl > 0 {
		entry.ExpiresAt = time.Now().Add(ttl)
	}
	c.entries[key] = entry
	c.order = append(c.order, key)

	return nil
}

// Get retrieves a value and moves it to the end (most recently used).
func (c *LRUCache) Get(ctx context.Context, key string) (any, bool, error) {
	c.mu.Lock()
	entry, exists := c.entries[key]
	if exists && !entry.IsExpired() {
		// Move to end
		for i, k := range c.order {
			if k == key {
				c.order = append(c.order[:i], c.order[i+1:]...)
				c.order = append(c.order, key)
				break
			}
		}
	}
	c.mu.Unlock()

	if !exists || entry.IsExpired() {
		atomic.AddInt64(&c.misses, 1)
		return nil, false, nil
	}

	atomic.AddInt64(&c.hits, 1)
	return entry.Value, true, nil
}
