"""
Serialization API - Public interfaces and types.
"""

from dev.engineeringlabs.pyboot.serialization.api.protocols import (
    Serializer,
    Deserializer,
)

from dev.engineeringlabs.pyboot.serialization.api.types import (
    Format,
    SerializationOptions,
)

from dev.engineeringlabs.pyboot.serialization.api.exceptions import (
    SerializationError,
    DeserializationError,
    UnsupportedFormatError,
)

__all__ = [
    # Protocols
    "Serializer",
    "Deserializer",
    # Types
    "Format",
    "SerializationOptions",
    # Exceptions
    "SerializationError",
    "DeserializationError",
    "UnsupportedFormatError",
]
