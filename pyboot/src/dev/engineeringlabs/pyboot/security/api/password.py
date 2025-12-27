"""Password hashing interface."""

from abc import ABC, abstractmethod


class PasswordHasher(ABC):
    """
    Abstract interface for password hashing.

    Example:
        hasher = get_password_hasher()
        hashed = hasher.hash("mypassword")
        is_valid = hasher.verify("mypassword", hashed)
    """

    @property
    @abstractmethod
    def algorithm(self) -> str:
        """Get the hashing algorithm name."""
        ...

    @abstractmethod
    def hash(self, password: str) -> str:
        """
        Hash a password.

        Args:
            password: Plain text password

        Returns:
            Hashed password string
        """
        ...

    @abstractmethod
    def verify(self, password: str, hashed: str) -> bool:
        """
        Verify a password against a hash.

        Args:
            password: Plain text password
            hashed: Hashed password

        Returns:
            True if password matches
        """
        ...

    def needs_rehash(self, hashed: str) -> bool:
        """
        Check if a hash needs to be rehashed.

        This is useful when upgrading hash parameters.

        Args:
            hashed: Hashed password

        Returns:
            True if rehashing is recommended
        """
        return False


__all__ = ["PasswordHasher"]
