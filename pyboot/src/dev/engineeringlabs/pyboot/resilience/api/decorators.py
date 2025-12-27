"""Resilience decorators for easy application of fault tolerance patterns."""

import asyncio
import functools
import random
import time
from collections.abc import Awaitable, Callable
from typing import Any, TypeVar

from dev.engineeringlabs.pyboot.resilience.api.config import (
    RetryConfig,
    CircuitBreakerConfig,
    TimeoutConfig,
    BulkheadConfig,
    RateLimitConfig,
)

T = TypeVar("T")


def retryable(
    max_attempts: int = 3,
    delay: float = 1.0,
    backoff: float = 2.0,
    max_delay: float = 60.0,
    jitter: bool = True,
    exceptions: tuple[type[Exception], ...] = (),
) -> Callable[[Callable[..., Awaitable[T]]], Callable[..., Awaitable[T]]]:
    """Decorator for automatic retry with exponential backoff.

    Args:
        max_attempts: Maximum retry attempts (including initial call)
        delay: Initial delay between retries in seconds
        backoff: Multiplier for exponential backoff
        max_delay: Maximum delay between retries
        jitter: Add randomness to prevent thundering herd
        exceptions: Exception types to retry on (empty = retry all)

    Example:
        @retryable(max_attempts=3, delay=1.0, backoff=2.0)
        async def fetch_data():
            return await api.get("/data")
    """
    def decorator(func: Callable[..., Awaitable[T]]) -> Callable[..., Awaitable[T]]:
        @functools.wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            last_exception: Exception | None = None
            current_delay = delay

            for attempt in range(max_attempts):
                try:
                    return await func(*args, **kwargs)
                except Exception as e:
                    if exceptions and not isinstance(e, exceptions):
                        raise
                    last_exception = e

                    if attempt < max_attempts - 1:
                        # Calculate delay with optional jitter
                        wait_time = min(current_delay, max_delay)
                        if jitter:
                            wait_time = wait_time * (0.5 + random.random())
                        await asyncio.sleep(wait_time)
                        current_delay *= backoff

            # All retries exhausted
            raise last_exception  # type: ignore[misc]

        return wrapper
    return decorator


def circuit_breaker(
    failure_threshold: int = 5,
    timeout_seconds: float = 30.0,
    success_threshold: int = 2,
    name: str | None = None,
) -> Callable[[Callable[..., Awaitable[T]]], Callable[..., Awaitable[T]]]:
    """Decorator for circuit breaker pattern.

    Args:
        failure_threshold: Failures before opening circuit
        timeout_seconds: Recovery timeout in seconds
        success_threshold: Successes to close circuit
        name: Circuit breaker name for tracking

    Example:
        @circuit_breaker(failure_threshold=5, timeout_seconds=30.0)
        async def call_service():
            return await external_api.get("/data")
    """
    def decorator(func: Callable[..., Awaitable[T]]) -> Callable[..., Awaitable[T]]:
        # Import here to avoid circular dependency
        from dev.engineeringlabs.pyboot.resilience.core.circuit_breaker import CircuitBreaker

        config = CircuitBreakerConfig(
            failure_threshold=failure_threshold,
            timeout_seconds=timeout_seconds,
            success_threshold=success_threshold,
        )
        breaker = CircuitBreaker(config=config, name=name or func.__name__)

        @functools.wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            return await breaker.execute(lambda: func(*args, **kwargs))

        # Expose breaker for inspection
        wrapper.circuit_breaker = breaker  # type: ignore[attr-defined]
        return wrapper

    return decorator


def timeout(
    seconds: float = 30.0,
    cancel_on_timeout: bool = True,
) -> Callable[[Callable[..., Awaitable[T]]], Callable[..., Awaitable[T]]]:
    """Decorator for operation timeout.

    Args:
        seconds: Timeout duration in seconds
        cancel_on_timeout: Whether to cancel the operation

    Example:
        @timeout(seconds=10.0)
        async def slow_operation():
            return await long_running_task()
    """
    def decorator(func: Callable[..., Awaitable[T]]) -> Callable[..., Awaitable[T]]:
        @functools.wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            try:
                return await asyncio.wait_for(
                    func(*args, **kwargs),
                    timeout=seconds,
                )
            except asyncio.TimeoutError:
                from dev.engineeringlabs.pyboot.resilience.core.timeout import TimeoutError
                raise TimeoutError(
                    f"Operation {func.__name__} timed out after {seconds}s",
                    timeout_seconds=seconds,
                )

        return wrapper
    return decorator


