"""
Testing utilities - Async helpers and log capture.
"""

import asyncio
import logging
from typing import Any, Callable, TypeVar, Awaitable
from contextlib import contextmanager
from io import StringIO

T = TypeVar("T")


def run_async(coro: Awaitable[T]) -> T:
    """Run an async coroutine synchronously.
    
    Example:
        result = run_async(async_function())
    """
    return asyncio.run(coro)


async def wait_for(
    condition: Callable[[], bool | Awaitable[bool]],
    timeout: float = 5.0,
    interval: float = 0.1,
) -> bool:
    """Wait for a condition to become true.
    
    Args:
        condition: Function that returns True when condition is met.
        timeout: Maximum time to wait in seconds.
        interval: Check interval in seconds.
        
    Returns:
        True if condition was met, False if timeout.
        
    Example:
        async def test_processing():
            await service.start_processing()
            success = await wait_for(lambda: service.is_complete, timeout=10)
            assert success
    """
    elapsed = 0.0
    while elapsed < timeout:
        result = condition()
        if asyncio.iscoroutine(result):
            result = await result
        if result:
            return True
        await asyncio.sleep(interval)
        elapsed += interval
    return False


@contextmanager
def capture_logs(logger_name: str | None = None, level: int = logging.DEBUG):
    """Capture log output for testing.
    
    Args:
        logger_name: Logger name to capture (None for root).
        level: Minimum log level to capture.
        
    Yields:
        StringIO containing log output.
        
    Example:
        with capture_logs("myapp") as logs:
            do_something()
            assert "expected message" in logs.getvalue()
    """
    stream = StringIO()
    handler = logging.StreamHandler(stream)
    handler.setLevel(level)
    handler.setFormatter(logging.Formatter("%(levelname)s - %(message)s"))
    
    logger = logging.getLogger(logger_name)
    original_level = logger.level
    logger.setLevel(level)
    logger.addHandler(handler)
    
    try:
        yield stream
    finally:
        logger.removeHandler(handler)
        logger.setLevel(original_level)


class TimeMeasure:
    """Context manager to measure execution time.
    
    Example:
        with TimeMeasure() as timer:
            do_something()
        assert timer.elapsed < 1.0
    """
    
    def __init__(self) -> None:
        self.start: float = 0
        self.end: float = 0
    
    @property
    def elapsed(self) -> float:
        return self.end - self.start
    
    def __enter__(self) -> "TimeMeasure":
        import time
        self.start = time.perf_counter()
        return self
    
    def __exit__(self, *args: Any) -> None:
        import time
        self.end = time.perf_counter()


def retry_test(
    times: int = 3,
    exceptions: tuple[type[Exception], ...] = (Exception,),
) -> Callable[[Callable[..., T]], Callable[..., T]]:
    """Decorator to retry flaky tests.
    
    Example:
        @retry_test(times=3)
        def test_flaky_integration():
            # May fail due to network issues
            pass
    """
    def decorator(func: Callable[..., T]) -> Callable[..., T]:
        def wrapper(*args: Any, **kwargs: Any) -> T:
            last_error: Exception | None = None
            for _ in range(times):
                try:
                    return func(*args, **kwargs)
                except exceptions as e:
                    last_error = e
            raise last_error  # type: ignore
        return wrapper
    return decorator
