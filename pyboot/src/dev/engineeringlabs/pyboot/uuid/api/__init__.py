"""UUID API - UUID types."""

from enum import Enum, auto


class UUIDVersion(Enum):
    """UUID version."""
    V4 = 4  # Random
    V7 = 7  # Time-sorted (draft RFC)


__all__ = [
    "UUIDVersion",
]
