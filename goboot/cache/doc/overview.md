# Cache Module Overview

## WHAT: Caching Abstractions

Multi-backend caching with TTL support and cache-aside pattern.

Key capabilities:
- **In-Memory** - Fast local caching with LRU eviction
- **TTL Support** - Time-based expiration
- **Cache-Aside** - Automatic load-on-miss
- **Interfaces** - Pluggable backends

## WHY: Performance Optimization

**Problems Solved**: Repeated computation, database load, latency

**When to Use**: Frequently accessed, rarely changing data

## HOW: Usage Guide

```go
cache := cache.NewInMemory(cache.Config{
    MaxSize: 1000,
    TTL:     5 * time.Minute,
})

cache.Set("key", value, 0)
value, found := cache.Get("key")

// Compute if absent
user := cache.GetOrSet("user:123", func() interface{} {
    return userRepo.FindById(123)
})
```

---

**Status**: Stable
