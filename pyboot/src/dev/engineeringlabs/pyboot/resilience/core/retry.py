"""Retry executor implementation."""

import asyncio
import random
from collections.abc import Awaitable, Callable
from typing import TypeVar

from dev.engineeringlabs.pyboot.resilience.api.config import RetryConfig

T = TypeVar("T")


class RetryExhaustedError(Exception):
    """Raised when all retry attempts are exhausted."""

    def __init__(self, message: str, attempts: int, last_error: Exception) -> None:
        super().__init__(message)
        self.attempts = attempts
        self.last_error = last_error


class RetryExecutor:
    """
    Retry executor with exponential backoff.

    Example:
        executor = RetryExecutor(RetryConfig(max_attempts=3))
        result = await executor.execute(lambda: api.call())
    """

    def __init__(self, config: RetryConfig | None = None) -> None:
        self._config = config or RetryConfig.default()

    @property
    def config(self) -> RetryConfig:
        """Get the configuration."""
        return self._config

    async def execute(self, func: Callable[[], Awaitable[T]]) -> T:
        """
        Execute a function with retry.

        Args:
            func: Async function to execute.

        Returns:
            Function result.

        Raises:
            RetryExhaustedError: If all retries are exhausted.
        """
        last_exception: Exception | None = None
        current_delay = self._config.delay_seconds

        for attempt in range(self._config.max_attempts):
            try:
                return await func()
            except Exception as e:
                if self._config.exceptions and not isinstance(e, self._config.exceptions):
                    raise
                last_exception = e

                if attempt < self._config.max_attempts - 1:
                    # Calculate delay with optional jitter
                    wait_time = min(current_delay, self._config.max_delay_seconds)
                    if self._config.jitter:
                        wait_time = wait_time * (0.5 + random.random())
                    await asyncio.sleep(wait_time)
                    current_delay *= self._config.backoff_multiplier

        raise RetryExhaustedError(
            f"All {self._config.max_attempts} retry attempts exhausted",
            attempts=self._config.max_attempts,
            last_error=last_exception,  # type: ignore[arg-type]
        )


__all__ = [
    "RetryExecutor",
    "RetryExhaustedError",
]
