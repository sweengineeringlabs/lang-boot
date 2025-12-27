"""Bulkhead executor implementation."""

import asyncio
from collections.abc import Awaitable, Callable
from typing import TypeVar

from dev.engineeringlabs.pyboot.resilience.api.config import BulkheadConfig

T = TypeVar("T")


class BulkheadFullError(Exception):
    """Raised when bulkhead is full."""

    def __init__(self, message: str, max_concurrent: int) -> None:
        super().__init__(message)
        self.max_concurrent = max_concurrent


class BulkheadExecutor:
    """
    Bulkhead executor for concurrency limiting.

    Limits the number of concurrent executions to prevent
    resource exhaustion.

    Example:
        executor = BulkheadExecutor(BulkheadConfig(max_concurrent=5))
        result = await executor.execute(lambda: heavy_operation())
    """

    def __init__(self, config: BulkheadConfig | None = None) -> None:
        self._config = config or BulkheadConfig.default()
        self._semaphore = asyncio.Semaphore(self._config.max_concurrent)
        self._current_count = 0

    @property
    def config(self) -> BulkheadConfig:
        """Get the configuration."""
        return self._config

    @property
    def current_count(self) -> int:
        """Get the current number of concurrent executions."""
        return self._current_count

    @property
    def available_permits(self) -> int:
        """Get the number of available permits."""
        return self._config.max_concurrent - self._current_count

    async def execute(self, func: Callable[[], Awaitable[T]]) -> T:
        """
        Execute a function with concurrency limiting.

        Args:
            func: Async function to execute.

        Returns:
            Function result.

        Raises:
            BulkheadFullError: If no slots available and max_wait exceeded.
        """
        try:
            if self._config.max_wait_seconds > 0:
                acquired = await asyncio.wait_for(
                    self._semaphore.acquire(),
                    timeout=self._config.max_wait_seconds,
                )
            else:
                # Try to acquire immediately
                acquired = self._semaphore.locked()
                if acquired:
                    raise BulkheadFullError(
                        f"Bulkhead full, max={self._config.max_concurrent}",
                        max_concurrent=self._config.max_concurrent,
                    )
                await self._semaphore.acquire()
        except asyncio.TimeoutError:
            raise BulkheadFullError(
                f"Bulkhead full after waiting {self._config.max_wait_seconds}s, "
                f"max={self._config.max_concurrent}",
                max_concurrent=self._config.max_concurrent,
            )

        self._current_count += 1
        try:
            return await func()
        finally:
            self._current_count -= 1
            self._semaphore.release()


__all__ = [
    "BulkheadExecutor",
    "BulkheadFullError",
]
