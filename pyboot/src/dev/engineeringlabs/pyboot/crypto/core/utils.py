"""
Crypto utilities - Random generation and constant-time comparison.
"""

import os
import secrets
import string
import hmac


def secure_random_bytes(length: int) -> bytes:
    """Generate cryptographically secure random bytes.
    
    Args:
        length: Number of bytes.
        
    Returns:
        Random bytes.
    """
    return os.urandom(length)


def secure_random_string(
    length: int = 32,
    alphabet: str | None = None,
) -> str:
    """Generate a cryptographically secure random string.
    
    Args:
        length: String length.
        alphabet: Characters to use (default: alphanumeric).
        
    Returns:
        Random string.
    """
    if alphabet is None:
        alphabet = string.ascii_letters + string.digits
    return "".join(secrets.choice(alphabet) for _ in range(length))


def secure_random_hex(length: int = 32) -> str:
    """Generate a random hex string."""
    return secrets.token_hex(length // 2)


def secure_random_urlsafe(length: int = 32) -> str:
    """Generate a URL-safe random string."""
    return secrets.token_urlsafe(length)


def constant_time_compare(a: bytes | str, b: bytes | str) -> bool:
    """Compare two values in constant time to prevent timing attacks.
    
    Args:
        a: First value.
        b: Second value.
        
    Returns:
        True if values are equal.
    """
    if isinstance(a, str):
        a = a.encode("utf-8")
    if isinstance(b, str):
        b = b.encode("utf-8")
    
    return hmac.compare_digest(a, b)
