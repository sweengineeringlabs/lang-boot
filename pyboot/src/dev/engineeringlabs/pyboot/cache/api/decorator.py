"""Cache decorator for automatic caching."""

import asyncio
import functools
import hashlib
import json
from collections.abc import Awaitable, Callable
from typing import Any, TypeVar

T = TypeVar("T")


def cached(
    ttl: int | None = None,
    key_prefix: str = "",
    cache_name: str | None = None,
    key_builder: Callable[..., str] | None = None,
) -> Callable[[Callable[..., Awaitable[T]]], Callable[..., Awaitable[T]]]:
    """
    Decorator for caching function results.

    Args:
        ttl: Time-to-live in seconds (None = use cache default)
        key_prefix: Prefix for cache keys
        cache_name: Name of cache to use (None = default)
        key_builder: Custom function to build cache key from args

    Example:
        @cached(ttl=300)
        async def get_user(user_id: str) -> User:
            return await db.get_user(user_id)

        @cached(ttl=60, key_prefix="weather")
        async def get_weather(city: str, country: str) -> Weather:
            return await weather_api.get(city, country)

        # Custom key builder
        @cached(key_builder=lambda item: f"item:{item.id}")
        async def process_item(item: Item) -> Result:
            return await heavy_processing(item)
    """
    def decorator(func: Callable[..., Awaitable[T]]) -> Callable[..., Awaitable[T]]:
        @functools.wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            # Import here to avoid circular dependency
            from dev.engineeringlabs.pyboot.cache.core import get_cache

            cache = get_cache(cache_name) if cache_name else get_cache()

            # Build cache key
            if key_builder:
                cache_key = key_builder(*args, **kwargs)
            else:
                cache_key = _build_cache_key(func.__name__, args, kwargs)

            if key_prefix:
                cache_key = f"{key_prefix}:{cache_key}"

            # Try to get from cache
            cached_value = await cache.get(cache_key)
            if cached_value is not None:
                return cached_value

            # Compute value
            result = await func(*args, **kwargs)

            # Store in cache
            await cache.set(cache_key, result, ttl=ttl)

            return result

        return wrapper

    return decorator


def _build_cache_key(func_name: str, args: tuple[Any, ...], kwargs: dict[str, Any]) -> str:
    """Build a cache key from function name and arguments."""
    # Create a representation of the arguments
    key_parts = [func_name]

    for arg in args:
        key_parts.append(_serialize_arg(arg))

    for k, v in sorted(kwargs.items()):
        key_parts.append(f"{k}={_serialize_arg(v)}")

    key_str = ":".join(key_parts)

    # If key is too long, hash it
    if len(key_str) > 200:
        return f"{func_name}:{hashlib.md5(key_str.encode()).hexdigest()}"

    return key_str


def _serialize_arg(arg: Any) -> str:
    """Serialize an argument to a string for cache key."""
    if isinstance(arg, (str, int, float, bool)):
        return str(arg)
    if isinstance(arg, (list, tuple)):
        return json.dumps(arg)
    if isinstance(arg, dict):
        return json.dumps(arg, sort_keys=True)
    if hasattr(arg, "id"):
        return f"{type(arg).__name__}:{arg.id}"
    if hasattr(arg, "__dict__"):
        return json.dumps(arg.__dict__, sort_keys=True, default=str)
    return str(arg)


def cache_invalidate(
    key: str | None = None,
    key_prefix: str = "",
    cache_name: str | None = None,
) -> Callable[[Callable[..., Awaitable[T]]], Callable[..., Awaitable[T]]]:
    """
    Decorator to invalidate cache after function execution.

    Args:
        key: Specific key to invalidate
        key_prefix: Prefix to match for invalidation
        cache_name: Name of cache to use

    Example:
        @cache_invalidate(key_prefix="user")
        async def update_user(user_id: str, data: dict) -> User:
            return await db.update_user(user_id, data)
    """
    def decorator(func: Callable[..., Awaitable[T]]) -> Callable[..., Awaitable[T]]:
        @functools.wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            from dev.engineeringlabs.pyboot.cache.core import get_cache

            result = await func(*args, **kwargs)

            cache = get_cache(cache_name) if cache_name else get_cache()

            if key:
                await cache.delete(key)
            # Note: key_prefix invalidation would require cache scan functionality
            # which depends on the backend

            return result

        return wrapper

    return decorator


__all__ = ["cached", "cache_invalidate"]
