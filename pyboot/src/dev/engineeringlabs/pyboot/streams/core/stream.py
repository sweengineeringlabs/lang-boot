"""Stream implementation with fluent API."""

import asyncio
from typing import (
    TypeVar, Generic, Callable, Awaitable, AsyncIterator, 
    Iterable, Any, AsyncIterable
)

T = TypeVar("T")
R = TypeVar("R")


class Stream(Generic[T]):
    """Async stream with fluent operators.
    
    Example:
        # Create from data
        stream = Stream.from_iterable([1, 2, 3, 4, 5])
        
        # Chain operators
        result = await (stream
            .map(lambda x: x * 2)
            .filter(lambda x: x > 4)
            .take(3)
            .collect())
        
        # result: [6, 8, 10]
    """
    
    def __init__(self, source: AsyncIterator[T]) -> None:
        self._source = source
    
    @classmethod
    def from_iterable(cls, items: Iterable[T]) -> "Stream[T]":
        """Create stream from iterable."""
        async def generate() -> AsyncIterator[T]:
            for item in items:
                yield item
        return cls(generate())
    
    @classmethod
    def from_async_iterable(cls, items: AsyncIterable[T]) -> "Stream[T]":
        """Create stream from async iterable."""
        return cls(items.__aiter__())
    
    @classmethod
    def from_callable(cls, fn: Callable[[], Awaitable[T | None]], until_none: bool = True) -> "Stream[T]":
        """Create stream from async callable."""
        async def generate() -> AsyncIterator[T]:
            while True:
                item = await fn()
                if item is None and until_none:
                    break
                if item is not None:
                    yield item
        return cls(generate())
    
    def map(self, fn: Callable[[T], R]) -> "Stream[R]":
        """Map items through function."""
        source = self._source
        async def generate() -> AsyncIterator[R]:
            async for item in source:
                yield fn(item)
        return Stream(generate())
    
    def map_async(self, fn: Callable[[T], Awaitable[R]]) -> "Stream[R]":
        """Map items through async function."""
        source = self._source
        async def generate() -> AsyncIterator[R]:
            async for item in source:
                yield await fn(item)
        return Stream(generate())
    
    def filter(self, predicate: Callable[[T], bool]) -> "Stream[T]":
        """Filter items by predicate."""
        source = self._source
        async def generate() -> AsyncIterator[T]:
            async for item in source:
                if predicate(item):
                    yield item
        return Stream(generate())
    
    def filter_async(self, predicate: Callable[[T], Awaitable[bool]]) -> "Stream[T]":
        """Filter items by async predicate."""
        source = self._source
        async def generate() -> AsyncIterator[T]:
            async for item in source:
                if await predicate(item):
                    yield item
        return Stream(generate())
    
    def take(self, n: int) -> "Stream[T]":
        """Take first n items."""
        source = self._source
        async def generate() -> AsyncIterator[T]:
            count = 0
            async for item in source:
                if count >= n:
                    break
                yield item
                count += 1
        return Stream(generate())
    
    def skip(self, n: int) -> "Stream[T]":
        """Skip first n items."""
        source = self._source
        async def generate() -> AsyncIterator[T]:
            count = 0
            async for item in source:
                if count >= n:
                    yield item
                count += 1
        return Stream(generate())
    
    def take_while(self, predicate: Callable[[T], bool]) -> "Stream[T]":
        """Take items while predicate is true."""
        source = self._source
        async def generate() -> AsyncIterator[T]:
            async for item in source:
                if not predicate(item):
                    break
                yield item
        return Stream(generate())
    
    def flatten(self) -> "Stream[Any]":
        """Flatten nested iterables."""
        source = self._source
        async def generate() -> AsyncIterator[Any]:
            async for item in source:
                if hasattr(item, "__iter__"):
                    for sub in item:
                        yield sub
                else:
                    yield item
        return Stream(generate())
    
    def distinct(self) -> "Stream[T]":
        """Remove duplicate items."""
        source = self._source
        async def generate() -> AsyncIterator[T]:
            seen: set[T] = set()
            async for item in source:
                if item not in seen:
                    seen.add(item)
                    yield item
        return Stream(generate())
    
    def enumerate(self) -> "Stream[tuple[int, T]]":
        """Add index to items."""
        source = self._source
        async def generate() -> AsyncIterator[tuple[int, T]]:
            index = 0
            async for item in source:
                yield (index, item)
                index += 1
        return Stream(generate())
    
    async def collect(self) -> list[T]:
        """Collect all items into list."""
        return [item async for item in self._source]
    
    async def reduce(self, fn: Callable[[R, T], R], initial: R) -> R:
        """Reduce stream to single value."""
        result = initial
        async for item in self._source:
            result = fn(result, item)
        return result
    
    async def first(self) -> T | None:
        """Get first item."""
        async for item in self._source:
            return item
        return None
    
    async def last(self) -> T | None:
        """Get last item."""
        result: T | None = None
        async for item in self._source:
            result = item
        return result
    
    async def count(self) -> int:
        """Count items."""
        total = 0
        async for _ in self._source:
            total += 1
        return total
    
    async def for_each(self, fn: Callable[[T], None]) -> None:
        """Execute function for each item."""
        async for item in self._source:
            fn(item)
    
    async def for_each_async(self, fn: Callable[[T], Awaitable[None]]) -> None:
        """Execute async function for each item."""
        async for item in self._source:
            await fn(item)
    
    def __aiter__(self) -> AsyncIterator[T]:
        return self._source


# Factory functions
def from_iterable(items: Iterable[T]) -> Stream[T]:
    """Create stream from iterable."""
    return Stream.from_iterable(items)


def from_async_iterable(items: AsyncIterable[T]) -> Stream[T]:
    """Create stream from async iterable."""
    return Stream.from_async_iterable(items)


async def from_queue(queue: asyncio.Queue[T]) -> Stream[T]:
    """Create stream from asyncio queue."""
    async def generate() -> AsyncIterator[T]:
        while True:
            item = await queue.get()
            if item is None:
                break
            yield item
    return Stream(generate())


def interval(seconds: float, count: int | None = None) -> Stream[int]:
    """Create stream that emits at intervals."""
    async def generate() -> AsyncIterator[int]:
        i = 0
        while count is None or i < count:
            yield i
            await asyncio.sleep(seconds)
            i += 1
    return Stream(generate())
