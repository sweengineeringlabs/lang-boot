"""
Base serializer - Common functionality for all serializers.
"""

from abc import ABC, abstractmethod
from typing import Any, TypeVar
import gzip

from dev.engineeringlabs.pyboot.serialization.api.protocols import Serializer, Deserializer
from dev.engineeringlabs.pyboot.serialization.api.types import Format, SerializationOptions

T = TypeVar("T")


class BaseSerializer(Serializer, Deserializer, ABC):
    """Base class for serializer implementations.
    
    Provides common functionality like compression and Pydantic integration.
    Subclasses implement format-specific serialization.
    """
    
    @property
    @abstractmethod
    def format(self) -> Format:
        """The format this serializer handles."""
        ...
    
    @abstractmethod
    def _serialize_impl(self, obj: Any, options: SerializationOptions) -> bytes:
        """Format-specific serialization implementation."""
        ...
    
    @abstractmethod
    def _deserialize_impl(self, data: bytes, options: SerializationOptions) -> Any:
        """Format-specific deserialization implementation."""
        ...
    
    def serialize(
        self,
        obj: Any,
        options: SerializationOptions | None = None,
    ) -> bytes:
        """Serialize an object to bytes with optional compression.
        
        Args:
            obj: Object to serialize.
            options: Serialization options.
            
        Returns:
            Serialized bytes, optionally compressed.
        """
        opts = options or SerializationOptions()
        
        # Convert Pydantic models to dict
        serializable = self._prepare_for_serialization(obj, opts)
        
        # Serialize using format-specific implementation
        data = self._serialize_impl(serializable, opts)
        
        # Apply compression if requested
        if opts.compress:
            data = gzip.compress(data, compresslevel=opts.compression_level)
        
        return data
    
    def deserialize(
        self,
        data: bytes,
        target_type: type[T] | None = None,
        options: SerializationOptions | None = None,
    ) -> T | Any:
        """Deserialize bytes to an object.
        
        Args:
            data: Bytes to deserialize.
            target_type: Optional type to coerce result into.
            options: Serialization options.
            
        Returns:
            Deserialized object.
        """
        opts = options or SerializationOptions()
        
        # Decompress if needed
        if opts.compress or self._is_gzip(data):
            data = gzip.decompress(data)
        
        # Deserialize using format-specific implementation
        result = self._deserialize_impl(data, opts)
        
        # Coerce to target type if specified
        if target_type is not None:
            result = self._coerce_to_type(result, target_type)
        
        return result
    
    def serialize_to_string(
        self,
        obj: Any,
        options: SerializationOptions | None = None,
    ) -> str:
        """Serialize to string (for text-based formats).
        
        Args:
            obj: Object to serialize.
            options: Serialization options.
            
        Returns:
            String representation.
        """
        opts = options or SerializationOptions()
        data = self.serialize(obj, opts)
        return data.decode(opts.encoding)
    
    def deserialize_from_string(
        self,
        data: str,
        target_type: type[T] | None = None,
        options: SerializationOptions | None = None,
    ) -> T | Any:
        """Deserialize from string.
        
        Args:
            data: String to deserialize.
            target_type: Optional target type.
            options: Serialization options.
            
        Returns:
            Deserialized object.
        """
        opts = options or SerializationOptions()
        return self.deserialize(
            data.encode(opts.encoding),
            target_type=target_type,
            options=opts,
        )
    
    def _prepare_for_serialization(
        self,
        obj: Any,
        options: SerializationOptions,
    ) -> Any:
        """Prepare object for serialization.
        
        Handles Pydantic models, dataclasses, and custom encoders.
        """
        # Handle Pydantic v2 models
        if hasattr(obj, "model_dump"):
            return obj.model_dump()
        
        # Handle Pydantic v1 models
        if hasattr(obj, "dict") and hasattr(obj, "__fields__"):
            return obj.dict()
        
        # Handle dataclasses
        if hasattr(obj, "__dataclass_fields__"):
            from dataclasses import asdict
            return asdict(obj)
        
        # Handle custom encoder
        if options.custom_encoder:
            return options.custom_encoder(obj)
        
        return obj
    
    def _coerce_to_type(self, data: Any, target_type: type[T]) -> T:
        """Coerce data to target type.
        
        Handles Pydantic models, dataclasses, and built-in types.
        """
        # Handle None
        if data is None:
            return None  # type: ignore
        
        # Already correct type
        if isinstance(data, target_type):
            return data
        
        # Handle Pydantic v2 models
        if hasattr(target_type, "model_validate"):
            return target_type.model_validate(data)
        
        # Handle Pydantic v1 models
        if hasattr(target_type, "parse_obj"):
            return target_type.parse_obj(data)
        
        # Handle dataclasses
        if hasattr(target_type, "__dataclass_fields__") and isinstance(data, dict):
            return target_type(**data)
        
        # Handle built-in types
        if target_type in (dict, list, str, int, float, bool):
            return target_type(data)
        
        # Return as-is if we can't coerce
        return data  # type: ignore
    
    @staticmethod
    def _is_gzip(data: bytes) -> bool:
        """Check if data is gzip compressed."""
        return len(data) >= 2 and data[0:2] == b"\x1f\x8b"
