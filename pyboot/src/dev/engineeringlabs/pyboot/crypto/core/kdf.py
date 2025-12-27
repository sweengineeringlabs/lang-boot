"""
Key derivation functions - PBKDF2 implementation.
"""

import os
from dev.engineeringlabs.pyboot.crypto.api.exceptions import CryptoError

try:
    from cryptography.hazmat.primitives import hashes
    from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
    from cryptography.hazmat.backends import default_backend
    HAS_KDF = True
except ImportError:
    HAS_KDF = False


def derive_key(
    password: str | bytes,
    salt: bytes | None = None,
    length: int = 32,
    iterations: int = 100000,
) -> tuple[bytes, bytes]:
    """Derive a cryptographic key from a password.
    
    Uses PBKDF2-HMAC-SHA256 for key derivation.
    
    Args:
        password: Password to derive key from.
        salt: Random salt (generated if not provided).
        length: Desired key length in bytes.
        iterations: Number of iterations (higher = slower, more secure).
        
    Returns:
        Tuple of (derived_key, salt).
    """
    if not HAS_KDF:
        raise CryptoError("cryptography library not installed.")
    
    if salt is None:
        salt = os.urandom(16)
    
    if isinstance(password, str):
        password = password.encode("utf-8")
    
    kdf = PBKDF2HMAC(
        algorithm=hashes.SHA256(),
        length=length,
        salt=salt,
        iterations=iterations,
        backend=default_backend(),
    )
    
    key = kdf.derive(password)
    return key, salt


def verify_key(
    password: str | bytes,
    salt: bytes,
    expected_key: bytes,
    iterations: int = 100000,
) -> bool:
    """Verify a password against a derived key."""
    derived, _ = derive_key(password, salt, len(expected_key), iterations)
    return derived == expected_key


def is_kdf_available() -> bool:
    return HAS_KDF
