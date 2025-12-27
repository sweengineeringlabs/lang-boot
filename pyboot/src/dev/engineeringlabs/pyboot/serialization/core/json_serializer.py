"""
JSON Serializer - Fast JSON serialization with orjson/standard library fallback.
"""

from typing import Any
from datetime import datetime, date, time
from decimal import Decimal
from uuid import UUID
from enum import Enum
import json

from dev.engineeringlabs.pyboot.serialization.api.types import Format, SerializationOptions
from dev.engineeringlabs.pyboot.serialization.api.exceptions import SerializationError, DeserializationError
from dev.engineeringlabs.pyboot.serialization.core.base import BaseSerializer

# Try to use orjson for performance, fall back to standard json
try:
    import orjson
    HAS_ORJSON = True
except ImportError:
    HAS_ORJSON = False


class JsonSerializer(BaseSerializer):
    """JSON serializer with optional orjson acceleration.
    
    Features:
    - Automatic orjson usage when available (5-10x faster)
    - Pretty printing support
    - Custom type encoding (datetime, UUID, Decimal, Enum)
    - Pydantic model integration
    
    Example:
        serializer = JsonSerializer()
        
        # Basic usage
        data = serializer.serialize({"name": "Alice", "age": 30})
        obj = serializer.deserialize(data, dict)
        
        # With options
        pretty_data = serializer.serialize(
            obj,
            SerializationOptions(pretty=True, indent=4)
        )
    """
    
    @property
    def format(self) -> Format:
        return Format.JSON
    
    def _serialize_impl(self, obj: Any, options: SerializationOptions) -> bytes:
        """Serialize using orjson (if available) or standard json."""
        try:
            if HAS_ORJSON:
                return self._serialize_orjson(obj, options)
            return self._serialize_stdlib(obj, options)
        except (TypeError, ValueError) as e:
            raise SerializationError.from_exception(
                e,
                obj_type=type(obj),
                format_name="JSON",
            )
    
    def _deserialize_impl(self, data: bytes, options: SerializationOptions) -> Any:
        """Deserialize using orjson (if available) or standard json."""
        try:
            if HAS_ORJSON:
                return orjson.loads(data)
            return json.loads(data.decode(options.encoding))
        except (json.JSONDecodeError, ValueError) as e:
            raise DeserializationError.from_exception(
                e,
                format_name="JSON",
                data=data,
            )
    
    def _serialize_orjson(self, obj: Any, options: SerializationOptions) -> bytes:
        """Serialize using orjson with options."""
        opts = orjson.OPT_SERIALIZE_NUMPY
        
        if options.pretty:
            opts |= orjson.OPT_INDENT_2
        if options.sort_keys:
            opts |= orjson.OPT_SORT_KEYS
        
        return orjson.dumps(obj, option=opts, default=self._default_encoder)
    
    def _serialize_stdlib(self, obj: Any, options: SerializationOptions) -> bytes:
        """Serialize using standard library json."""
        kwargs: dict[str, Any] = {
            "default": self._default_encoder,
            "ensure_ascii": False,
        }
        
        if options.pretty:
            kwargs["indent"] = options.indent
        if options.sort_keys:
            kwargs["sort_keys"] = True
        
        return json.dumps(obj, **kwargs).encode(options.encoding)
    
    @staticmethod
    def _default_encoder(obj: Any) -> Any:
        """Default encoder for non-standard types."""
        # Datetime types
        if isinstance(obj, datetime):
            return obj.isoformat()
        if isinstance(obj, date):
            return obj.isoformat()
        if isinstance(obj, time):
            return obj.isoformat()
        
        # Other common types
        if isinstance(obj, UUID):
            return str(obj)
        if isinstance(obj, Decimal):
            return float(obj)
        if isinstance(obj, Enum):
            return obj.value
        if isinstance(obj, bytes):
            return obj.decode("utf-8", errors="replace")
        if isinstance(obj, set):
            return list(obj)
        if isinstance(obj, frozenset):
            return list(obj)
        
        # Pydantic models
        if hasattr(obj, "model_dump"):
            return obj.model_dump()
        if hasattr(obj, "dict") and hasattr(obj, "__fields__"):
            return obj.dict()
        
        # Dataclasses
        if hasattr(obj, "__dataclass_fields__"):
            from dataclasses import asdict
            return asdict(obj)
        
        raise TypeError(f"Object of type {type(obj).__name__} is not JSON serializable")


# Create default instance
_default_serializer = JsonSerializer()


def json_serialize(obj: Any, **kwargs: Any) -> bytes:
    """Convenience function for JSON serialization."""
    options = SerializationOptions(**kwargs) if kwargs else None
    return _default_serializer.serialize(obj, options)


def json_deserialize(data: bytes, target_type: type | None = None, **kwargs: Any) -> Any:
    """Convenience function for JSON deserialization."""
    options = SerializationOptions(**kwargs) if kwargs else None
    return _default_serializer.deserialize(data, target_type, options)
