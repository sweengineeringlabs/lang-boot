"""Streams Core."""

from dev.engineeringlabs.pyboot.streams.core.stream import (
    Stream,
    from_iterable,
    from_async_iterable,
    from_queue,
    interval,
)

from dev.engineeringlabs.pyboot.streams.core.parallel import (
    parallel,
    parallel_map,
    parallel_filter,
    TaskPool,
)

from dev.engineeringlabs.pyboot.streams.core.operators import (
    merge,
    concat,
    zip_streams,
)

from dev.engineeringlabs.pyboot.streams.core.collectors import (
    collect,
    collect_to_list,
    first,
    last,
    count,
)

__all__ = [
    # Stream
    "Stream",
    "from_iterable",
    "from_async_iterable",
    "from_queue",
    "interval",
    # Parallel
    "parallel",
    "parallel_map",
    "parallel_filter",
    "TaskPool",
    # Operators
    "merge",
    "concat",
    "zip_streams",
    # Collectors
    "collect",
    "collect_to_list",
    "first",
    "last",
    "count",
]
