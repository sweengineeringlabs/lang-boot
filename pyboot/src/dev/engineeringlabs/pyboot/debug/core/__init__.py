"""Debug Core - Debug utilities implementation."""

import time
import functools
import sys
from typing import Callable, TypeVar, Any
from datetime import datetime
from dev.engineeringlabs.pyboot.debug.api import DebugLevel, DebugConfig


T = TypeVar("T")

_config = DebugConfig()


def debug_log(
    message: str,
    level: DebugLevel = DebugLevel.DEBUG,
    **context: Any,
) -> None:
    """Log a debug message."""
    if not _config.enabled:
        return
    
    if level.value < _config.level.value:
        return
    
    parts = []
    
    if _config.show_timestamps:
        parts.append(f"[{datetime.now().isoformat()}]")
    
    parts.append(f"[{level.name}]")
    parts.append(message)
    
    if context:
        ctx_str = " ".join(f"{k}={v}" for k, v in context.items())
        parts.append(f"({ctx_str})")
    
    print(" ".join(parts), file=sys.stderr)


def timed(func: Callable[..., T]) -> Callable[..., T]:
    """Decorator to time function execution."""
    @functools.wraps(func)
    def wrapper(*args: Any, **kwargs: Any) -> T:
        start = time.perf_counter()
        try:
            return func(*args, **kwargs)
        finally:
            elapsed = time.perf_counter() - start
            debug_log(
                f"{func.__name__} completed",
                level=DebugLevel.DEBUG,
                elapsed_ms=f"{elapsed * 1000:.2f}",
            )
    return wrapper


def memory_usage() -> dict[str, Any]:
    """Get current memory usage."""
    try:
        import resource
        usage = resource.getrusage(resource.RUSAGE_SELF)
        return {
            "max_rss_kb": usage.ru_maxrss,
        }
    except ImportError:
        return {"error": "resource module not available"}


class Timer:
    """Context manager for timing code blocks."""
    
    def __init__(self, name: str = "timer") -> None:
        self.name = name
        self.start_time: float = 0
        self.elapsed: float = 0
    
    def __enter__(self) -> "Timer":
        self.start_time = time.perf_counter()
        return self
    
    def __exit__(self, *args: Any) -> None:
        self.elapsed = time.perf_counter() - self.start_time
        debug_log(
            f"{self.name} completed",
            level=DebugLevel.DEBUG,
            elapsed_ms=f"{self.elapsed * 1000:.2f}",
        )


class Profiler:
    """Simple profiler for code blocks."""
    
    def __init__(self) -> None:
        self._timings: dict[str, list[float]] = {}
    
    def record(self, name: str, elapsed: float) -> None:
        """Record a timing."""
        if name not in self._timings:
            self._timings[name] = []
        self._timings[name].append(elapsed)
    
    def summary(self) -> dict[str, dict[str, float]]:
        """Get timing summary."""
        result = {}
        for name, timings in self._timings.items():
            result[name] = {
                "count": len(timings),
                "total_ms": sum(timings) * 1000,
                "avg_ms": (sum(timings) / len(timings)) * 1000 if timings else 0,
                "min_ms": min(timings) * 1000 if timings else 0,
                "max_ms": max(timings) * 1000 if timings else 0,
            }
        return result


__all__ = [
    "debug_log",
    "timed",
    "memory_usage",
    "Timer",
    "Profiler",
]
