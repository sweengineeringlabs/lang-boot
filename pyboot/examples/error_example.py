"""
PyBoot Examples - Error Module

Demonstrates Result monad and error handling patterns.
"""

from dev.engineeringlabs.pyboot.error import Result, Ok, Err, ErrorCode, PybootError, chain_errors, wrap_error


# Example 1: Basic Result usage
print("=" * 50)
print("Example 1: Basic Result Usage")
print("=" * 50)


def divide(a: float, b: float) -> Result[float, str]:
    """Divide two numbers, returning Result."""
    if b == 0:
        return Err("Division by zero")
    return Ok(a / b)


result1 = divide(10, 2)
result2 = divide(10, 0)

print(f"divide(10, 2) -> is_ok: {result1.is_ok}, value: {result1.unwrap()}")
print(f"divide(10, 0) -> is_err: {result2.is_err}, error: {result2.unwrap_err()}")
print()


# Example 2: Result chaining with map and and_then
print("=" * 50)
print("Example 2: Result Chaining")
print("=" * 50)


def parse_int(s: str) -> Result[int, str]:
    """Parse string to int."""
    try:
        return Ok(int(s))
    except ValueError:
        return Err(f"Cannot parse '{s}' as int")


def double(n: int) -> Result[int, str]:
    """Double a number."""
    return Ok(n * 2)


# Chain operations
result = parse_int("21").and_then(double)
print(f"parse_int('21').and_then(double) = {result.unwrap()}")

result = parse_int("abc").and_then(double)
print(f"parse_int('abc').and_then(double) = {result.unwrap_err()}")

# Map values
result = parse_int("5").map(lambda x: x ** 2)
print(f"parse_int('5').map(x => x²) = {result.unwrap()}")
print()


# Example 3: unwrap_or for defaults
print("=" * 50)
print("Example 3: Default Values")
print("=" * 50)

result = parse_int("invalid").unwrap_or(0)
print(f"parse_int('invalid').unwrap_or(0) = {result}")

result = parse_int("42").unwrap_or(0)
print(f"parse_int('42').unwrap_or(0) = {result}")
print()


# Example 4: PybootError with error codes
print("=" * 50)
print("Example 4: PybootError")
print("=" * 50)


class ValidationError(PybootError):
    """Validation error."""
    def __init__(self, field: str, message: str):
        super().__init__(
            message=f"Validation failed for '{field}': {message}",
            code=ErrorCode.VALIDATION,
            details={"field": field},
        )


try:
    raise ValidationError("email", "invalid format")
except PybootError as e:
    print(f"Error: {e}")
    print(f"Code: {e.code.name}")
    print(f"Details: {e.details}")
    print(f"Dict: {e.to_dict()}")
print()


# Example 5: Error chaining
print("=" * 50)
print("Example 5: Error Chaining")
print("=" * 50)


def process_file(path: str) -> None:
    """Process a file with error wrapping."""
    try:
        # Simulate IO error
        raise FileNotFoundError(f"File not found: {path}")
    except FileNotFoundError as e:
        raise wrap_error(e, PybootError, f"Failed to process {path}")


try:
    process_file("config.yaml")
except PybootError as e:
    print(f"Caught: {e}")
    print(f"Cause: {e.__cause__}")
print()


# Example 6: Pattern matching style
print("=" * 50)
print("Example 6: Pattern Matching Style")
print("=" * 50)


def fetch_user(user_id: int) -> Result[dict, str]:
    """Fetch user by ID."""
    users = {1: {"name": "Alice"}, 2: {"name": "Bob"}}
    if user_id in users:
        return Ok(users[user_id])
    return Err(f"User {user_id} not found")


def display_user(user_id: int) -> str:
    result = fetch_user(user_id)
    if result.is_ok:
        return f"Found: {result.unwrap()['name']}"
    else:
        return f"Error: {result.unwrap_err()}"


print(display_user(1))
print(display_user(999))
