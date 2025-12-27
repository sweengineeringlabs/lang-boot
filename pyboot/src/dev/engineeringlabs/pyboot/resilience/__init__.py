"""
Resilience Module - Fault tolerance patterns.

This module provides comprehensive resilience patterns for building
robust, fault-tolerant applications:

API Layer:
    - Decorators: @retryable, @circuit_breaker, @timeout, @bulkhead,
                  @rate_limited, @fallback
    - Configuration models for all resilience patterns
    - State models for runtime monitoring

Core Layer:
    - RetryExecutor: Exponential backoff with jitter
    - CircuitBreaker: Three-state circuit breaker pattern
    - TimeoutExecutor: Operation timeouts
    - BulkheadExecutor: Concurrency limiting
    - RateLimiter: Token bucket rate limiting

Examples:
    # Using decorators
    from dev.engineeringlabs.pyboot.resilience import retryable, circuit_breaker, timeout

    @retryable(max_attempts=3, delay=1.0, backoff=2.0)
    @circuit_breaker(failure_threshold=5, timeout_seconds=30.0)
    @timeout(seconds=10.0)
    async def fetch_data():
        return await api.get("/data")

    # Using executors directly
    from dev.engineeringlabs.pyboot.resilience import (
        RetryExecutor,
        CircuitBreaker,
        RetryConfig,
        CircuitBreakerConfig,
    )

    retry_exec = RetryExecutor(RetryConfig(max_attempts=3))
    breaker = CircuitBreaker(
        name="api",
        config=CircuitBreakerConfig(failure_threshold=5)
    )

    async def call_api():
        async with breaker:
            result = await retry_exec.execute(lambda: api.call())
            return result

    # Combining multiple patterns
    from dev.engineeringlabs.pyboot.resilience import bulkhead, rate_limited

    @rate_limited(calls=100, period=60.0)
    @bulkhead(max_concurrent=10)
    @retryable(max_attempts=3)
    async def process_request():
        return await heavy_operation()
"""

from dev.engineeringlabs.pyboot.resilience.api import (
    # Decorators
    retryable,
    circuit_breaker,
    timeout,
    bulkhead,
    rate_limited,
    fallback,
    # Config models
    RetryConfig,
    CircuitBreakerConfig,
    TimeoutConfig,
    BulkheadConfig,
    RateLimitConfig,
    FallbackConfig,
    # State models
    CircuitState,
)

from dev.engineeringlabs.pyboot.resilience.core import (
    # Executors
    RetryExecutor,
    CircuitBreaker,
    TimeoutExecutor,
    BulkheadExecutor,
    RateLimiter,
    # Errors
    RetryExhaustedError,
    CircuitOpenError,
    TimeoutError,
    BulkheadFullError,
    RateLimitExceededError,
)

__all__ = [
    # Decorators
    "retryable",
    "circuit_breaker",
    "timeout",
    "bulkhead",
    "rate_limited",
    "fallback",
    # Config models
    "RetryConfig",
    "CircuitBreakerConfig",
    "TimeoutConfig",
    "BulkheadConfig",
    "RateLimitConfig",
    "FallbackConfig",
    "CircuitState",
    # Executors
    "RetryExecutor",
    "CircuitBreaker",
    "TimeoutExecutor",
    "BulkheadExecutor",
    "RateLimiter",
    # Errors
    "RetryExhaustedError",
    "CircuitOpenError",
    "TimeoutError",
    "BulkheadFullError",
    "RateLimitExceededError",
]
