"""
Serializer Registry - Central registry for format handlers.
"""

from typing import Any

from dev.engineeringlabs.pyboot.serialization.api.types import Format
from dev.engineeringlabs.pyboot.serialization.api.protocols import Serializer, Deserializer
from dev.engineeringlabs.pyboot.serialization.api.exceptions import UnsupportedFormatError


class _SerializerRegistry:
    """Internal registry for serializer implementations.
    
    Manages registration and lookup of serializers by format.
    Lazy-loads serializers to avoid import errors for optional dependencies.
    """
    
    def __init__(self) -> None:
        self._serializers: dict[Format, Serializer] = {}
        self._loaded: set[Format] = set()
    
    def register(self, format: Format, serializer: Serializer) -> None:
        """Register a serializer for a format.
        
        Args:
            format: The format to register.
            serializer: The serializer instance.
        """
        self._serializers[format] = serializer
        self._loaded.add(format)
    
    def get(self, format: Format) -> Serializer:
        """Get a serializer for the specified format.
        
        Args:
            format: The format to get a serializer for.
            
        Returns:
            Serializer instance.
            
        Raises:
            UnsupportedFormatError: If format is not supported.
        """
        # Return cached serializer if available
        if format in self._serializers:
            return self._serializers[format]
        
        # Try to lazy-load the serializer
        serializer = self._lazy_load(format)
        if serializer:
            self._serializers[format] = serializer
            self._loaded.add(format)
            return serializer
        
        raise UnsupportedFormatError(
            format.value,
            reason=f"No serializer registered for format: {format}",
        )
    
    def _lazy_load(self, format: Format) -> Serializer | None:
        """Lazy-load a serializer for the format."""
        if format == Format.JSON:
            from dev.engineeringlabs.pyboot.serialization.core.json_serializer import JsonSerializer
            return JsonSerializer()
        
        if format == Format.MSGPACK:
            try:
                from dev.engineeringlabs.pyboot.serialization.core.msgpack_serializer import MsgpackSerializer
                return MsgpackSerializer()
            except UnsupportedFormatError:
                return None
        
        if format == Format.YAML:
            try:
                from dev.engineeringlabs.pyboot.serialization.core.yaml_serializer import YamlSerializer
                return YamlSerializer()
            except UnsupportedFormatError:
                return None
        
        if format == Format.PICKLE:
            from dev.engineeringlabs.pyboot.serialization.core.pickle_serializer import PickleSerializer
            return PickleSerializer()
        
        return None
    
    def is_available(self, format: Format) -> bool:
        """Check if a format is available.
        
        Args:
            format: The format to check.
            
        Returns:
            True if the format is supported.
        """
        if format in self._loaded:
            return True
        
        # Try to load it
        try:
            self.get(format)
            return True
        except UnsupportedFormatError:
            return False
    
    def available_formats(self) -> list[Format]:
        """Get list of available formats.
        
        Returns:
            List of supported formats.
        """
        available = []
        for format in Format:
            if self.is_available(format):
                available.append(format)
        return available
    
    def clear(self) -> None:
        """Clear all registered serializers (for testing)."""
        self._serializers.clear()
        self._loaded.clear()


# Global registry instance
_registry = _SerializerRegistry()


def register_serializer(format: Format, serializer: Serializer) -> None:
    """Register a custom serializer for a format.
    
    Use this to override the default serializer for a format
    or to register a serializer for a custom format.
    
    Example:
        class MyJsonSerializer(Serializer):
            ...
        
        register_serializer(Format.JSON, MyJsonSerializer())
    """
    _registry.register(format, serializer)


def get_serializer(format: Format) -> Serializer:
    """Get the serializer for a format.
    
    Args:
        format: The format to get a serializer for.
        
    Returns:
        Serializer instance.
        
    Raises:
        UnsupportedFormatError: If format is not supported.
    """
    return _registry.get(format)


def is_format_available(format: Format) -> bool:
    """Check if a serialization format is available.
    
    Args:
        format: The format to check.
        
    Returns:
        True if available (dependencies installed).
    """
    return _registry.is_available(format)


def available_formats() -> list[Format]:
    """Get list of available serialization formats.
    
    Returns:
        List of formats that can be used.
    """
    return _registry.available_formats()
