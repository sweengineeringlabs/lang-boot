"""
PyBoot Examples - Debug Utilities

Demonstrates debugging and profiling utilities.
"""

import time
from dev.engineeringlabs.pyboot.debug import (
    debug_log,
    timed,
    memory_usage,
    Timer,
    Profiler,
    DebugLevel,
)


# Example 1: Debug logging
print("=" * 50)
print("Example 1: Debug Logging")
print("=" * 50)

debug_log("Application starting", level=DebugLevel.INFO)
debug_log("Loading configuration", level=DebugLevel.DEBUG, config_path="/etc/app/config.yaml")
debug_log("Database connected", level=DebugLevel.INFO, host="localhost", port=5432)
debug_log("This is a warning", level=DebugLevel.WARN)
print()


# Example 2: Timed decorator
print("=" * 50)
print("Example 2: Timed Decorator")
print("=" * 50)


@timed
def slow_operation():
    """A slow operation."""
    time.sleep(0.1)
    return "done"


result = slow_operation()
print(f"Result: {result}")
print()


# Example 3: Timer context manager
print("=" * 50)
print("Example 3: Timer Context Manager")
print("=" * 50)

with Timer("data_processing") as timer:
    # Simulate some work
    total = sum(range(100000))

print(f"Elapsed time: {timer.elapsed * 1000:.2f}ms")
print(f"Result: {total}")
print()


# Example 4: Profiler
print("=" * 50)
print("Example 4: Profiler")
print("=" * 50)

profiler = Profiler()


def operation_a():
    time.sleep(0.05)
    return "a"


def operation_b():
    time.sleep(0.02)
    return "b"


# Run operations and record timings
for _ in range(3):
    start = time.perf_counter()
    operation_a()
    profiler.record("operation_a", time.perf_counter() - start)
    
    start = time.perf_counter()
    operation_b()
    profiler.record("operation_b", time.perf_counter() - start)

summary = profiler.summary()
print("Profiler Summary:")
for name, stats in summary.items():
    print(f"  {name}:")
    print(f"    count: {stats['count']}")
    print(f"    total: {stats['total_ms']:.2f}ms")
    print(f"    avg: {stats['avg_ms']:.2f}ms")
print()


# Example 5: Memory usage
print("=" * 50)
print("Example 5: Memory Usage")
print("=" * 50)

mem = memory_usage()
print(f"Memory usage: {mem}")
print()


# Example 6: Combining debug tools
print("=" * 50)
print("Example 6: Combined Usage")
print("=" * 50)


@timed
def process_batch(items: list) -> list:
    """Process a batch of items with timing."""
    debug_log(f"Processing {len(items)} items", level=DebugLevel.DEBUG)
    results = []
    
    with Timer("batch_transform"):
        results = [item * 2 for item in items]
    
    debug_log(f"Processed {len(results)} items", level=DebugLevel.DEBUG)
    return results


batch = list(range(1000))
result = process_batch(batch)
print(f"Processed {len(result)} items")
