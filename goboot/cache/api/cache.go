// Package api contains the public interfaces and types for the cache module.
package api

import (
	"context"
	"time"
)

// CacheEntry represents a cached item.
type CacheEntry struct {
	Key       string
	Value     any
	ExpiresAt time.Time
	CreatedAt time.Time
}

// IsExpired returns true if the entry has expired.
func (e *CacheEntry) IsExpired() bool {
	if e.ExpiresAt.IsZero() {
		return false
	}
	return time.Now().After(e.ExpiresAt)
}

// Cache is the interface for cache implementations.
type Cache interface {
	// Get retrieves a value from the cache.
	Get(ctx context.Context, key string) (any, bool, error)

	// Set stores a value in the cache with an optional TTL.
	Set(ctx context.Context, key string, value any, ttl time.Duration) error

	// Delete removes a value from the cache.
	Delete(ctx context.Context, key string) error

	// Exists checks if a key exists in the cache.
	Exists(ctx context.Context, key string) (bool, error)

	// Clear removes all items from the cache.
	Clear(ctx context.Context) error

	// Keys returns all keys matching the pattern.
	Keys(ctx context.Context, pattern string) ([]string, error)
}

// CacheStats holds cache statistics.
type CacheStats struct {
	Hits      int64
	Misses    int64
	Size      int64
	Evictions int64
}

// HitRate returns the cache hit rate.
func (s *CacheStats) HitRate() float64 {
	total := s.Hits + s.Misses
	if total == 0 {
		return 0
	}
	return float64(s.Hits) / float64(total)
}

// StatsCache extends Cache with statistics.
type StatsCache interface {
	Cache
	Stats() CacheStats
}
