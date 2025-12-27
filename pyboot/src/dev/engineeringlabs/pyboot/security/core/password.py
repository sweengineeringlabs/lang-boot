"""Password hashing utilities."""

import hashlib
import secrets
from typing import Literal

from dev.engineeringlabs.pyboot.security.api.password import PasswordHasher


class Pbkdf2Hasher(PasswordHasher):
    """PBKDF2 password hasher (built-in, no dependencies)."""

    def __init__(
        self,
        iterations: int = 600_000,
        salt_length: int = 16,
        hash_length: int = 32,
    ) -> None:
        self._iterations = iterations
        self._salt_length = salt_length
        self._hash_length = hash_length

    @property
    def algorithm(self) -> str:
        return "pbkdf2_sha256"

    def hash(self, password: str) -> str:
        """Hash a password using PBKDF2."""
        salt = secrets.token_bytes(self._salt_length)
        key = hashlib.pbkdf2_hmac(
            "sha256",
            password.encode("utf-8"),
            salt,
            self._iterations,
            dklen=self._hash_length,
        )
        # Format: algorithm$iterations$salt$hash
        salt_hex = salt.hex()
        key_hex = key.hex()
        return f"pbkdf2_sha256${self._iterations}${salt_hex}${key_hex}"

    def verify(self, password: str, hashed: str) -> bool:
        """Verify a password against a hash."""
        try:
            parts = hashed.split("$")
            if len(parts) != 4:
                return False

            algorithm, iterations_str, salt_hex, key_hex = parts
            if algorithm != "pbkdf2_sha256":
                return False

            iterations = int(iterations_str)
            salt = bytes.fromhex(salt_hex)
            expected_key = bytes.fromhex(key_hex)

            computed_key = hashlib.pbkdf2_hmac(
                "sha256",
                password.encode("utf-8"),
                salt,
                iterations,
                dklen=len(expected_key),
            )

            return secrets.compare_digest(computed_key, expected_key)
        except Exception:
            return False

    def needs_rehash(self, hashed: str) -> bool:
        """Check if hash needs upgrading."""
        try:
            parts = hashed.split("$")
            if len(parts) != 4:
                return True

            _, iterations_str, _, _ = parts
            return int(iterations_str) < self._iterations
        except Exception:
            return True


# Default hasher
_default_hasher: PasswordHasher = Pbkdf2Hasher()


def get_password_hasher() -> PasswordHasher:
    """Get the default password hasher."""
    return _default_hasher


def set_password_hasher(hasher: PasswordHasher) -> None:
    """Set the default password hasher."""
    global _default_hasher
    _default_hasher = hasher


def hash_password(password: str) -> str:
    """
    Hash a password.

    Args:
        password: Plain text password

    Returns:
        Hashed password string
    """
    return _default_hasher.hash(password)


def verify_password(password: str, hashed: str) -> bool:
    """
    Verify a password against a hash.

    Args:
        password: Plain text password
        hashed: Hashed password

    Returns:
        True if password matches
    """
    return _default_hasher.verify(password, hashed)


__all__ = [
    "Pbkdf2Hasher",
    "get_password_hasher",
    "set_password_hasher",
    "hash_password",
    "verify_password",
]
