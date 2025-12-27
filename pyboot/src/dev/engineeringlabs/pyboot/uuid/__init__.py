"""
UUID Module - UUID generation utilities.

Provides UUID generation:
- UUID v4 (random)
- UUID v7 (time-sorted)
- ULID generation
"""

from dev.engineeringlabs.pyboot.uuid.api import (
    UUIDVersion,
)

from dev.engineeringlabs.pyboot.uuid.core import (
    uuid4,
    uuid7,
    ulid,
    is_valid_uuid,
    parse_uuid,
)

__all__ = [
    # API
    "UUIDVersion",
    # Core
    "uuid4",
    "uuid7",
    "ulid",
    "is_valid_uuid",
    "parse_uuid",
]
