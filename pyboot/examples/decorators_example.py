"""
PyBoot Examples - Decorators Module

Demonstrates decorator composition and utilities.
"""

from dev.engineeringlabs.pyboot.decorators import compose, conditional, debug, memoize, once, deprecated


# Example 1: Compose multiple decorators
print("=" * 50)
print("Example 1: Compose Decorators")
print("=" * 50)


def log_entry(func):
    """Simple logging decorator."""
    def wrapper(*args, **kwargs):
        print(f"  -> Calling {func.__name__}")
        return func(*args, **kwargs)
    return wrapper


def log_exit(func):
    """Simple exit logging decorator."""
    def wrapper(*args, **kwargs):
        result = func(*args, **kwargs)
        print(f"  <- {func.__name__} returned {result}")
        return result
    return wrapper


@compose(log_entry, log_exit)
def add(a: int, b: int) -> int:
    """Add two numbers."""
    return a + b


result = add(3, 5)
print(f"Result: {result}\n")


# Example 2: Debug decorator
print("=" * 50)
print("Example 2: Debug Decorator")
print("=" * 50)


@debug(enabled=True, prefix="[TRACE]")
def multiply(x: int, y: int) -> int:
    """Multiply two numbers."""
    return x * y


multiply(4, 7)
print()


# Example 3: Memoization
print("=" * 50)
print("Example 3: Memoization")
print("=" * 50)


@memoize
def fibonacci(n: int) -> int:
    """Calculate Fibonacci number (with memoization)."""
    if n < 2:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)


print(f"fibonacci(10) = {fibonacci(10)}")
print(f"fibonacci(20) = {fibonacci(20)}")
print(f"fibonacci(30) = {fibonacci(30)}")
print(f"Cache has {len(fibonacci.cache)} entries\n")


# Example 4: Once decorator
print("=" * 50)
print("Example 4: Once Decorator")
print("=" * 50)


@once
def initialize_config() -> dict:
    """Initialize config (only runs once)."""
    print("  Initializing config...")
    return {"database": "postgres", "port": 5432}


print("First call:")
config1 = initialize_config()
print(f"  Config: {config1}")

print("Second call (cached):")
config2 = initialize_config()
print(f"  Config: {config2}")

print(f"Same object? {config1 is config2}\n")


# Example 5: Conditional decorator
print("=" * 50)
print("Example 5: Conditional Decorator")
print("=" * 50)

ENABLE_LOGGING = True


@conditional(ENABLE_LOGGING, debug(prefix="[LOG]"))
def process_data(data: str) -> str:
    """Process data with optional logging."""
    return data.upper()


result = process_data("hello world")
print(f"Result: {result}\n")


# Example 6: Deprecated decorator
print("=" * 50)
print("Example 6: Deprecated Decorator")
print("=" * 50)

import warnings
warnings.filterwarnings("always", category=DeprecationWarning)


@deprecated("Use new_api() instead", version="2.0")
def old_api() -> str:
    """Old API function."""
    return "old result"


print("Calling deprecated function:")
result = old_api()
print(f"Result: {result}")
