"""Dependency Injection container and decorators.

Provides Java-style dependency injection with @Provider and @Inject decorators.

Example:
    # Register a provider
    @Provider
    class RedisCacheProvider(CacheProvider):
        def get_name(self) -> str:
            return "redis"

    # Inject dependencies
    class UserService:
        @Inject
        cache_provider: CacheProvider

        def run(self):
            print(self.cache_provider.get_name())  # "redis"

    # Or use the container directly
    container = Container()
    container.register(CacheProvider, RedisCacheProvider)
    provider = container.get(CacheProvider)
    provider.get_name()  # Works!
"""

from dev.engineeringlabs.pyboot.di.core.container import (
    # Core classes
    Container,
    Scope,
    ProviderMetadata,
    Registration,
    # Decorators
    Provider,
    Inject,
    # Errors
    ContainerError,
    ProviderNotFoundError,
    CircularDependencyError,
    AmbiguousProviderError,
    # Global functions
    get_container,
    set_container,
    reset_container,
    inject,
    register,
)

__all__ = [
    # Core classes
    "Container",
    "Scope",
    "ProviderMetadata",
    "Registration",
    # Decorators
    "Provider",
    "Inject",
    # Errors
    "ContainerError",
    "ProviderNotFoundError",
    "CircularDependencyError",
    "AmbiguousProviderError",
    # Global functions
    "get_container",
    "set_container",
    "reset_container",
    "inject",
    "register",
]
