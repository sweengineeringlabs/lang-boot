"""Cache interface definition."""

from abc import ABC, abstractmethod
from typing import Any, TypeVar

from dev.engineeringlabs.pyboot.cache.api.entry import CacheEntry, CacheStats

T = TypeVar("T")


class Cache(ABC):
    """
    Abstract cache interface.

    Provides async methods for cache operations.

    Example:
        cache = get_cache()

        # Basic operations
        await cache.set("key", {"data": "value"}, ttl=300)
        value = await cache.get("key")
        exists = await cache.exists("key")
        await cache.delete("key")

        # Get with default
        value = await cache.get_or_default("key", default="fallback")

        # Get or compute
        value = await cache.get_or_set("key", lambda: compute_value(), ttl=60)
    """

    @property
    @abstractmethod
    def name(self) -> str:
        """Get the cache name."""
        ...

    @abstractmethod
    async def get(self, key: str) -> Any | None:
        """
        Get a value from the cache.

        Args:
            key: Cache key

        Returns:
            Cached value or None if not found
        """
        ...

    @abstractmethod
    async def get_entry(self, key: str) -> CacheEntry | None:
        """
        Get a cache entry with metadata.

        Args:
            key: Cache key

        Returns:
            CacheEntry or None if not found
        """
        ...

    @abstractmethod
    async def set(
        self,
        key: str,
        value: Any,
        ttl: int | None = None,
    ) -> None:
        """
        Set a value in the cache.

        Args:
            key: Cache key
            value: Value to cache
            ttl: Time-to-live in seconds (None = no expiry)
        """
        ...

    @abstractmethod
    async def delete(self, key: str) -> bool:
        """
        Delete a value from the cache.

        Args:
            key: Cache key

        Returns:
            True if deleted, False if not found
        """
        ...

    @abstractmethod
    async def exists(self, key: str) -> bool:
        """
        Check if a key exists in the cache.

        Args:
            key: Cache key

        Returns:
            True if exists, False otherwise
        """
        ...

    @abstractmethod
    async def clear(self) -> None:
        """Clear all values from the cache."""
        ...

    @abstractmethod
    async def get_stats(self) -> CacheStats:
        """Get cache statistics."""
        ...

    async def get_or_default(self, key: str, default: T) -> T:
        """
        Get a value or return default if not found.

        Args:
            key: Cache key
            default: Default value to return

        Returns:
            Cached value or default
        """
        value = await self.get(key)
        if value is None:
            return default
        return value

    async def get_or_set(
        self,
        key: str,
        factory: Any,  # Callable[[], T] | Callable[[], Awaitable[T]]
        ttl: int | None = None,
    ) -> Any:
        """
        Get a value or compute and cache it.

        Args:
            key: Cache key
            factory: Function to compute value if not cached
            ttl: Time-to-live in seconds

        Returns:
            Cached or computed value
        """
        import asyncio

        value = await self.get(key)
        if value is not None:
            return value

        result = factory()
        if asyncio.iscoroutine(result):
            result = await result

        await self.set(key, result, ttl=ttl)
        return result

    async def get_many(self, keys: list[str]) -> dict[str, Any]:
        """
        Get multiple values from the cache.

        Args:
            keys: List of cache keys

        Returns:
            Dictionary of key -> value (only found keys)
        """
        result = {}
        for key in keys:
            value = await self.get(key)
            if value is not None:
                result[key] = value
        return result

    async def set_many(
        self,
        items: dict[str, Any],
        ttl: int | None = None,
    ) -> None:
        """
        Set multiple values in the cache.

        Args:
            items: Dictionary of key -> value
            ttl: Time-to-live in seconds
        """
        for key, value in items.items():
            await self.set(key, value, ttl=ttl)

    async def delete_many(self, keys: list[str]) -> int:
        """
        Delete multiple values from the cache.

        Args:
            keys: List of cache keys

        Returns:
            Number of keys deleted
        """
        deleted = 0
        for key in keys:
            if await self.delete(key):
                deleted += 1
        return deleted


__all__ = ["Cache"]
