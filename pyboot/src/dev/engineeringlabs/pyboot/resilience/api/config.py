"""Configuration models for resilience patterns."""

from dataclasses import dataclass, field
from enum import Enum
from typing import Any


class CircuitState(str, Enum):
    """Circuit breaker states."""
    CLOSED = "closed"      # Normal operation
    OPEN = "open"          # Failing, rejecting calls
    HALF_OPEN = "half_open"  # Testing recovery


@dataclass(frozen=True, slots=True)
class RetryConfig:
    """Configuration for retry behavior.

    Attributes:
        max_attempts: Maximum number of retry attempts (including initial call)
        delay_seconds: Initial delay between retries in seconds
        backoff_multiplier: Multiplier for exponential backoff
        max_delay_seconds: Maximum delay between retries
        jitter: Add randomness to delays to avoid thundering herd
        exceptions: Exception types to retry on (empty = retry all)
    """
    max_attempts: int = 3
    delay_seconds: float = 1.0
    backoff_multiplier: float = 2.0
    max_delay_seconds: float = 60.0
    jitter: bool = True
    exceptions: tuple[type[Exception], ...] = ()

    @classmethod
    def default(cls) -> "RetryConfig":
        """Get default configuration."""
        return cls()


@dataclass(frozen=True, slots=True)
class CircuitBreakerConfig:
    """Configuration for circuit breaker.

    Attributes:
        failure_threshold: Number of failures before opening circuit
        timeout_seconds: Time to wait before testing recovery
        success_threshold: Successes needed in half-open to close
        half_open_max_calls: Max calls allowed in half-open state
        exceptions: Exception types that count as failures
    """
    failure_threshold: int = 5
    timeout_seconds: float = 30.0
    success_threshold: int = 2
    half_open_max_calls: int = 3
    exceptions: tuple[type[Exception], ...] = ()

    @classmethod
    def default(cls) -> "CircuitBreakerConfig":
        """Get default configuration."""
        return cls()


@dataclass(frozen=True, slots=True)
class TimeoutConfig:
    """Configuration for timeout behavior.

    Attributes:
        seconds: Timeout duration in seconds
        cancel_on_timeout: Whether to cancel the operation on timeout
    """
    seconds: float = 30.0
    cancel_on_timeout: bool = True

    @classmethod
    def default(cls) -> "TimeoutConfig":
        """Get default configuration."""
        return cls()


@dataclass(frozen=True, slots=True)
class BulkheadConfig:
    """Configuration for bulkhead (concurrency limiting).

    Attributes:
        max_concurrent: Maximum concurrent executions
        max_wait_seconds: Maximum time to wait for a slot
    """
    max_concurrent: int = 10
    max_wait_seconds: float = 0.0  # 0 = no wait, fail immediately

    @classmethod
    def default(cls) -> "BulkheadConfig":
        """Get default configuration."""
        return cls()


@dataclass(frozen=True, slots=True)
class RateLimitConfig:
    """Configuration for rate limiting.

    Attributes:
        calls: Number of calls allowed in the period
        period_seconds: Time period in seconds
        strategy: Rate limiting strategy ("sliding_window", "fixed_window", "token_bucket")
    """
    calls: int = 100
    period_seconds: float = 60.0
    strategy: str = "sliding_window"

    @classmethod
    def default(cls) -> "RateLimitConfig":
        """Get default configuration."""
        return cls()


@dataclass(frozen=True, slots=True)
class FallbackConfig:
    """Configuration for fallback behavior.

    Attributes:
        fallback_value: Static value to return on failure
        fallback_function: Function to call on failure (takes exception as arg)
        exceptions: Exception types to catch for fallback
    """
    fallback_value: Any = None
    fallback_function: Any = None  # Callable[[Exception], T]
    exceptions: tuple[type[Exception], ...] = ()

    @classmethod
    def default(cls) -> "FallbackConfig":
        """Get default configuration."""
        return cls()


__all__ = [
    "CircuitState",
    "RetryConfig",
    "CircuitBreakerConfig",
    "TimeoutConfig",
    "BulkheadConfig",
    "RateLimitConfig",
    "FallbackConfig",
]
