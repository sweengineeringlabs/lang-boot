"""
Serialization Core - Implementations and registry.
"""

from dev.engineeringlabs.pyboot.serialization.core.functions import (
    serialize,
    deserialize,
    serialize_to_string,
    deserialize_from_string,
)

from dev.engineeringlabs.pyboot.serialization.core.registry import (
    register_serializer,
    get_serializer,
)

from dev.engineeringlabs.pyboot.serialization.core.json_serializer import JsonSerializer
from dev.engineeringlabs.pyboot.serialization.core.msgpack_serializer import MsgpackSerializer
from dev.engineeringlabs.pyboot.serialization.core.yaml_serializer import YamlSerializer
from dev.engineeringlabs.pyboot.serialization.core.pickle_serializer import PickleSerializer

__all__ = [
    # Functions
    "serialize",
    "deserialize",
    "serialize_to_string",
    "deserialize_from_string",
    # Registry
    "register_serializer",
    "get_serializer",
    # Serializers
    "JsonSerializer",
    "MsgpackSerializer",
    "YamlSerializer",
    "PickleSerializer",
]
