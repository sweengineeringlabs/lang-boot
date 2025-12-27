"""Cache registry for global cache access."""

from dev.engineeringlabs.pyboot.cache.api.cache import Cache
from dev.engineeringlabs.pyboot.cache.core.memory import InMemoryCache

# Global cache registry
_caches: dict[str, Cache] = {}
_default_cache: Cache | None = None


def get_cache(name: str | None = None) -> Cache:
    """
    Get a cache instance by name.

    Args:
        name: Cache name (None = default cache)

    Returns:
        Cache instance
    """
    global _default_cache

    if name is None:
        if _default_cache is None:
            _default_cache = InMemoryCache(name="default")
        return _default_cache

    if name not in _caches:
        _caches[name] = InMemoryCache(name=name)

    return _caches[name]


def set_cache(cache: Cache, name: str | None = None) -> None:
    """
    Register a cache instance.

    Args:
        cache: Cache instance
        name: Cache name (None = set as default)
    """
    global _default_cache

    if name is None:
        _default_cache = cache
    else:
        _caches[name] = cache


def clear_caches() -> None:
    """Clear all cache registrations."""
    global _default_cache
    _caches.clear()
    _default_cache = None


__all__ = ["get_cache", "set_cache", "clear_caches"]
