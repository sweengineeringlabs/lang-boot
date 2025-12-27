"""
Serialization protocols - Type-safe serializer/deserializer interfaces.
"""

from typing import Protocol, TypeVar, Any, runtime_checkable

T = TypeVar("T")


@runtime_checkable
class Serializer(Protocol):
    """Protocol for serializing objects to bytes.
    
    Implementations should handle:
    - Primitive types (str, int, float, bool, None)
    - Collections (list, dict, set, tuple)
    - Dataclasses and Pydantic models
    - Custom types via encoders
    
    Example:
        class MySerializer:
            def serialize(self, obj: Any) -> bytes:
                return json.dumps(obj).encode()
            
            def serialize_to_string(self, obj: Any) -> str:
                return json.dumps(obj)
    """
    
    def serialize(self, obj: Any) -> bytes:
        """Serialize an object to bytes.
        
        Args:
            obj: Object to serialize. Should be JSON-serializable or
                 a Pydantic model.
                 
        Returns:
            Serialized bytes representation.
            
        Raises:
            SerializationError: If serialization fails.
        """
        ...
    
    def serialize_to_string(self, obj: Any) -> str:
        """Serialize an object to a string.
        
        Args:
            obj: Object to serialize.
            
        Returns:
            String representation (e.g., JSON string).
            
        Raises:
            SerializationError: If serialization fails.
        """
        ...


@runtime_checkable
class Deserializer(Protocol):
    """Protocol for deserializing bytes to objects.
    
    Implementations should handle type coercion and validation
    when a target type is specified.
    
    Example:
        class MyDeserializer:
            def deserialize(self, data: bytes, target_type: type[T]) -> T:
                parsed = json.loads(data)
                if hasattr(target_type, 'model_validate'):
                    return target_type.model_validate(parsed)
                return parsed
    """
    
    def deserialize(self, data: bytes, target_type: type[T] | None = None) -> T | Any:
        """Deserialize bytes to an object.
        
        Args:
            data: Bytes to deserialize.
            target_type: Optional type to deserialize into. If provided and
                         the type is a Pydantic model, validation is performed.
                         
        Returns:
            Deserialized object, optionally typed.
            
        Raises:
            DeserializationError: If deserialization fails.
        """
        ...
    
    def deserialize_from_string(
        self, data: str, target_type: type[T] | None = None
    ) -> T | Any:
        """Deserialize a string to an object.
        
        Args:
            data: String to deserialize.
            target_type: Optional type to deserialize into.
            
        Returns:
            Deserialized object.
            
        Raises:
            DeserializationError: If deserialization fails.
        """
        ...
