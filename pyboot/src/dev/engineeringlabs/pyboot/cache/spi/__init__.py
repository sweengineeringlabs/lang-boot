"""Cache SPI layer - Service Provider Interface for cache backends."""

from abc import ABC, abstractmethod
from typing import Any

from dev.engineeringlabs.pyboot.cache.api.cache import Cache


class CacheProvider(ABC):
    """
    Abstract interface for cache backend providers.

    Implement this to create custom cache backends like:
    - Redis
    - Memcached
    - DynamoDB
    - Custom distributed caches

    Example:
        class RedisCacheProvider(CacheProvider):
            def __init__(self, redis_client):
                self._client = redis_client

            @property
            def name(self) -> str:
                return "redis"

            def create_cache(self, cache_name: str, config: CacheConfig) -> Cache:
                return RedisCache(self._client, cache_name, config)
    """

    @property
    @abstractmethod
    def name(self) -> str:
        """Get the provider name."""
        ...

    @abstractmethod
    def create_cache(self, cache_name: str, config: Any = None) -> Cache:
        """
        Create a cache instance.

        Args:
            cache_name: Name for the cache
            config: Optional configuration

        Returns:
            Cache instance
        """
        ...

    def is_available(self) -> bool:
        """Check if this provider is available/configured."""
        return True


__all__ = ["CacheProvider"]
