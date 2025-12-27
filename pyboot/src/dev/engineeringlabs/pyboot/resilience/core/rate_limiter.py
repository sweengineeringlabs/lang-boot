"""Rate limiter implementation."""

import asyncio
import time
from collections.abc import Awaitable, Callable
from typing import TypeVar

from dev.engineeringlabs.pyboot.resilience.api.config import RateLimitConfig

T = TypeVar("T")


class RateLimitExceededError(Exception):
    """Raised when rate limit is exceeded."""

    def __init__(
        self,
        message: str,
        calls: int,
        period_seconds: float,
    ) -> None:
        super().__init__(message)
        self.calls = calls
        self.period_seconds = period_seconds


class RateLimiter:
    """
    Rate limiter using sliding window algorithm.

    Limits the number of calls within a time window.

    Example:
        limiter = RateLimiter(RateLimitConfig(calls=100, period_seconds=60.0))
        result = await limiter.execute(lambda: api.call())
    """

    def __init__(self, config: RateLimitConfig | None = None) -> None:
        self._config = config or RateLimitConfig.default()
        self._call_times: list[float] = []
        self._lock = asyncio.Lock()

    @property
    def config(self) -> RateLimitConfig:
        """Get the configuration."""
        return self._config

    @property
    def remaining_calls(self) -> int:
        """Get the number of remaining calls in the current window."""
        now = time.time()
        cutoff = now - self._config.period_seconds
        valid_calls = sum(1 for t in self._call_times if t >= cutoff)
        return max(0, self._config.calls - valid_calls)

    async def execute(self, func: Callable[[], Awaitable[T]]) -> T:
        """
        Execute a function with rate limiting.

        Args:
            func: Async function to execute.

        Returns:
            Function result.

        Raises:
            RateLimitExceededError: If rate limit is exceeded.
        """
        async with self._lock:
            now = time.time()
            # Remove old calls outside the window
            cutoff = now - self._config.period_seconds
            self._call_times = [t for t in self._call_times if t >= cutoff]

            if len(self._call_times) >= self._config.calls:
                raise RateLimitExceededError(
                    f"Rate limit exceeded: {self._config.calls} calls "
                    f"per {self._config.period_seconds}s",
                    calls=self._config.calls,
                    period_seconds=self._config.period_seconds,
                )

            self._call_times.append(now)

        return await func()

    async def try_acquire(self) -> bool:
        """Try to acquire a rate limit slot.

        Returns:
            True if acquired, False if rate limit would be exceeded.
        """
        async with self._lock:
            now = time.time()
            cutoff = now - self._config.period_seconds
            self._call_times = [t for t in self._call_times if t >= cutoff]

            if len(self._call_times) >= self._config.calls:
                return False

            self._call_times.append(now)
            return True

    def reset(self) -> None:
        """Reset the rate limiter."""
        self._call_times.clear()


__all__ = [
    "RateLimiter",
    "RateLimitExceededError",
]
