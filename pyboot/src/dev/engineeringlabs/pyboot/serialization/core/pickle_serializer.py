"""
Pickle Serializer - Python-native object serialization.
"""

from typing import Any
import pickle

from dev.engineeringlabs.pyboot.serialization.api.types import Format, SerializationOptions
from dev.engineeringlabs.pyboot.serialization.api.exceptions import SerializationError, DeserializationError
from dev.engineeringlabs.pyboot.serialization.core.base import BaseSerializer


class PickleSerializer(BaseSerializer):
    """Python Pickle serializer for native object serialization.
    
    Pickle can serialize almost any Python object, including:
    - Functions and lambdas (with limitations)
    - Complex nested objects
    - Custom classes
    
    WARNING: Pickle is NOT secure for untrusted data. Only use with
    data from trusted sources.
    
    Example:
        serializer = PickleSerializer()
        
        # Serialize complex objects
        class MyClass:
            def __init__(self, x: int):
                self.x = x
        
        obj = MyClass(42)
        data = serializer.serialize(obj)
        restored = serializer.deserialize(data, MyClass)
        assert restored.x == 42
    """
    
    @property
    def format(self) -> Format:
        return Format.PICKLE
    
    def _serialize_impl(self, obj: Any, options: SerializationOptions) -> bytes:
        """Serialize using pickle."""
        try:
            return pickle.dumps(obj, protocol=pickle.HIGHEST_PROTOCOL)
        except (pickle.PicklingError, TypeError, AttributeError) as e:
            raise SerializationError.from_exception(
                e,
                obj_type=type(obj),
                format_name="Pickle",
            )
    
    def _deserialize_impl(self, data: bytes, options: SerializationOptions) -> Any:
        """Deserialize using pickle.
        
        WARNING: This is inherently insecure for untrusted data.
        """
        try:
            return pickle.loads(data)
        except (pickle.UnpicklingError, AttributeError, ModuleNotFoundError) as e:
            raise DeserializationError.from_exception(
                e,
                format_name="Pickle",
                data=data,
            )
    
    def serialize_to_string(
        self,
        obj: Any,
        options: SerializationOptions | None = None,
    ) -> str:
        """Pickle is binary - return base64 encoded string."""
        import base64
        data = self.serialize(obj, options)
        return base64.b64encode(data).decode("ascii")
    
    def deserialize_from_string(
        self,
        data: str,
        target_type: type | None = None,
        options: SerializationOptions | None = None,
    ) -> Any:
        """Deserialize from base64 encoded pickle."""
        import base64
        binary_data = base64.b64decode(data.encode("ascii"))
        return self.deserialize(binary_data, target_type, options)
    
    def deserialize(
        self,
        data: bytes,
        target_type: type | None = None,
        options: SerializationOptions | None = None,
    ) -> Any:
        """Deserialize pickle data.
        
        Note: target_type is ignored for pickle since it preserves
        the original type. It's provided for API consistency.
        """
        opts = options or SerializationOptions()
        
        # Decompress if needed
        if opts.compress or self._is_gzip(data):
            import gzip
            data = gzip.decompress(data)
        
        result = self._deserialize_impl(data, opts)
        
        # Validate type if specified
        if target_type is not None and not isinstance(result, target_type):
            raise DeserializationError(
                f"Expected type {target_type.__name__}, got {type(result).__name__}",
                target_type=target_type,
                format_name="Pickle",
            )
        
        return result
