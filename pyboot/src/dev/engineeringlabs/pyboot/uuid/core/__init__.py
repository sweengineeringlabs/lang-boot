"""UUID Core - UUID generation implementations."""

import uuid as _uuid
import time
import random
from typing import Any


def uuid4() -> str:
    """Generate a random UUID v4."""
    return str(_uuid.uuid4())


def uuid7() -> str:
    """Generate a time-sorted UUID v7 (draft RFC).
    
    UUID v7 includes a timestamp for time-sorted ordering.
    """
    # Get timestamp in milliseconds
    timestamp_ms = int(time.time() * 1000)
    
    # 48 bits of timestamp
    time_high = (timestamp_ms >> 16) & 0xFFFFFFFF
    time_low = timestamp_ms & 0xFFFF
    
    # Random bits for the rest
    rand_a = random.getrandbits(12)
    rand_b = random.getrandbits(62)
    
    # Construct UUID v7
    # Format: tttttttt-tttt-7xxx-yxxx-xxxxxxxxxxxx
    # where t = timestamp, 7 = version, y = variant (8, 9, a, or b)
    uuid_int = (
        (time_high << 96) |
        (time_low << 80) |
        (0x7 << 76) |  # version 7
        (rand_a << 64) |
        (0b10 << 62) |  # variant
        rand_b
    )
    
    hex_str = f"{uuid_int:032x}"
    return f"{hex_str[:8]}-{hex_str[8:12]}-{hex_str[12:16]}-{hex_str[16:20]}-{hex_str[20:]}"


def ulid() -> str:
    """Generate a ULID (Universally Unique Lexicographically Sortable Identifier).
    
    ULIDs are 128-bit identifiers that are lexicographically sortable.
    """
    # 48 bits of timestamp (milliseconds)
    timestamp_ms = int(time.time() * 1000)
    
    # 80 bits of randomness
    random_bits = random.getrandbits(80)
    
    # Crockford's Base32 encoding
    alphabet = "0123456789ABCDEFGHJKMNPQRSTVWXYZ"
    
    # Encode timestamp (10 characters)
    time_chars = []
    t = timestamp_ms
    for _ in range(10):
        time_chars.append(alphabet[t & 0x1F])
        t >>= 5
    time_str = "".join(reversed(time_chars))
    
    # Encode random (16 characters)
    rand_chars = []
    r = random_bits
    for _ in range(16):
        rand_chars.append(alphabet[r & 0x1F])
        r >>= 5
    rand_str = "".join(reversed(rand_chars))
    
    return time_str + rand_str


def is_valid_uuid(value: str) -> bool:
    """Check if a string is a valid UUID."""
    try:
        _uuid.UUID(value)
        return True
    except ValueError:
        return False


def parse_uuid(value: str) -> _uuid.UUID:
    """Parse a UUID string."""
    return _uuid.UUID(value)


__all__ = [
    "uuid4",
    "uuid7",
    "ulid",
    "is_valid_uuid",
    "parse_uuid",
]
