"""Stream operators - merge, concat, zip."""

import asyncio
from typing import TypeVar, AsyncIterator
from dev.engineeringlabs.pyboot.streams.core.stream import Stream

T = TypeVar("T")


async def merge(*streams: Stream[T]) -> Stream[T]:
    """Merge multiple streams into one (interleaved).
    
    Items are emitted as soon as they're available from any stream.
    
    Example:
        stream1 = Stream.from_iterable([1, 2, 3])
        stream2 = Stream.from_iterable([4, 5, 6])
        merged = await merge(stream1, stream2)
        result = await merged.collect()
        # [1, 4, 2, 5, 3, 6] or similar interleaving
    """
    async def generate() -> AsyncIterator[T]:
        queues = [asyncio.Queue() for _ in streams]
        done_count = 0
        total = len(streams)
        
        async def producer(stream: Stream[T], queue: asyncio.Queue) -> None:
            async for item in stream:
                await queue.put(item)
            await queue.put(None)  # Signal done
        
        # Start producers
        tasks = [
            asyncio.create_task(producer(s, q))
            for s, q in zip(streams, queues)
        ]
        
        # Round-robin consume
        active_queues = list(enumerate(queues))
        while active_queues:
            for i, (idx, queue) in enumerate(list(active_queues)):
                try:
                    item = queue.get_nowait()
                    if item is None:
                        active_queues.remove((idx, queue))
                    else:
                        yield item
                except asyncio.QueueEmpty:
                    pass
            
            if active_queues:
                await asyncio.sleep(0)
        
        # Wait for producers
        for task in tasks:
            await task
    
    return Stream(generate())


async def concat(*streams: Stream[T]) -> Stream[T]:
    """Concatenate streams sequentially.
    
    Example:
        stream1 = Stream.from_iterable([1, 2])
        stream2 = Stream.from_iterable([3, 4])
        combined = await concat(stream1, stream2)
        result = await combined.collect()
        # [1, 2, 3, 4]
    """
    async def generate() -> AsyncIterator[T]:
        for stream in streams:
            async for item in stream:
                yield item
    
    return Stream(generate())


async def zip_streams(*streams: Stream[T]) -> Stream[tuple]:
    """Zip streams together.
    
    Emits tuples of items from each stream. Stops when shortest stream ends.
    
    Example:
        stream1 = Stream.from_iterable([1, 2, 3])
        stream2 = Stream.from_iterable(['a', 'b', 'c'])
        zipped = await zip_streams(stream1, stream2)
        result = await zipped.collect()
        # [(1, 'a'), (2, 'b'), (3, 'c')]
    """
    async def generate() -> AsyncIterator[tuple]:
        iterators = [s.__aiter__() for s in streams]
        
        while True:
            items = []
            for it in iterators:
                try:
                    item = await it.__anext__()
                    items.append(item)
                except StopAsyncIteration:
                    return
            yield tuple(items)
    
    return Stream(generate())
