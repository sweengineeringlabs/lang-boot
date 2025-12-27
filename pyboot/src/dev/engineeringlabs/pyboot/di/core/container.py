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

from __future__ import annotations

import inspect
from contextlib import asynccontextmanager, contextmanager
from dataclasses import dataclass
from enum import Enum
from typing import (
    Any,
    Callable,
    ClassVar,
    Generic,
    TypeVar,
    get_type_hints,
)

T = TypeVar("T")


class Scope(str, Enum):
    """Dependency injection scopes."""

    SINGLETON = "singleton"  # One instance for entire application
    PROTOTYPE = "prototype"  # New instance every time
    REQUEST = "request"      # One instance per request/context
    THREAD = "thread"        # One instance per thread


@dataclass(frozen=True, slots=True)
class ProviderMetadata:
    """Metadata for a registered provider."""

    name: str
    scope: Scope = Scope.SINGLETON
    primary: bool = False
    qualifier: str | None = None
    priority: int = 0
    tags: tuple[str, ...] = ()


@dataclass
class Registration:
    """A registered dependency."""

    interface: type
    implementation: type | Callable[..., Any]
    metadata: ProviderMetadata
    instance: Any | None = None
    factory: Callable[..., Any] | None = None


class ContainerError(Exception):
    """Base exception for container errors."""
    pass


class ProviderNotFoundError(ContainerError):
    """Raised when a provider is not found."""
    pass


class CircularDependencyError(ContainerError):
    """Raised when a circular dependency is detected."""
    pass


class AmbiguousProviderError(ContainerError):
    """Raised when multiple providers match and none is primary."""
    pass


# Global registry for @Provider decorated classes
_provider_registry: dict[type, list[tuple[type, ProviderMetadata]]] = {}
_inject_fields: dict[type, dict[str, type]] = {}


def Provider(
    cls: type[T] | None = None,
    *,
    name: str | None = None,
    scope: Scope = Scope.SINGLETON,
    primary: bool = False,
    qualifier: str | None = None,
    priority: int = 0,
    tags: tuple[str, ...] | list[str] = (),
) -> type[T] | Callable[[type[T]], type[T]]:
    """
    Mark a class as a dependency provider.

    Similar to Java's @Component, @Service, @Repository, or @Provider.

    Args:
        cls: The class to decorate (when used without parentheses)
        name: Provider name for logging/debugging
        scope: Injection scope (SINGLETON, PROTOTYPE, REQUEST, THREAD)
        primary: If True, this provider is preferred when multiple match
        qualifier: Optional qualifier for disambiguation
        priority: Priority for ordering (higher = preferred)
        tags: Tags for categorization

    Example:
        @Provider
        class MyService:
            pass

        @Provider(scope=Scope.PROTOTYPE, primary=True)
        class PrimaryService(ServiceInterface):
            pass

        @Provider(qualifier="redis")
        class RedisCache(CacheProvider):
            pass
    """
    def decorator(cls: type[T]) -> type[T]:
        provider_name = name or cls.__name__
        metadata = ProviderMetadata(
            name=provider_name,
            scope=scope,
            primary=primary,
            qualifier=qualifier,
            priority=priority,
            tags=tuple(tags),
        )

        # Store metadata on class
        cls.__provider_metadata__ = metadata  # type: ignore[attr-defined]

        # Register for all base classes (interfaces)
        for base in cls.__mro__:
            if base not in (cls, object) and not base.__name__.startswith("_"):
                if base not in _provider_registry:
                    _provider_registry[base] = []
                _provider_registry[base].append((cls, metadata))

        # Also register for itself
        if cls not in _provider_registry:
            _provider_registry[cls] = []
        _provider_registry[cls].append((cls, metadata))

        return cls

    if cls is not None:
        return decorator(cls)
    return decorator


class _InjectDescriptor(Generic[T]):
    """Descriptor for @Inject fields."""

    def __init__(
        self,
        type_hint: type[T],
        qualifier: str | None = None,
        optional: bool = False,
    ) -> None:
        self.type_hint = type_hint
        self.qualifier = qualifier
        self.optional = optional
        self.name: str = ""

    def __set_name__(self, owner: type, name: str) -> None:
        self.name = name
        # Track inject fields for the class
        if owner not in _inject_fields:
            _inject_fields[owner] = {}
        _inject_fields[owner][name] = self.type_hint

    def __get__(self, obj: Any | None, owner: type) -> T | None:
        if obj is None:
            return self  # type: ignore[return-value]

        # Check if already resolved
        attr_name = f"_inject_{self.name}"
        if hasattr(obj, attr_name):
            return getattr(obj, attr_name)

        # Try to resolve from global container
        try:
            container = get_container()
            value = container.get(self.type_hint, qualifier=self.qualifier)
            setattr(obj, attr_name, value)
            return value
        except ProviderNotFoundError:
            if self.optional:
                return None
            raise

    def __set__(self, obj: Any, value: T) -> None:
        setattr(obj, f"_inject_{self.name}", value)


