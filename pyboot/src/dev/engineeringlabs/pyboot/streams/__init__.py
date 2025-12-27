"""
Streams Module - Reactive stream processing with parallel task execution.

This module provides:
- Async stream processing
- Parallel task execution
- Stream operators (map, filter, reduce)
- Backpressure handling
- Stream composition

Example:
    from dev.engineeringlabs.pyboot.streams import Stream, parallel, merge, from_iterable
    
    # Create stream from data
    stream = Stream.from_iterable([1, 2, 3, 4, 5])
    
    # Transform with operators
    result = await (stream
        .map(lambda x: x * 2)
        .filter(lambda x: x > 4)
        .collect())
    
    # Parallel processing
    async def process_item(item):
        await asyncio.sleep(0.1)
        return item * 2
    
    results = await parallel(
        items=[1, 2, 3, 4, 5],
        processor=process_item,
        concurrency=3,
    )
    
    # Merge multiple streams
    stream1 = Stream.from_iterable([1, 2, 3])
    stream2 = Stream.from_iterable([4, 5, 6])
    merged = merge(stream1, stream2)
"""

from dev.engineeringlabs.pyboot.streams.api import (
    # Types
    StreamItem,
    StreamConfig,
    # Protocols
    Publisher,
    Subscriber,
    Processor,
    # Exceptions
    StreamError,
    BackpressureError,
)

from dev.engineeringlabs.pyboot.streams.core import (
    # Main stream class
    Stream,
    # Parallel execution
    parallel,
    parallel_map,
    parallel_filter,
    TaskPool,
    # Operators
    merge,
    concat,
    zip_streams,
    # Factories
    from_iterable,
    from_async_iterable,
    from_queue,
    interval,
    # Collectors
    collect,
    collect_to_list,
    first,
    last,
    count,
)

__all__ = [
    # API
    "StreamItem",
    "StreamConfig",
    "Publisher",
    "Subscriber",
    "Processor",
    "StreamError",
    "BackpressureError",
    # Core - Stream
    "Stream",
    # Core - Parallel
    "parallel",
    "parallel_map",
    "parallel_filter",
    "TaskPool",
    # Core - Operators
    "merge",
    "concat",
    "zip_streams",
    # Core - Factories
    "from_iterable",
    "from_async_iterable",
    "from_queue",
    "interval",
    # Core - Collectors
    "collect",
    "collect_to_list",
    "first",
    "last",
    "count",
]
