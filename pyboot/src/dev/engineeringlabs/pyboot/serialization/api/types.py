"""
Serialization types - Format enum and options.
"""

from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Callable


class Format(str, Enum):
    """Supported serialization formats.
    
    Each format has different characteristics:
    - JSON: Human-readable, widely supported, text-based
    - MSGPACK: Binary, compact, fast
    - YAML: Human-readable, supports comments, good for config
    - PICKLE: Python-specific, supports any Python object
    """
    
    JSON = "json"
    """JSON format - Human-readable, universal."""
    
    MSGPACK = "msgpack"
    """MessagePack format - Binary, efficient, compact."""
    
    YAML = "yaml"
    """YAML format - Human-readable, supports comments."""
    
    PICKLE = "pickle"
    """Python Pickle - Supports any Python object, not portable."""
    
    def __str__(self) -> str:
        return self.value


@dataclass(frozen=True)
class SerializationOptions:
    """Options for serialization behavior.
    
    Attributes:
        format: Output format (JSON, MSGPACK, YAML, PICKLE).
        compress: Whether to compress output using gzip.
        compression_level: Gzip compression level (1-9).
        pretty: Pretty-print output (JSON, YAML only).
        indent: Indentation for pretty printing.
        sort_keys: Sort dictionary keys in output.
        encoding: String encoding for text formats.
        custom_encoder: Custom encoder function for non-standard types.
        custom_decoder: Custom decoder function for non-standard types.
    
    Example:
        options = SerializationOptions(
            format=Format.JSON,
            compress=True,
            pretty=True,
            indent=2,
        )
        data = serialize(obj, options=options)
    """
    
    format: Format = Format.JSON
    """Serialization format to use."""
    
    compress: bool = False
    """Enable gzip compression."""
    
    compression_level: int = 6
    """Gzip compression level (1=fast, 9=best)."""
    
    pretty: bool = False
    """Pretty-print output (adds whitespace)."""
    
    indent: int = 2
    """Indentation spaces for pretty printing."""
    
    sort_keys: bool = False
    """Sort dictionary keys alphabetically."""
    
    encoding: str = "utf-8"
    """String encoding for text-based formats."""
    
    custom_encoder: Callable[[Any], Any] | None = None
    """Custom encoder for non-serializable types."""
    
    custom_decoder: Callable[[dict[str, Any]], Any] | None = None
    """Custom decoder for special types."""
    
    strict: bool = True
    """Raise errors on serialization failures vs. skip."""
    
    include_type_hints: bool = False
    """Include __type__ field for polymorphic deserialization."""
    
    def with_format(self, fmt: Format) -> "SerializationOptions":
        """Create new options with different format."""
        return SerializationOptions(
            format=fmt,
            compress=self.compress,
            compression_level=self.compression_level,
            pretty=self.pretty,
            indent=self.indent,
            sort_keys=self.sort_keys,
            encoding=self.encoding,
            custom_encoder=self.custom_encoder,
            custom_decoder=self.custom_decoder,
            strict=self.strict,
            include_type_hints=self.include_type_hints,
        )
    
    def with_compression(self, enabled: bool = True, level: int = 6) -> "SerializationOptions":
        """Create new options with compression settings."""
        return SerializationOptions(
            format=self.format,
            compress=enabled,
            compression_level=level,
            pretty=self.pretty,
            indent=self.indent,
            sort_keys=self.sort_keys,
            encoding=self.encoding,
            custom_encoder=self.custom_encoder,
            custom_decoder=self.custom_decoder,
            strict=self.strict,
            include_type_hints=self.include_type_hints,
        )


# Predefined option sets
DEFAULT_OPTIONS = SerializationOptions()
"""Default options: JSON, no compression, no pretty print."""

PRETTY_JSON = SerializationOptions(format=Format.JSON, pretty=True, indent=2)
"""Pretty JSON with 2-space indentation."""

COMPACT_BINARY = SerializationOptions(format=Format.MSGPACK, compress=True)
"""Compact MessagePack with compression."""

CONFIG_YAML = SerializationOptions(format=Format.YAML, pretty=True)
"""YAML for configuration files."""
