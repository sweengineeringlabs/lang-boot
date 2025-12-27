"""
Serialization Module - Unified data serialization and deserialization.

This module provides:
- Multiple format support (JSON, MessagePack, YAML, Pickle)
- Type-safe serialization with Pydantic integration
- Compression support
- Custom serializer registration

Example:
    from dev.engineeringlabs.pyboot.serialization import serialize, deserialize, Format
    
    # JSON serialization (default)
    data = {"name": "Alice", "age": 30}
    json_bytes = serialize(data)
    restored = deserialize(json_bytes, dict)
    
    # MessagePack (binary, efficient)
    msgpack_bytes = serialize(data, format=Format.MSGPACK)
    
    # With compression
    compressed = serialize(large_data, compress=True)
    
    # Pydantic models
    from pydantic import BaseModel
    
    class User(BaseModel):
        name: str
        age: int
    
    user = User(name="Bob", age=25)
    serialized = serialize(user)
    restored_user = deserialize(serialized, User)
"""

from dev.engineeringlabs.pyboot.serialization.api import (
    # Protocols
    Serializer,
    Deserializer,
    # Types
    Format,
    SerializationOptions,
    # Exceptions
    SerializationError,
    DeserializationError,
    UnsupportedFormatError,
)

from dev.engineeringlabs.pyboot.serialization.core import (
    # Functions
    serialize,
    deserialize,
    serialize_to_string,
    deserialize_from_string,
    # Registry
    register_serializer,
    get_serializer,
    # Serializers
    JsonSerializer,
    MsgpackSerializer,
    YamlSerializer,
    PickleSerializer,
)

__all__ = [
    # API - Protocols
    "Serializer",
    "Deserializer",
    # API - Types
    "Format",
    "SerializationOptions",
    # API - Exceptions
    "SerializationError",
    "DeserializationError",
    "UnsupportedFormatError",
    # Core - Functions
    "serialize",
    "deserialize",
    "serialize_to_string",
    "deserialize_from_string",
    # Core - Registry
    "register_serializer",
    "get_serializer",
    # Core - Serializers
    "JsonSerializer",
    "MsgpackSerializer",
    "YamlSerializer",
    "PickleSerializer",
]
