"""
Cache Module - Multi-backend caching infrastructure.

This module provides:
- API layer: Cache interface, CacheEntry, CacheConfig
- Core layer: In-memory cache, TTL management
- SPI layer: CacheProvider interface for custom backends

Supported backends:
- In-memory (default)
- Redis (with pyboot-cache[redis])
- Memcached (with pyboot-cache[memcached])

Example:
    from dev.engineeringlabs.pyboot.cache import Cache, get_cache

    # Using the default in-memory cache
    cache = get_cache()
    await cache.set("key", "value", ttl=300)
    value = await cache.get("key")

    # With decorator
    @cached(ttl=60)
    async def fetch_user(user_id: str) -> User:
        return await db.get_user(user_id)
"""

from dev.engineeringlabs.pyboot.cache.api import (
    Cache,
    CacheConfig,
    CacheEntry,
    CacheStats,
    cached,
)

from dev.engineeringlabs.pyboot.cache.core import (
    InMemoryCache,
    get_cache,
    set_cache,
)

from dev.engineeringlabs.pyboot.cache.spi import (
    CacheProvider,
)

__all__ = [
    # API
    "Cache",
    "CacheConfig",
    "CacheEntry",
    "CacheStats",
    "cached",
    # Core
    "InMemoryCache",
    "get_cache",
    "set_cache",
    # SPI
    "CacheProvider",
]
