"""Stream collectors - terminal operations."""

from typing import TypeVar, Callable
from dev.engineeringlabs.pyboot.streams.core.stream import Stream

T = TypeVar("T")
R = TypeVar("R")


async def collect(stream: Stream[T]) -> list[T]:
    """Collect stream to list."""
    return await stream.collect()


async def collect_to_list(stream: Stream[T]) -> list[T]:
    """Alias for collect."""
    return await stream.collect()


async def first(stream: Stream[T]) -> T | None:
    """Get first item from stream."""
    return await stream.first()


async def last(stream: Stream[T]) -> T | None:
    """Get last item from stream."""
    return await stream.last()


async def count(stream: Stream[T]) -> int:
    """Count items in stream."""
    return await stream.count()


async def reduce(stream: Stream[T], fn: Callable[[R, T], R], initial: R) -> R:
    """Reduce stream to single value."""
    return await stream.reduce(fn, initial)


async def sum_stream(stream: Stream[int | float]) -> int | float:
    """Sum numeric stream."""
    return await stream.reduce(lambda a, b: a + b, 0)


async def any_match(stream: Stream[T], predicate: Callable[[T], bool]) -> bool:
    """Check if any item matches predicate."""
    async for item in stream:
        if predicate(item):
            return True
    return False


async def all_match(stream: Stream[T], predicate: Callable[[T], bool]) -> bool:
    """Check if all items match predicate."""
    async for item in stream:
        if not predicate(item):
            return False
    return True


async def none_match(stream: Stream[T], predicate: Callable[[T], bool]) -> bool:
    """Check if no items match predicate."""
    async for item in stream:
        if predicate(item):
            return False
    return True
