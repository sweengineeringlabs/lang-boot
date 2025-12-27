"""Session utilities."""

import secrets
import hashlib
import time


def generate_session_id(length: int = 32) -> str:
    """Generate cryptographically secure session ID."""
    return secrets.token_urlsafe(length)


def generate_session_id_with_hash() -> str:
    """Generate session ID with timestamp hash."""
    random_part = secrets.token_hex(16)
    time_part = str(time.time_ns())
    combined = f"{random_part}{time_part}"
    return hashlib.sha256(combined.encode()).hexdigest()
