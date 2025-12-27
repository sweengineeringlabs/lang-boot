# Cache Module Overview

## WHAT: Caching Abstractions

Multi-backend caching with TTL support, cache-aside pattern, and pluggable storage.

Key capabilities:
- **In-Memory Cache** - Fast local caching with eviction
- **TTL Support** - Time-based expiration
- **Cache-Aside** - Automatic load-on-miss pattern
- **Multi-Backend** - Memory, Redis, custom backends

## WHY: Performance Optimization

**Problems Solved**:
1. **Repeated Computation** - Cache expensive operations
2. **Database Load** - Reduce database queries
3. **Latency** - Faster response times

**When to Use**: Frequently accessed, rarely changing data

## HOW: Usage Guide

```java
var cache = Cache.inMemory(CacheConfig.builder()
    .maxSize(1000)
    .defaultTtl(Duration.ofMinutes(5))
    .build());

cache.set("key", value);
var result = cache.get("key", Value.class);

// Compute if absent
var user = cache.computeIfAbsent("user:123", 
    key -> userRepository.findById(123));
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-config | Cache configuration |
| jboot-observability | Cache metrics |

---

**Status**: Stable
