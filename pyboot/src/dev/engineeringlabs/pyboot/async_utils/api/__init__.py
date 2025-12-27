"""Async API - Task types and errors."""

from typing import TypeVar, Generic, Callable, Awaitable, Any
from dataclasses import dataclass
import asyncio


T = TypeVar("T")


class TaskError(Exception):
    """Base error for task operations."""
    
    def __init__(self, message: str, *, cause: Exception | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.cause = cause


class TaskHandle(Generic[T]):
    """Handle to a running task."""
    
    def __init__(self, task: asyncio.Task[T]) -> None:
        self._task = task
    
    async def result(self) -> T:
        """Wait for and return the task result."""
        return await self._task
    
    def cancel(self) -> bool:
        """Cancel the task."""
        return self._task.cancel()
    
    def done(self) -> bool:
        """Check if task is done."""
        return self._task.done()


async def spawn(coro: Awaitable[T]) -> TaskHandle[T]:
    """Spawn an async task."""
    task = asyncio.create_task(coro)
    return TaskHandle(task)


async def spawn_blocking(func: Callable[..., T], *args: Any, **kwargs: Any) -> T:
    """Run a blocking function in a thread pool."""
    loop = asyncio.get_running_loop()
    return await loop.run_in_executor(None, lambda: func(*args, **kwargs))


async def gather(*coros: Awaitable[T]) -> list[T]:
    """Gather multiple coroutines."""
    return await asyncio.gather(*coros)


__all__ = [
    "TaskError",
    "TaskHandle",
    "spawn",
    "spawn_blocking",
    "gather",
]
