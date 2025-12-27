"""Timeout executor implementation."""

import asyncio
from collections.abc import Awaitable, Callable
from typing import TypeVar

from dev.engineeringlabs.pyboot.resilience.api.config import TimeoutConfig

T = TypeVar("T")


class TimeoutError(Exception):
    """Raised when an operation times out."""

    def __init__(
        self,
        message: str,
        timeout_seconds: float | None = None,
        operation: str | None = None,
    ) -> None:
        super().__init__(message)
        self.timeout_seconds = timeout_seconds
        self.operation = operation


class TimeoutExecutor:
    """
    Timeout executor for operation timeouts.

    Example:
        executor = TimeoutExecutor(TimeoutConfig(seconds=10.0))
        result = await executor.execute(lambda: long_operation())
    """

    def __init__(self, config: TimeoutConfig | None = None) -> None:
        self._config = config or TimeoutConfig.default()

    @property
    def config(self) -> TimeoutConfig:
        """Get the configuration."""
        return self._config

    async def execute(
        self,
        func: Callable[[], Awaitable[T]],
        operation_name: str = "operation",
    ) -> T:
        """
        Execute a function with timeout.

        Args:
            func: Async function to execute.
            operation_name: Name of the operation for error messages.

        Returns:
            Function result.

        Raises:
            TimeoutError: If the operation times out.
        """
        try:
            return await asyncio.wait_for(
                func(),
                timeout=self._config.seconds,
            )
        except asyncio.TimeoutError:
            raise TimeoutError(
                f"Operation '{operation_name}' timed out after {self._config.seconds}s",
                timeout_seconds=self._config.seconds,
                operation=operation_name,
            )


__all__ = [
    "TimeoutExecutor",
    "TimeoutError",
]
