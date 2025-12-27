"""In-memory cache implementation."""

import asyncio
import time
from collections import OrderedDict
from typing import Any

from dev.engineeringlabs.pyboot.cache.api.cache import Cache
from dev.engineeringlabs.pyboot.cache.api.config import CacheConfig
from dev.engineeringlabs.pyboot.cache.api.entry import CacheEntry, CacheStats


class InMemoryCache(Cache):
    """
    In-memory cache with LRU eviction.

    Thread-safe async implementation suitable for single-process applications.

    Example:
        cache = InMemoryCache(
            name="app-cache",
            config=CacheConfig(max_size=1000, default_ttl=300)
        )

        await cache.set("key", "value")
        value = await cache.get("key")
    """

    def __init__(
        self,
        name: str = "default",
        config: CacheConfig | None = None,
    ) -> None:
        self._name = name
        self._config = config or CacheConfig.default()
        self._entries: OrderedDict[str, CacheEntry] = OrderedDict()
        self._lock = asyncio.Lock()
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    @property
    def name(self) -> str:
        """Get the cache name."""
        return self._name

    @property
    def config(self) -> CacheConfig:
        """Get the cache configuration."""
        return self._config

    async def get(self, key: str) -> Any | None:
        """Get a value from the cache."""
        async with self._lock:
            full_key = self._make_key(key)
            entry = self._entries.get(full_key)

            if entry is None:
                self._misses += 1
                return None

            if entry.is_expired:
                del self._entries[full_key]
                self._misses += 1
                return None

            # Move to end for LRU
            self._entries.move_to_end(full_key)
            entry.touch()
            self._hits += 1
            return entry.value

    async def get_entry(self, key: str) -> CacheEntry | None:
        """Get a cache entry with metadata."""
        async with self._lock:
            full_key = self._make_key(key)
            entry = self._entries.get(full_key)

            if entry is None or entry.is_expired:
                return None

            return entry

    async def set(
        self,
        key: str,
        value: Any,
        ttl: int | None = None,
    ) -> None:
        """Set a value in the cache."""
        async with self._lock:
            full_key = self._make_key(key)

            # Calculate expiry
            effective_ttl = ttl if ttl is not None else self._config.default_ttl
            expires_at = None
            if effective_ttl is not None:
                expires_at = time.time() + effective_ttl

            # Create entry
            entry = CacheEntry(
                key=full_key,
                value=value,
                expires_at=expires_at,
            )

            # Evict if needed before adding
            if (
                self._config.max_size is not None
                and full_key not in self._entries
                and len(self._entries) >= self._config.max_size
            ):
                self._evict_one()

            # Add or update entry
            self._entries[full_key] = entry
            self._entries.move_to_end(full_key)

    async def delete(self, key: str) -> bool:
        """Delete a value from the cache."""
        async with self._lock:
            full_key = self._make_key(key)
            if full_key in self._entries:
                del self._entries[full_key]
                return True
            return False

    async def exists(self, key: str) -> bool:
        """Check if a key exists in the cache."""
        async with self._lock:
            full_key = self._make_key(key)
            entry = self._entries.get(full_key)
            if entry is None:
                return False
            if entry.is_expired:
                del self._entries[full_key]
                return False
            return True

    async def clear(self) -> None:
        """Clear all values from the cache."""
        async with self._lock:
            self._entries.clear()
            self._hits = 0
            self._misses = 0
            self._evictions = 0

    async def get_stats(self) -> CacheStats:
        """Get cache statistics."""
        async with self._lock:
            return CacheStats(
                hits=self._hits,
                misses=self._misses,
                size=len(self._entries),
                max_size=self._config.max_size,
                evictions=self._evictions,
            )

    async def cleanup_expired(self) -> int:
        """Remove all expired entries."""
        async with self._lock:
            expired_keys = [
                key for key, entry in self._entries.items()
                if entry.is_expired
            ]
            for key in expired_keys:
                del self._entries[key]
            return len(expired_keys)

    def _make_key(self, key: str) -> str:
        """Create full key with namespace."""
        if self._config.namespace:
            return f"{self._config.namespace}:{key}"
        return key

    def _evict_one(self) -> None:
        """Evict one entry based on policy."""
        if not self._entries:
            return

        if self._config.eviction_policy == "lru":
            # Remove least recently used (first item in OrderedDict)
            self._entries.popitem(last=False)
        elif self._config.eviction_policy == "fifo":
            # Same as LRU for OrderedDict
            self._entries.popitem(last=False)
        else:
            # Default to LRU
            self._entries.popitem(last=False)

        self._evictions += 1


__all__ = ["InMemoryCache"]
