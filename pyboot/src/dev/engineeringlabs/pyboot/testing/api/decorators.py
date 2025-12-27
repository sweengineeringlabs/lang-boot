"""
Test decorators - fixture, async_test, parametrize.
"""

import asyncio
import functools
from typing import Any, Callable, TypeVar, ParamSpec

P = ParamSpec("P")
T = TypeVar("T")


def fixture(
    scope: str = "function",
    autouse: bool = False,
    name: str | None = None,
) -> Callable[[Callable[P, T]], Callable[P, T]]:
    """Mark a method as a test fixture.
    
    Fixtures provide test data and setup/teardown logic.
    Compatible with pytest's fixture system.
    
    Args:
        scope: Fixture scope ("function", "class", "module", "session").
        autouse: Automatically use this fixture in tests.
        name: Override fixture name.
        
    Example:
        class Tests:
            @fixture
            def user(self) -> User:
                return User(name="Test")
            
            @fixture(scope="class")
            def database(self) -> Database:
                db = Database()
                yield db
                db.close()
    """
    def decorator(func: Callable[P, T]) -> Callable[P, T]:
        func._fixture = True  # type: ignore
        func._fixture_scope = scope  # type: ignore
        func._fixture_autouse = autouse  # type: ignore
        func._fixture_name = name or func.__name__  # type: ignore
        return func
    return decorator


def async_test(func: Callable[P, T]) -> Callable[P, T]:
    """Decorator to run async test functions.
    
    Automatically wraps async test functions to run in event loop.
    Works with pytest and unittest.
    
    Example:
        @async_test
        async def test_async_operation():
            result = await service.fetch_data()
            assert result is not None
    """
    @functools.wraps(func)
    def wrapper(*args: P.args, **kwargs: P.kwargs) -> T:
        return asyncio.run(func(*args, **kwargs))  # type: ignore
    return wrapper  # type: ignore


def parametrize(
    argnames: str | list[str],
    argvalues: list[Any],
    ids: list[str] | None = None,
) -> Callable[[Callable[P, T]], Callable[P, T]]:
    """Parametrize a test function with multiple inputs.
    
    Similar to pytest.mark.parametrize but framework-agnostic.
    
    Args:
        argnames: Comma-separated argument names or list.
        argvalues: List of argument value tuples.
        ids: Optional test IDs for each case.
        
    Example:
        @parametrize("a,b,expected", [
            (1, 1, 2),
            (2, 3, 5),
            (0, 0, 0),
        ])
        def test_add(a: int, b: int, expected: int):
            assert add(a, b) == expected
    """
    def decorator(func: Callable[P, T]) -> Callable[P, T]:
        func._parametrize = True  # type: ignore
        func._parametrize_argnames = argnames  # type: ignore
        func._parametrize_argvalues = argvalues  # type: ignore
        func._parametrize_ids = ids  # type: ignore
        return func
    return decorator


def skip(reason: str = "") -> Callable[[Callable[P, T]], Callable[P, T]]:
    """Skip a test with optional reason."""
    def decorator(func: Callable[P, T]) -> Callable[P, T]:
        func._skip = True  # type: ignore
        func._skip_reason = reason  # type: ignore
        return func
    return decorator


def skip_if(condition: bool, reason: str = "") -> Callable[[Callable[P, T]], Callable[P, T]]:
    """Conditionally skip a test."""
    def decorator(func: Callable[P, T]) -> Callable[P, T]:
        if condition:
            func._skip = True  # type: ignore
            func._skip_reason = reason  # type: ignore
        return func
    return decorator