def bulkhead(
    max_concurrent: int = 10,
    max_wait: float = 0.0,
) -> Callable[[Callable[..., Awaitable[T]]], Callable[..., Awaitable[T]]]:
    """Decorator for bulkhead (concurrency limiting).

    Args:
        max_concurrent: Maximum concurrent executions
        max_wait: Maximum wait time for a slot (0 = fail immediately)

    Example:
        @bulkhead(max_concurrent=5)
        async def resource_heavy():
            return await consume_resource()
    """
    def decorator(func: Callable[..., Awaitable[T]]) -> Callable[..., Awaitable[T]]:
        semaphore = asyncio.Semaphore(max_concurrent)

        @functools.wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            try:
                acquired = await asyncio.wait_for(
                    semaphore.acquire(),
                    timeout=max_wait if max_wait > 0 else None,
                )
            except asyncio.TimeoutError:
                from dev.engineeringlabs.pyboot.resilience.core.bulkhead import BulkheadFullError
                raise BulkheadFullError(
                    f"Bulkhead full for {func.__name__}, max={max_concurrent}"
                )

            try:
                return await func(*args, **kwargs)
            finally:
                semaphore.release()

        return wrapper
    return decorator


def rate_limited(
    calls: int = 100,
    period: float = 60.0,
) -> Callable[[Callable[..., Awaitable[T]]], Callable[..., Awaitable[T]]]:
    """Decorator for rate limiting.

    Args:
        calls: Number of calls allowed in the period
        period: Time period in seconds

    Example:
        @rate_limited(calls=100, period=60.0)
        async def api_call():
            return await external_api.get("/data")
    """
    def decorator(func: Callable[..., Awaitable[T]]) -> Callable[..., Awaitable[T]]:
        call_times: list[float] = []
        lock = asyncio.Lock()

        @functools.wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            async with lock:
                now = time.time()
                # Remove old calls outside the window
                cutoff = now - period
                while call_times and call_times[0] < cutoff:
                    call_times.pop(0)

                if len(call_times) >= calls:
                    from dev.engineeringlabs.pyboot.resilience.core.rate_limiter import RateLimitExceededError
                    raise RateLimitExceededError(
                        f"Rate limit exceeded for {func.__name__}: "
                        f"{calls} calls per {period}s"
                    )

                call_times.append(now)

            return await func(*args, **kwargs)

        return wrapper
    return decorator


def fallback(
    fallback_value: T | None = None,
    fallback_func: Callable[[Exception], T] | None = None,
    exceptions: tuple[type[Exception], ...] = (),
) -> Callable[[Callable[..., Awaitable[T]]], Callable[..., Awaitable[T]]]:
    """Decorator for fallback on failure.

    Args:
        fallback_value: Static value to return on failure
        fallback_func: Function to call on failure (receives exception)
        exceptions: Exception types to catch (empty = catch all)

    Example:
        @fallback(fallback_value={"status": "unavailable"})
        async def fetch_status():
            return await service.get_status()

        @fallback(fallback_func=lambda e: {"error": str(e)})
        async def fetch_data():
            return await service.get_data()
    """
    def decorator(func: Callable[..., Awaitable[T]]) -> Callable[..., Awaitable[T]]:
        @functools.wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            try:
                return await func(*args, **kwargs)
            except Exception as e:
                if exceptions and not isinstance(e, exceptions):
                    raise
                if fallback_func is not None:
                    return fallback_func(e)
                return fallback_value  # type: ignore[return-value]

        return wrapper
    return decorator


__all__ = [
    "retryable",
    "circuit_breaker",
    "timeout",
    "bulkhead",
    "rate_limited",
    "fallback",
]
