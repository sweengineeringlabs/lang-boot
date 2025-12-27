"""Circuit breaker implementation for resilience."""

import asyncio
import time
from collections.abc import Awaitable, Callable
from typing import Any, TypeVar

from dev.engineeringlabs.pyboot.resilience.api.config import CircuitBreakerConfig, CircuitState

T = TypeVar("T")


class CircuitOpenError(Exception):
    """Raised when circuit breaker is open."""
    pass


class CircuitBreaker:
    """
    Circuit breaker for fault tolerance.

    Prevents cascading failures by stopping calls to failing services.
    Uses a state machine with three states: CLOSED, OPEN, and HALF_OPEN.

    States:
        - CLOSED: Normal operation, calls pass through
        - OPEN: Circuit is tripped, calls are rejected immediately
        - HALF_OPEN: Testing recovery, limited calls allowed

    Example:
        breaker = CircuitBreaker(
            name="api",
            config=CircuitBreakerConfig(failure_threshold=5)
        )

        # Use as context manager
        async def call_service():
            async with breaker:
                return await external_service()

        # Use as decorator
        @breaker
        async def call_service():
            return await external_service()

        # Use execute method
        result = await breaker.execute(lambda: external_service())
    """

    def __init__(
        self,
        config: CircuitBreakerConfig | None = None,
        name: str = "default",
    ) -> None:
        self._config = config or CircuitBreakerConfig.default()
        self._name = name
        self._state = CircuitState.CLOSED
        self._failure_count = 0
        self._success_count = 0
        self._last_failure_time: float | None = None
        self._half_open_calls = 0
        self._lock = asyncio.Lock()

    @property
    def name(self) -> str:
        """Get the circuit breaker name."""
        return self._name

    @property
    def state(self) -> CircuitState:
        """Get the current state."""
        self._check_timeout()
        return self._state

    @property
    def config(self) -> CircuitBreakerConfig:
        """Get the configuration."""
        return self._config

    def is_closed(self) -> bool:
        """Check if circuit is closed (healthy)."""
        return self.state == CircuitState.CLOSED

    def is_open(self) -> bool:
        """Check if circuit is open (failing)."""
        return self.state == CircuitState.OPEN

    def is_half_open(self) -> bool:
        """Check if circuit is half-open (testing)."""
        return self.state == CircuitState.HALF_OPEN

    async def execute(self, func: Callable[[], Awaitable[T]]) -> T:
        """
        Execute a function through the circuit breaker.

        Args:
            func: Async function to execute.

        Returns:
            Function result.

        Raises:
            CircuitOpenError: If the circuit is open or half-open limit reached.
        """
        async with self._lock:
            self._check_timeout()

            if self._state == CircuitState.OPEN:
                raise CircuitOpenError(
                    f"Circuit '{self._name}' is open. "
                    f"Failures: {self._failure_count}"
                )

            if self._state == CircuitState.HALF_OPEN:
                if self._half_open_calls >= self._config.half_open_max_calls:
                    raise CircuitOpenError(
                        f"Circuit '{self._name}' is half-open, "
                        f"max test calls reached ({self._config.half_open_max_calls})"
                    )
                self._half_open_calls += 1

        # Execute outside the lock to allow concurrency
        try:
            result = await func()
            await self._on_success()
            return result
        except Exception:
            await self._on_failure()
            raise

    async def _on_success(self) -> None:
        """Handle successful call."""
        async with self._lock:
            if self._state == CircuitState.HALF_OPEN:
                self._success_count += 1
                if self._success_count >= self._config.success_threshold:
                    self._reset()
            elif self._state == CircuitState.CLOSED:
                # Reset failure count on success in closed state
                self._failure_count = 0

    async def _on_failure(self) -> None:
        """Handle failed call."""
        async with self._lock:
            self._failure_count += 1
            self._last_failure_time = time.time()

            if self._state == CircuitState.HALF_OPEN:
                # Any failure in half-open immediately trips the circuit
                self._trip()
            elif self._failure_count >= self._config.failure_threshold:
                self._trip()

    def _trip(self) -> None:
        """Trip the circuit to open state."""
        self._state = CircuitState.OPEN
        self._success_count = 0
        self._half_open_calls = 0

    def _reset(self) -> None:
        """Reset the circuit to closed state."""
        self._state = CircuitState.CLOSED
        self._failure_count = 0
        self._success_count = 0
        self._half_open_calls = 0
        self._last_failure_time = None

    def _check_timeout(self) -> None:
        """Check if timeout has elapsed and transition to half-open."""
        if self._state == CircuitState.OPEN and self._last_failure_time:
            elapsed = time.time() - self._last_failure_time
            if elapsed >= self._config.timeout_seconds:
                self._state = CircuitState.HALF_OPEN
                self._success_count = 0
                self._half_open_calls = 0

    async def __aenter__(self) -> "CircuitBreaker":
        """Context manager entry."""
        async with self._lock:
            self._check_timeout()
            if self._state == CircuitState.OPEN:
                raise CircuitOpenError(f"Circuit '{self._name}' is open")

            if self._state == CircuitState.HALF_OPEN:
                if self._half_open_calls >= self._config.half_open_max_calls:
                    raise CircuitOpenError(
                        f"Circuit '{self._name}' is half-open, max test calls reached"
                    )
                self._half_open_calls += 1

        return self

    async def __aexit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: Any,
    ) -> None:
        """Context manager exit."""
        if exc_type is None:
            await self._on_success()
        else:
            await self._on_failure()

    def __call__(
        self,
        func: Callable[..., Awaitable[T]],
    ) -> Callable[..., Awaitable[T]]:
        """Decorator for circuit breaker."""
        from functools import wraps

        @wraps(func)
        async def wrapper(*args: Any, **kwargs: Any) -> T:
            return await self.execute(lambda: func(*args, **kwargs))

        return wrapper


__all__ = [
    "CircuitBreaker",
    "CircuitOpenError",
]