def Inject(
    type_hint: type[T] | None = None,
    *,
    qualifier: str | None = None,
    optional: bool = False,
) -> T:
    """
    Mark a class attribute for dependency injection.

    Similar to Java's @Inject or @Autowired.

    Args:
        type_hint: The type to inject (optional if using type annotations)
        qualifier: Optional qualifier for disambiguation
        optional: If True, injection failure returns None instead of error

    Example:
        class MyService:
            # Inject by type annotation
            cache_provider: CacheProvider = Inject()

            # Inject with qualifier
            redis: CacheProvider = Inject(qualifier="redis")

            # Optional injection
            metrics: MetricsService = Inject(optional=True)

            def process(self):
                result = self.cache_provider.get("key")
                return result
    """
    # This will be replaced by __set_name__ with proper type hint
    return _InjectDescriptor(type_hint or object, qualifier, optional)  # type: ignore[return-value]


class Container:
    """
    Dependency injection container.

    Manages registration and resolution of dependencies.

    Example:
        container = Container()

        # Register implementations
        container.register(CacheProvider, RedisCache)
        container.register(DatabaseService, PostgresDB, scope=Scope.SINGLETON)

        # Resolve dependencies
        provider = container.get(CacheProvider)
        provider.get_name()  # Works!

        # Auto-wire a class
        service = container.create(MyService)  # All @Inject fields resolved
    """

    _global_instance: ClassVar[Container | None] = None

    def __init__(self) -> None:
        self._registrations: dict[type, list[Registration]] = {}
        self._singletons: dict[tuple[type, str | None], Any] = {}
        self._request_scope: dict[tuple[type, str | None], Any] = {}
        self._resolving: set[type] = set()  # For circular dependency detection

        # Auto-register from @Provider decorated classes
        for interface, providers in _provider_registry.items():
            for impl, metadata in providers:
                self._add_registration(interface, impl, metadata)

    def _add_registration(
        self,
        interface: type,
        implementation: type | Callable[..., Any],
        metadata: ProviderMetadata,
    ) -> None:
        """Add a registration to the container."""
        if interface not in self._registrations:
            self._registrations[interface] = []

        reg = Registration(
            interface=interface,
            implementation=implementation,
            metadata=metadata,
        )
        self._registrations[interface].append(reg)

        # Sort by priority (highest first)
        self._registrations[interface].sort(
            key=lambda r: (-r.metadata.priority, not r.metadata.primary)
        )

    def register(
        self,
        interface: type[T],
        implementation: type[T] | Callable[..., T] | None = None,
        *,
        name: str | None = None,
        scope: Scope = Scope.SINGLETON,
        primary: bool = False,
        qualifier: str | None = None,
        priority: int = 0,
        instance: T | None = None,
    ) -> None:
        """
        Register a dependency.

        Args:
            interface: The interface/type to register
            implementation: The implementation class or factory
            name: Provider name
            scope: Injection scope
            primary: If True, preferred when multiple match
            qualifier: Optional qualifier
            priority: Priority for ordering
            instance: Pre-created instance (for SINGLETON scope)
        """
        if implementation is None and instance is None:
            implementation = interface

        metadata = ProviderMetadata(
            name=name or (implementation.__name__ if implementation else interface.__name__),
            scope=scope,
            primary=primary,
            qualifier=qualifier,
            priority=priority,
        )

        if instance is not None:
            # Pre-created singleton
            self._singletons[(interface, qualifier)] = instance
            self._add_registration(interface, implementation or type(instance), metadata)
        else:
            self._add_registration(interface, implementation, metadata)  # type: ignore[arg-type]

    def register_instance(
        self,
        interface: type[T],
        instance: T,
        qualifier: str | None = None,
    ) -> None:
        """Register a pre-created instance."""
        self.register(
            interface,
            instance=instance,
            scope=Scope.SINGLETON,
            qualifier=qualifier,
        )

    def register_factory(
        self,
        interface: type[T],
        factory: Callable[..., T],
        *,
        scope: Scope = Scope.PROTOTYPE,
        qualifier: str | None = None,
    ) -> None:
        """Register a factory function."""
        self.register(
            interface,
            implementation=factory,
            scope=scope,
            qualifier=qualifier,
        )

    def get(
        self,
        interface: type[T],
        qualifier: str | None = None,
    ) -> T:
        """
        Get an instance of the requested type.

        Args:
            interface: The type to resolve
            qualifier: Optional qualifier for disambiguation

        Returns:
            An instance of the requested type

        Raises:
            ProviderNotFoundError: If no provider matches
            AmbiguousProviderError: If multiple providers match without primary
            CircularDependencyError: If circular dependency detected
        """
        # Check singleton cache
        cache_key = (interface, qualifier)
        if cache_key in self._singletons:
            return self._singletons[cache_key]

        # Check request scope cache
        if cache_key in self._request_scope:
            return self._request_scope[cache_key]

        # Find registration
        registration = self._find_registration(interface, qualifier)
        if registration is None:
            raise ProviderNotFoundError(
                f"No provider found for {interface.__name__}"
                + (f" with qualifier '{qualifier}'" if qualifier else "")
            )

        # Check for circular dependency
        if interface in self._resolving:
            raise CircularDependencyError(
                f"Circular dependency detected for {interface.__name__}"
            )

        # Create instance
        self._resolving.add(interface)
        try:
            instance = self._create_instance(registration)
        finally:
            self._resolving.discard(interface)

        # Cache based on scope
        if registration.metadata.scope == Scope.SINGLETON:
            self._singletons[cache_key] = instance
        elif registration.metadata.scope == Scope.REQUEST:
            self._request_scope[cache_key] = instance

        return instance

    def get_all(self, interface: type[T]) -> list[T]:
        """Get all instances matching the interface."""
        registrations = self._registrations.get(interface, [])
        return [self._create_instance(reg) for reg in registrations]

    def get_optional(
        self,
        interface: type[T],
        qualifier: str | None = None,
    ) -> T | None:
        """Get an instance or None if not found."""
        try:
            return self.get(interface, qualifier)
        except ProviderNotFoundError:
            return None

    def create(self, cls: type[T]) -> T:
        """
        Create an instance with all @Inject fields resolved.

        Args:
            cls: The class to instantiate

        Returns:
            An instance with injected dependencies
        """
        # Get constructor parameters
        init_params: dict[str, Any] = {}
        sig = inspect.signature(cls.__init__)
        hints = get_type_hints(cls.__init__) if hasattr(cls.__init__, "__annotations__") else {}

        for name, param in sig.parameters.items():
            if name == "self":
                continue
            if name in hints:
                try:
                    init_params[name] = self.get(hints[name])
                except ProviderNotFoundError:
                    if param.default is inspect.Parameter.empty:
                        raise

        # Create instance
        instance = cls(**init_params)

        # Resolve @Inject fields
        self._inject_fields(instance)

        return instance

    def _inject_fields(self, instance: Any) -> None:
        """Inject all @Inject fields on an instance."""
        cls = type(instance)
        if cls in _inject_fields:
            for field_name in _inject_fields[cls]:
                # The descriptor will auto-resolve
                getattr(instance, field_name)

    def _find_registration(
        self,
        interface: type,
        qualifier: str | None,
    ) -> Registration | None:
        """Find the best matching registration."""
        registrations = self._registrations.get(interface, [])

        if not registrations:
            return None

        # Filter by qualifier if specified
        if qualifier:
            registrations = [r for r in registrations if r.metadata.qualifier == qualifier]
            if not registrations:
                return None

        if len(registrations) == 1:
            return registrations[0]

        # Find primary
        primary = [r for r in registrations if r.metadata.primary]
        if len(primary) == 1:
            return primary[0]

        if len(primary) > 1:
            raise AmbiguousProviderError(
                f"Multiple primary providers for {interface.__name__}"
            )

        # Return highest priority (already sorted)
        return registrations[0]

    def _create_instance(self, registration: Registration) -> Any:
        """Create an instance from a registration."""
        impl = registration.implementation

        if registration.instance is not None:
            return registration.instance

        if callable(impl) and not isinstance(impl, type):
            # Factory function
            return impl()

        # Class - create with injected constructor params
        return self.create(impl)

    @contextmanager
    def request_scope(self):
        """Context manager for request-scoped dependencies."""
        old_scope = self._request_scope.copy()
        self._request_scope.clear()
        try:
            yield self
        finally:
            self._request_scope = old_scope

    @asynccontextmanager
    async def async_request_scope(self):
        """Async context manager for request-scoped dependencies."""
        old_scope = self._request_scope.copy()
        self._request_scope.clear()
        try:
            yield self
        finally:
            self._request_scope = old_scope

    def clear(self) -> None:
        """Clear all registrations and cached instances."""
        self._registrations.clear()
        self._singletons.clear()
        self._request_scope.clear()

    def has(self, interface: type, qualifier: str | None = None) -> bool:
        """Check if a provider is registered."""
        return self._find_registration(interface, qualifier) is not None


# Global container access
_container: Container | None = None


def get_container() -> Container:
    """Get the global container instance."""
    global _container
    if _container is None:
        _container = Container()
    return _container


def set_container(container: Container) -> None:
    """Set the global container instance."""
    global _container
    _container = container


def reset_container() -> None:
    """Reset the global container."""
    global _container
    _container = None


# Convenience functions
def inject(interface: type[T], qualifier: str | None = None) -> T:
    """Inject a dependency from the global container."""
    return get_container().get(interface, qualifier)


def register(
    interface: type[T],
    implementation: type[T] | None = None,
    **kwargs: Any,
) -> None:
    """Register a dependency in the global container."""
    get_container().register(interface, implementation, **kwargs)


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
