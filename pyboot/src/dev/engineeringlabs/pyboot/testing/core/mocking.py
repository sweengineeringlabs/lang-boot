"""
Mocking utilities - Mock builders and helpers.
"""

from typing import Any, TypeVar, Callable, Generic
from unittest.mock import Mock, AsyncMock, MagicMock, patch

T = TypeVar("T")


class MockBuilder(Generic[T]):
    """Fluent builder for creating mocks.
    
    Example:
        mock_repo = (MockBuilder(UserRepository)
            .with_method("find_by_id", returns=user)
            .with_method("save", side_effect=save_user)
            .with_property("count", 10)
            .build())
    """
    
    def __init__(self, spec: type[T] | None = None) -> None:
        self._spec = spec
        self._methods: dict[str, dict[str, Any]] = {}
        self._properties: dict[str, Any] = {}
        self._attributes: dict[str, Any] = {}
    
    def with_method(
        self,
        name: str,
        *,
        returns: Any = None,
        side_effect: Any = None,
        is_async: bool = False,
    ) -> "MockBuilder[T]":
        """Configure a method mock."""
        self._methods[name] = {
            "return_value": returns,
            "side_effect": side_effect,
            "is_async": is_async,
        }
        return self
    
    def with_property(self, name: str, value: Any) -> "MockBuilder[T]":
        """Configure a property value."""
        self._properties[name] = value
        return self
    
    def with_attribute(self, name: str, value: Any) -> "MockBuilder[T]":
        """Configure an attribute value."""
        self._attributes[name] = value
        return self
    
    def build(self) -> T:
        """Build the mock object."""
        mock = MagicMock(spec=self._spec) if self._spec else MagicMock()
        
        # Configure methods
        for name, config in self._methods.items():
            method_mock = AsyncMock() if config["is_async"] else Mock()
            if config["side_effect"]:
                method_mock.side_effect = config["side_effect"]
            else:
                method_mock.return_value = config["return_value"]
            setattr(mock, name, method_mock)
        
        # Configure properties
        for name, value in self._properties.items():
            setattr(type(mock), name, property(lambda s, v=value: v))
        
        # Configure attributes
        for name, value in self._attributes.items():
            setattr(mock, name, value)
        
        return mock  # type: ignore


def mock_provider(
    interface: type[T],
    **method_returns: Any,
) -> T:
    """Quick mock creation for provider interfaces.
    
    Example:
        mock_cache = mock_provider(
            CacheProvider,
            get=cached_value,
            set=None,
            delete=True,
        )
    """
    builder = MockBuilder(interface)
    for method_name, return_value in method_returns.items():
        builder.with_method(method_name, returns=return_value)
    return builder.build()


def mock_async_provider(
    interface: type[T],
    **method_returns: Any,
) -> T:
    """Quick mock for async provider interfaces."""
    builder = MockBuilder(interface)
    for method_name, return_value in method_returns.items():
        builder.with_method(method_name, returns=return_value, is_async=True)
    return builder.build()


def patch_provider(target: str, **method_returns: Any):
    """Patch a provider with a mock. Use as decorator or context manager."""
    mock = MagicMock()
    for method_name, return_value in method_returns.items():
        setattr(mock, method_name, Mock(return_value=return_value))
    return patch(target, mock)
