"""
YAML Serializer - Human-readable configuration format.
"""

from typing import Any
from datetime import datetime, date, time
from decimal import Decimal
from uuid import UUID
from enum import Enum
from pathlib import Path

from dev.engineeringlabs.pyboot.serialization.api.types import Format, SerializationOptions
from dev.engineeringlabs.pyboot.serialization.api.exceptions import (
    SerializationError,
    DeserializationError,
    UnsupportedFormatError,
)
from dev.engineeringlabs.pyboot.serialization.core.base import BaseSerializer

# Try to import PyYAML
try:
    import yaml
    from yaml import SafeLoader, SafeDumper
    HAS_YAML = True
except ImportError:
    HAS_YAML = False
    yaml = None  # type: ignore


class YamlSerializer(BaseSerializer):
    """YAML serializer for human-readable configuration files.
    
    YAML is ideal for:
    - Configuration files
    - Human-edited data
    - Documents with comments
    
    Requires: pip install pyyaml
    
    Example:
        serializer = YamlSerializer()
        
        config = {
            "server": {
                "host": "localhost",
                "port": 8080,
            },
            "features": ["auth", "cache"],
        }
        
        # Human-readable output
        yaml_str = serializer.serialize_to_string(config)
        # server:
        #   host: localhost
        #   port: 8080
        # features:
        #   - auth
        #   - cache
    """
    
    def __init__(self) -> None:
        if not HAS_YAML:
            raise UnsupportedFormatError(
                "yaml",
                reason="PyYAML library not installed",
                install_hint="pip install pyyaml",
            )
        self._setup_representers()
    
    @property
    def format(self) -> Format:
        return Format.YAML
    
    def _setup_representers(self) -> None:
        """Register custom type representers for YAML."""
        # Add representer for Path
        yaml.add_representer(
            Path,
            lambda dumper, data: dumper.represent_str(str(data)),
            Dumper=SafeDumper,
        )
        
        # Add representer for UUID
        yaml.add_representer(
            UUID,
            lambda dumper, data: dumper.represent_str(str(data)),
            Dumper=SafeDumper,
        )
        
        # Add representer for Decimal
        yaml.add_representer(
            Decimal,
            lambda dumper, data: dumper.represent_float(float(data)),
            Dumper=SafeDumper,
        )
    
    def _serialize_impl(self, obj: Any, options: SerializationOptions) -> bytes:
        """Serialize to YAML format."""
        try:
            # Prepare object for serialization
            prepared = self._prepare_yaml_object(obj)
            
            yaml_str = yaml.dump(
                prepared,
                Dumper=SafeDumper,
                default_flow_style=not options.pretty,
                indent=options.indent if options.pretty else None,
                sort_keys=options.sort_keys,
                allow_unicode=True,
            )
            return yaml_str.encode(options.encoding)
        except yaml.YAMLError as e:
            raise SerializationError.from_exception(
                e,
                obj_type=type(obj),
                format_name="YAML",
            )
    
    def _deserialize_impl(self, data: bytes, options: SerializationOptions) -> Any:
        """Deserialize from YAML format."""
        try:
            return yaml.safe_load(data.decode(options.encoding))
        except yaml.YAMLError as e:
            raise DeserializationError.from_exception(
                e,
                format_name="YAML",
                data=data,
            )
    
    def _prepare_yaml_object(self, obj: Any) -> Any:
        """Recursively prepare object for YAML serialization."""
        if obj is None:
            return None
        
        if isinstance(obj, (str, int, float, bool)):
            return obj
        
        if isinstance(obj, dict):
            return {k: self._prepare_yaml_object(v) for k, v in obj.items()}
        
        if isinstance(obj, (list, tuple)):
            return [self._prepare_yaml_object(item) for item in obj]
        
        if isinstance(obj, set):
            return [self._prepare_yaml_object(item) for item in sorted(obj, key=str)]
        
        if isinstance(obj, datetime):
            return obj.isoformat()
        
        if isinstance(obj, (date, time)):
            return obj.isoformat()
        
        if isinstance(obj, UUID):
            return str(obj)
        
        if isinstance(obj, Decimal):
            return float(obj)
        
        if isinstance(obj, Enum):
            return obj.value
        
        if isinstance(obj, Path):
            return str(obj)
        
        if isinstance(obj, bytes):
            return obj.decode("utf-8", errors="replace")
        
        # Pydantic models
        if hasattr(obj, "model_dump"):
            return self._prepare_yaml_object(obj.model_dump())
        
        if hasattr(obj, "dict") and hasattr(obj, "__fields__"):
            return self._prepare_yaml_object(obj.dict())
        
        # Dataclasses
        if hasattr(obj, "__dataclass_fields__"):
            from dataclasses import asdict
            return self._prepare_yaml_object(asdict(obj))
        
        # Last resort - convert to string
        return str(obj)


def is_yaml_available() -> bool:
    """Check if PyYAML is available."""
    return HAS_YAML
