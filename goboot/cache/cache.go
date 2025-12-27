// Package cache provides caching utilities for the goboot framework.
//
// This module provides:
//   - API layer: Cache interface, CacheEntry, CacheStats
//   - Core layer: MemoryCache, LRUCache implementations
//   - SPI layer: CacheBackend, Serializer interfaces for custom backends
//
// Example:
//
//	import "dev.engineeringlabs/goboot/cache"
//
//	c := cache.NewMemoryCache()
//
//	// Set a value with TTL
//	c.Set(ctx, "user:123", userData, 5*time.Minute)
//
//	// Get a value
//	if value, found, _ := c.Get(ctx, "user:123"); found {
//	    user := value.(User)
//	}
//
//	// Use LRU cache with max size
//	lru := cache.NewLRUCache(1000)
package cache

import (
	"dev.engineeringlabs/goboot/cache/api"
	"dev.engineeringlabs/goboot/cache/core"
	"dev.engineeringlabs/goboot/cache/spi"
)

// Re-export API types
type (
	// CacheEntry represents a cached item.
	CacheEntry = api.CacheEntry
	// Cache is the interface for cache implementations.
	Cache = api.Cache
	// CacheStats holds cache statistics.
	CacheStats = api.CacheStats
	// StatsCache extends Cache with statistics.
	StatsCache = api.StatsCache
)

// Re-export Core types
type (
	// MemoryCache is an in-memory cache implementation.
	MemoryCache = core.MemoryCache
	// LRUCache is an LRU cache implementation.
	LRUCache = core.LRUCache
)

// Re-export Core functions
var (
	NewMemoryCache = core.NewMemoryCache
	NewLRUCache    = core.NewLRUCache
)

// Re-export SPI types
type (
	// CacheBackend is the interface for cache backend implementations.
	CacheBackend = spi.CacheBackend
	// Serializer is the interface for cache value serialization.
	Serializer = spi.Serializer
)
