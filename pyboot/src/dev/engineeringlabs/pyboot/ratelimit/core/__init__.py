"""Ratelimit Core - Rate limiter implementations."""

import time
import functools
import asyncio
from typing import Callable, TypeVar, Any, Awaitable
from dev.engineeringlabs.pyboot.ratelimit.api import (
    RateLimitConfig,
    RateLimitStrategy,
    RateLimitError,
    RateLimitExceededError,
)


T = TypeVar("T")


class RateLimiter:
    """Base rate limiter."""
    
    def __init__(self, config: RateLimitConfig) -> None:
        self.config = config
    
    async def acquire(self) -> bool:
        """Acquire a permit. Returns True if allowed."""
        raise NotImplementedError
    
    async def wait(self) -> None:
        """Wait until a permit is available."""
        while not await self.acquire():
            await asyncio.sleep(0.1)


class TokenBucket(RateLimiter):
    """Token bucket rate limiter."""
    
    def __init__(self, config: RateLimitConfig) -> None:
        super().__init__(config)
        self._tokens = float(config.max_requests)
        self._last_update = time.monotonic()
        self._lock = asyncio.Lock()
    
    async def acquire(self) -> bool:
        async with self._lock:
            now = time.monotonic()
            elapsed = now - self._last_update
            self._last_update = now
            
            # Add tokens based on elapsed time
            rate = self.config.max_requests / self.config.window_seconds
            self._tokens = min(
                self.config.max_requests,
                self._tokens + (elapsed * rate),
            )
            
            if self._tokens >= 1:
                self._tokens -= 1
                return True
            return False


class LeakyBucket(RateLimiter):
    """Leaky bucket rate limiter."""
    
    def __init__(self, config: RateLimitConfig) -> None:
        super().__init__(config)
        self._queue: list[float] = []
        self._lock = asyncio.Lock()
    
    async def acquire(self) -> bool:
        async with self._lock:
            now = time.monotonic()
            interval = self.config.window_seconds / self.config.max_requests
            
            # Remove old entries
            self._queue = [t for t in self._queue if now - t < self.config.window_seconds]
            
            if len(self._queue) < self.config.max_requests:
                self._queue.append(now)
                return True
            return False


class FixedWindow(RateLimiter):
    """Fixed window rate limiter."""
    
    def __init__(self, config: RateLimitConfig) -> None:
        super().__init__(config)
        self._count = 0
        self._window_start = time.monotonic()
        self._lock = asyncio.Lock()
    
    async def acquire(self) -> bool:
        async with self._lock:
            now = time.monotonic()
            
            # Reset window if expired
            if now - self._window_start >= self.config.window_seconds:
                self._count = 0
                self._window_start = now
            
            if self._count < self.config.max_requests:
                self._count += 1
                return True
            return False


class SlidingWindow(RateLimiter):
    """Sliding window rate limiter."""
    
    def __init__(self, config: RateLimitConfig) -> None:
        super().__init__(config)
        self._timestamps: list[float] = []
        self._lock = asyncio.Lock()
    
    async def acquire(self) -> bool:
        async with self._lock:
            now = time.monotonic()
            cutoff = now - self.config.window_seconds
            
            # Remove old timestamps
            self._timestamps = [t for t in self._timestamps if t > cutoff]
            
            if len(self._timestamps) < self.config.max_requests:
                self._timestamps.append(now)
                return True
            return False


def rate_limited(
    max_requests: int = 100,
    window_seconds: float = 60.0,
    strategy: RateLimitStrategy = RateLimitStrategy.TOKEN_BUCKET,
) -> Callable[[Callable[..., Awaitable[T]]], Callable[..., Awaitable[T]]]:
    """Decorator to rate limit an async function."""
    config = RateLimitConfig(
        strategy=strategy,
        max_requests=max_requests,
        window_seconds=window_seconds,
    )
    
    limiter: RateLimiter
    if strategy == RateLimitStrategy.TOKEN_BUCKET:
        limiter = TokenBucket(config)
    elif strategy == RateLimitStrategy.LEAKY_BUCKET:
        limiter = LeakyBucket(config)
    elif strategy == RateLimitStrategy.FIXED_WINDOW:
        limiter = FixedWindow(config)
    else:
        limiter = SlidingWindow(config)
    
    def decorator(func: Callable[..., Awaitable[T]]) -> Callable[..., Awaitable[T]]:
        @functools.wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            if not await limiter.acquire():
                raise RateLimitExceededError()
            return await func(*args, **kwargs)
        return wrapper
    return decorator


__all__ = [
    "RateLimiter",
    "TokenBucket",
    "LeakyBucket",
    "FixedWindow",
    "SlidingWindow",
    "rate_limited",
]
