"""
MessagePack Serializer - Efficient binary serialization.
"""

from typing import Any
from datetime import datetime, date, time
from decimal import Decimal
from uuid import UUID
from enum import Enum

from dev.engineeringlabs.pyboot.serialization.api.types import Format, SerializationOptions
from dev.engineeringlabs.pyboot.serialization.api.exceptions import (
    SerializationError,
    DeserializationError,
    UnsupportedFormatError,
)
from dev.engineeringlabs.pyboot.serialization.core.base import BaseSerializer

# Try to import msgpack
try:
    import msgpack
    HAS_MSGPACK = True
except ImportError:
    HAS_MSGPACK = False
    msgpack = None  # type: ignore


class MsgpackSerializer(BaseSerializer):
    """MessagePack serializer for efficient binary serialization.
    
    MessagePack is a binary format that is:
    - More compact than JSON (typically 50-80% smaller)
    - Faster to serialize/deserialize
    - Supports binary data natively
    
    Requires: pip install msgpack
    
    Example:
        serializer = MsgpackSerializer()
        
        # Compact binary output
        data = serializer.serialize({"users": [1, 2, 3], "count": 3})
        obj = serializer.deserialize(data, dict)
        
        # Best for:
        # - Network protocols
        # - Cache storage
        # - High-throughput applications
    """
    
    def __init__(self) -> None:
        if not HAS_MSGPACK:
            raise UnsupportedFormatError(
                "msgpack",
                reason="msgpack library not installed",
                install_hint="pip install msgpack",
            )
    
    @property
    def format(self) -> Format:
        return Format.MSGPACK
    
    def _serialize_impl(self, obj: Any, options: SerializationOptions) -> bytes:
        """Serialize to MessagePack format."""
        try:
            return msgpack.packb(
                obj,
                default=self._default_encoder,
                use_bin_type=True,
            )
        except (TypeError, ValueError) as e:
            raise SerializationError.from_exception(
                e,
                obj_type=type(obj),
                format_name="MessagePack",
            )
    
    def _deserialize_impl(self, data: bytes, options: SerializationOptions) -> Any:
        """Deserialize from MessagePack format."""
        try:
            return msgpack.unpackb(
                data,
                raw=False,
                strict_map_key=True,
            )
        except (msgpack.ExtraData, msgpack.FormatError, ValueError) as e:
            raise DeserializationError.from_exception(
                e,
                format_name="MessagePack",
                data=data,
            )
    
    def serialize_to_string(
        self,
        obj: Any,
        options: SerializationOptions | None = None,
    ) -> str:
        """MessagePack is binary - return base64 encoded string."""
        import base64
        data = self.serialize(obj, options)
        return base64.b64encode(data).decode("ascii")
    
    def deserialize_from_string(
        self,
        data: str,
        target_type: type | None = None,
        options: SerializationOptions | None = None,
    ) -> Any:
        """Deserialize from base64 encoded MessagePack."""
        import base64
        binary_data = base64.b64decode(data.encode("ascii"))
        return self.deserialize(binary_data, target_type, options)
    
    @staticmethod
    def _default_encoder(obj: Any) -> Any:
        """Default encoder for non-standard types."""
        # Datetime types
        if isinstance(obj, datetime):
            return {"__datetime__": obj.isoformat()}
        if isinstance(obj, date):
            return {"__date__": obj.isoformat()}
        if isinstance(obj, time):
            return {"__time__": obj.isoformat()}
        
        # Other common types
        if isinstance(obj, UUID):
            return {"__uuid__": str(obj)}
        if isinstance(obj, Decimal):
            return {"__decimal__": str(obj)}
        if isinstance(obj, Enum):
            return obj.value
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
        
        raise TypeError(f"Object of type {type(obj).__name__} is not serializable")


def is_msgpack_available() -> bool:
    """Check if msgpack is available."""
    return HAS_MSGPACK
