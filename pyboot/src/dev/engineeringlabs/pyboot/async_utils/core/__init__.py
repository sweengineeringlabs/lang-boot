"""Async Core - Task executor and pool implementations."""

from typing import TypeVar, Generic, Callable, Awaitable, Any
import asyncio
from concurrent.futures import ThreadPoolExecutor


T = TypeVar("T")


class TaskExecutor:
    """Executor for running async tasks."""
    
    def __init__(self, max_workers: int = 4) -> None:
        self._max_workers = max_workers
        self._thread_pool = ThreadPoolExecutor(max_workers=max_workers)
    
    async def run(self, coro: Awaitable[T]) -> T:
        """Run a coroutine."""
        return await coro
    
    async def run_blocking(self, func: Callable[..., T], *args: Any, **kwargs: Any) -> T:
        """Run a blocking function in thread pool."""
        loop = asyncio.get_running_loop()
        return await loop.run_in_executor(
            self._thread_pool,
            lambda: func(*args, **kwargs),
        )
    
    def shutdown(self) -> None:
        """Shutdown the executor."""
        self._thread_pool.shutdown(wait=True)


class TaskPool:
    """Pool for managing multiple concurrent tasks."""
    
    def __init__(self, max_concurrent: int = 10) -> None:
        self._max_concurrent = max_concurrent
        self._semaphore = asyncio.Semaphore(max_concurrent)
        self._tasks: list[asyncio.Task[Any]] = []
    
    async def submit(self, coro: Awaitable[T]) -> asyncio.Task[T]:
        """Submit a task to the pool."""
        async def _wrapped() -> T:
            async with self._semaphore:
                return await coro
        
        task = asyncio.create_task(_wrapped())
        self._tasks.append(task)
        return task
    
    async def gather_all(self) -> list[Any]:
        """Wait for all tasks to complete."""
        results = await asyncio.gather(*self._tasks, return_exceptions=True)
        self._tasks.clear()
        return results
    
    async def cancel_all(self) -> None:
        """Cancel all pending tasks."""
        for task in self._tasks:
            task.cancel()
        self._tasks.clear()


__all__ = [
    "TaskExecutor",
    "TaskPool",
]
