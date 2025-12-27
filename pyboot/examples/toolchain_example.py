"""
PyBoot Examples - Toolchain

Demonstrates build and environment utilities.
"""

from dev.engineeringlabs.pyboot.toolchain import (
    get_version,
    get_python_version,
    get_environment,
    is_debug,
    is_production,
    get_build_info,
    Environment,
    BuildMode,
)


# Example 1: Version info
print("=" * 50)
print("Example 1: Version Information")
print("=" * 50)

print(f"PyBoot version: {get_version()}")
print(f"Python version: {get_python_version()}")
print()


# Example 2: Environment detection
print("=" * 50)
print("Example 2: Environment Detection")
print("=" * 50)

env = get_environment()
print(f"Current environment: {env.name}")
print(f"Is debug mode: {is_debug()}")
print(f"Is production: {is_production()}")
print()


# Example 3: Build info
print("=" * 50)
print("Example 3: Build Information")
print("=" * 50)

info = get_build_info()
print(f"Version:     {info.version}")
print(f"Python:      {info.python_version}")
print(f"Platform:    {info.platform}")
print(f"Environment: {info.environment.name}")
print(f"Build mode:  {info.build_mode.name}")
print(f"Timestamp:   {info.timestamp}")
print()


# Example 4: Environment-based configuration
print("=" * 50)
print("Example 4: Environment-based Config")
print("=" * 50)


def get_database_url() -> str:
    """Get database URL based on environment."""
    env = get_environment()
    
    if env == Environment.PRODUCTION:
        return "postgresql://prod-db:5432/app"
    elif env == Environment.STAGING:
        return "postgresql://staging-db:5432/app"
    elif env == Environment.TEST:
        return "sqlite:///:memory:"
    else:  # DEVELOPMENT
        return "postgresql://localhost:5432/app_dev"


print(f"Database URL: {get_database_url()}")
print()


# Example 5: Debug vs Release behavior
print("=" * 50)
print("Example 5: Debug vs Release Behavior")
print("=" * 50)


def expensive_validation(data: dict) -> bool:
    """Expensive validation only in debug mode."""
    if is_debug():
        print("  Running expensive debug validations...")
        # In real code, this would do thorough validation
        return all(isinstance(v, (str, int, float, bool, type(None))) for v in data.values())
    return True


data = {"name": "test", "value": 42}
print(f"Validation result: {expensive_validation(data)}")
