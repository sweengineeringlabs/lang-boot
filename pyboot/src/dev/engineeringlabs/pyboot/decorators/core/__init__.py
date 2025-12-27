"""Decorators Core - Decorator implementations."""

import functools
import warnings
from typing import Callable, TypeVar, Any, ParamSpec


P = ParamSpec("P")
T = TypeVar("T")


def compose(*decorators: Callable[[Callable[P, T]], Callable[P, T]]) -> Callable[[Callable[P, T]], Callable[P, T]]:
    """Compose multiple decorators into one.
    
    Decorators are applied from right to left (innermost first),
    matching the order when stacked vertically.
    
    Example:
        @compose(retryable(3), timeout(10), cached(ttl=60))
        async def fetch_data():
            pass
        
        # Equivalent to:
        @retryable(3)
        @timeout(10)
        @cached(ttl=60)
        async def fetch_data():
            pass
    """
    def composed_decorator(func: Callable[P, T]) -> Callable[P, T]:
        result = func
        for decorator in reversed(decorators):
            result = decorator(result)
        return result
    return composed_decorator


def conditional(
    condition: bool | Callable[[], bool],
    decorator: Callable[[Callable[P, T]], Callable[P, T]],
) -> Callable[[Callable[P, T]], Callable[P, T]]:
    """Apply a decorator conditionally.
    
    Example:
        @conditional(config.enable_cache, cached(ttl=60))
        async def fetch_data():
            pass
    """
    def conditional_decorator(func: Callable[P, T]) -> Callable[P, T]:
        should_apply = condition() if callable(condition) else condition
        if should_apply:
            return decorator(func)
        return func
    return conditional_decorator


def debug(
    enabled: bool = True,
    prefix: str = "[DEBUG]",
) -> Callable[[Callable[P, T]], Callable[P, T]]:
    """Debug decorator execution.
    
    Prints function entry/exit with arguments and return value.
    
    Example:
        @debug()
        def calculate(x: int, y: int) -> int:
            return x + y
    """
    def debug_decorator(func: Callable[P, T]) -> Callable[P, T]:
        if not enabled:
            return func
        
        @functools.wraps(func)
        def wrapper(*args: Any, **kwargs: Any) -> T:
            args_str = ", ".join(repr(a) for a in args)
            kwargs_str = ", ".join(f"{k}={v!r}" for k, v in kwargs.items())
            all_args = ", ".join(filter(None, [args_str, kwargs_str]))
            
            print(f"{prefix} {func.__name__}({all_args})")
            
            try:
                result = func(*args, **kwargs)
                print(f"{prefix} {func.__name__} -> {result!r}")
                return result
            except Exception as e:
                print(f"{prefix} {func.__name__} raised {type(e).__name__}: {e}")
                raise
        
        return wrapper
    return debug_decorator


def memoize(func: Callable[P, T]) -> Callable[P, T]:
    """Simple memoization decorator.
    
    Caches results based on arguments (hashable only).
    
    Example:
        @memoize
        def fibonacci(n: int) -> int:
            if n < 2:
                return n
            return fibonacci(n - 1) + fibonacci(n - 2)
    """
    cache: dict[tuple[Any, ...], T] = {}
    
    @functools.wraps(func)
    def wrapper(*args: Any, **kwargs: Any) -> T:
        key = (args, tuple(sorted(kwargs.items())))
        if key not in cache:
            cache[key] = func(*args, **kwargs)
        return cache[key]
    
    wrapper.cache = cache  # type: ignore
    wrapper.cache_clear = cache.clear  # type: ignore
    return wrapper


def once(func: Callable[P, T]) -> Callable[P, T]:
    """Execute function only once, cache the result.
    
    Example:
        @once
        def load_config() -> dict:
            print("Loading config...")  # Only printed once
            return {"key": "value"}
    """
    result: list[T] = []
    called = False
    
    @functools.wraps(func)
    def wrapper(*args: Any, **kwargs: Any) -> T:
        nonlocal called
        if not called:
            result.append(func(*args, **kwargs))
            called = True
        return result[0]
    
    return wrapper


def deprecated(
    message: str = "",
    version: str | None = None,
) -> Callable[[Callable[P, T]], Callable[P, T]]:
    """Mark a function as deprecated.
    
    Example:
        @deprecated("Use new_function() instead", version="2.0")
        def old_function():
            pass
    """
    def deprecated_decorator(func: Callable[P, T]) -> Callable[P, T]:
        @functools.wraps(func)
        def wrapper(*args: Any, **kwargs: Any) -> T:
            warn_msg = f"{func.__name__} is deprecated"
            if version:
                warn_msg += f" since version {version}"
            if message:
                warn_msg += f". {message}"
            
            warnings.warn(warn_msg, DeprecationWarning, stacklevel=2)
            return func(*args, **kwargs)
        
        return wrapper
    return deprecated_decorator


__all__ = [
    "compose",
    "conditional",
    "debug",
    "memoize",
    "once",
    "deprecated",
]
