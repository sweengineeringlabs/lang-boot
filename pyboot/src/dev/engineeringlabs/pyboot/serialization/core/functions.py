"""
Serialization functions - High-level convenience functions.
"""

from typing import Any, TypeVar, overload

from dev.engineeringlabs.pyboot.serialization.api.types import Format, SerializationOptions
from dev.engineeringlabs.pyboot.serialization.core.registry import get_serializer

T = TypeVar("T")


def serialize(
    obj: Any,
    *,
    format: Format = Format.JSON,
    compress: bool = False,
    pretty: bool = False,
    options: SerializationOptions | None = None,
) -> bytes:
    """Serialize an object to bytes.
    
    Args:
        obj: Object to serialize. Can be:
            - Primitive types (str, int, float, bool, None)
            - Collections (list, dict, set, tuple)
            - Pydantic models
            - Dataclasses
        format: Serialization format (JSON, MSGPACK, YAML, PICKLE).
        compress: Enable gzip compression.
        pretty: Pretty-print output (JSON, YAML only).
        options: Full serialization options (overrides other params).
        
    Returns:
        Serialized bytes.
        
    Example:
        # Basic JSON
        data = serialize({"name": "Alice"})
        
        # Compressed MessagePack
        data = serialize(large_obj, format=Format.MSGPACK, compress=True)
        
        # Pretty JSON
        data = serialize(config, pretty=True)
    """
    if options is None:
        options = SerializationOptions(
            format=format,
            compress=compress,
            pretty=pretty,
        )
    
    serializer = get_serializer(options.format)
    return serializer.serialize(obj, options)


@overload
def deserialize(data: bytes, target_type: type[T], **kwargs: Any) -> T: ...

@overload
def deserialize(data: bytes, target_type: None = None, **kwargs: Any) -> Any: ...

def deserialize(
    data: bytes,
    target_type: type[T] | None = None,
    *,
    format: Format = Format.JSON,
    options: SerializationOptions | None = None,
) -> T | Any:
    """Deserialize bytes to an object.
    
    Args:
        data: Bytes to deserialize.
        target_type: Optional type to deserialize into. Supports:
            - dict, list, str, int, float, bool
            - Pydantic models (v1 and v2)
            - Dataclasses
        format: Serialization format used.
        options: Full deserialization options.
        
    Returns:
        Deserialized object, optionally typed.
        
    Example:
        # Basic
        obj = deserialize(data)
        
        # With type
        user = deserialize(data, User)
        
        # MessagePack format
        obj = deserialize(data, format=Format.MSGPACK)
    """
    if options is None:
        options = SerializationOptions(format=format)
    
    serializer = get_serializer(options.format)
    return serializer.deserialize(data, target_type, options)


def serialize_to_string(
    obj: Any,
    *,
    format: Format = Format.JSON,
    pretty: bool = False,
    options: SerializationOptions | None = None,
) -> str:
    """Serialize an object to a string.
    
    For binary formats (MSGPACK, PICKLE), returns base64-encoded string.
    
    Args:
        obj: Object to serialize.
        format: Serialization format.
        pretty: Pretty-print output.
        options: Full serialization options.
        
    Returns:
        String representation.
        
    Example:
        json_str = serialize_to_string({"key": "value"})
        yaml_str = serialize_to_string(config, format=Format.YAML, pretty=True)
    """
    if options is None:
        options = SerializationOptions(format=format, pretty=pretty)
    
    serializer = get_serializer(options.format)
    return serializer.serialize_to_string(obj, options)


@overload
def deserialize_from_string(
    data: str, target_type: type[T], **kwargs: Any
) -> T: ...

@overload
def deserialize_from_string(
    data: str, target_type: None = None, **kwargs: Any
) -> Any: ...

def deserialize_from_string(
    data: str,
    target_type: type[T] | None = None,
    *,
    format: Format = Format.JSON,
    options: SerializationOptions | None = None,
) -> T | Any:
    """Deserialize a string to an object.
    
    For binary formats, expects base64-encoded string.
    
    Args:
        data: String to deserialize.
        target_type: Optional type to deserialize into.
        format: Serialization format.
        options: Full deserialization options.
        
    Returns:
        Deserialized object.
        
    Example:
        obj = deserialize_from_string('{"key": "value"}')
        config = deserialize_from_string(yaml_str, Config, format=Format.YAML)
    """
    if options is None:
        options = SerializationOptions(format=format)
    
    serializer = get_serializer(options.format)
    return serializer.deserialize_from_string(data, target_type, options)
