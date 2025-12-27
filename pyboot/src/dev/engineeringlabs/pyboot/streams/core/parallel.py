"""Parallel stream processing."""

import asyncio
from typing import TypeVar, Callable, Awaitable, Iterable, Any
from dataclasses import dataclass

T = TypeVar("T")
R = TypeVar("R")


async def parallel(
    items: Iterable[T],
    processor: Callable[[T], Awaitable[R]],
    concurrency: int = 10,
    ordered: bool = True,
) -> list[R]:
    """Process items in parallel with controlled concurrency.
    
    Example:
        async def fetch_user(id: int) -> User:
            return await api.get_user(id)
        
        users = await parallel(
            items=[1, 2, 3, 4, 5],
            processor=fetch_user,
            concurrency=3,
        )
    
    Args:
        items: Items to process.
        processor: Async function to apply.
        concurrency: Max concurrent tasks.
        ordered: Preserve input order in results.
        
    Returns:
        List of results.
    """
    semaphore = asyncio.Semaphore(concurrency)
    
    async def bounded_process(index: int, item: T) -> tuple[int, R]:
        async with semaphore:
            result = await processor(item)
            return (index, result)
    
    tasks = [
        asyncio.create_task(bounded_process(i, item))
        for i, item in enumerate(items)
    ]
    
    results = await asyncio.gather(*tasks)
    
    if ordered:
        results = sorted(results, key=lambda x: x[0])
    
    return [r[1] for r in results]


async def parallel_map(
    items: Iterable[T],
    fn: Callable[[T], Awaitable[R]],
    concurrency: int = 10,
) -> list[R]:
    """Map function over items in parallel."""
    return await parallel(items, fn, concurrency, ordered=True)


async def parallel_filter(
    items: Iterable[T],
    predicate: Callable[[T], Awaitable[bool]],
    concurrency: int = 10,
) -> list[T]:
    """Filter items in parallel."""
    async def check(item: T) -> tuple[T, bool]:
        result = await predicate(item)
        return (item, result)
    
    results = await parallel(list(items), check, concurrency)
    return [item for item, passed in results if passed]


@dataclass
class TaskResult(Awaitable[Any]):
    """Result wrapper with metadata."""
    value: Any
    index: int
    error: Exception | None = None
    
    def __await__(self):
        async def _get():
            return self.value
        return _get().__await__()


class TaskPool:
    """Pool for managing parallel task execution.
    
    Example:
        pool = TaskPool(concurrency=5)
        
        async with pool:
            for item in items:
                await pool.submit(process_item, item)
            
            results = await pool.results()
    """
    
    def __init__(self, concurrency: int = 10, timeout: float | None = None) -> None:
        self._concurrency = concurrency
        self._timeout = timeout
        self._semaphore = asyncio.Semaphore(concurrency)
        self._tasks: list[asyncio.Task] = []
        self._results: list[Any] = []
    
    async def __aenter__(self) -> "TaskPool":
        return self
    
    async def __aexit__(self, *args: Any) -> None:
        await self.wait()
    
    async def submit(self, fn: Callable[..., Awaitable[R]], *args: Any, **kwargs: Any) -> None:
        """Submit task to pool."""
        async def run() -> R:
            async with self._semaphore:
                return await fn(*args, **kwargs)
        
        task = asyncio.create_task(run())
        self._tasks.append(task)
    
    async def wait(self) -> None:
        """Wait for all tasks to complete."""
        if self._tasks:
            if self._timeout:
                await asyncio.wait_for(
                    asyncio.gather(*self._tasks, return_exceptions=True),
                    timeout=self._timeout,
                )
            else:
                await asyncio.gather(*self._tasks, return_exceptions=True)
    
    async def results(self) -> list[Any]:
        """Wait and get all results."""
        await self.wait()
        return [task.result() for task in self._tasks if not task.cancelled()]
    
    @property
    def pending_count(self) -> int:
        """Number of pending tasks."""
        return len([t for t in self._tasks if not t.done()])
    
    def cancel_all(self) -> None:
        """Cancel all pending tasks."""
        for task in self._tasks:
            if not task.done():
                task.cancel()
