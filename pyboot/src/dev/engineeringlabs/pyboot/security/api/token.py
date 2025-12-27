"""Token encoding interface."""

from abc import ABC, abstractmethod
from typing import Any
from datetime import timedelta


class TokenEncoder(ABC):
    """
    Abstract interface for token encoding/decoding.

    Example:
        encoder = get_token_encoder()
        token = encoder.encode({"user_id": "123"}, expires_in=timedelta(hours=1))
        payload = encoder.decode(token)
    """

    @property
    @abstractmethod
    def algorithm(self) -> str:
        """Get the encoding algorithm."""
        ...

    @abstractmethod
    def encode(
        self,
        payload: dict[str, Any],
        expires_in: timedelta | None = None,
    ) -> str:
        """
        Encode a payload to a token.

        Args:
            payload: Data to encode
            expires_in: Optional expiration time

        Returns:
            Encoded token string
        """
        ...

    @abstractmethod
    def decode(self, token: str) -> dict[str, Any]:
        """
        Decode a token to a payload.

        Args:
            token: Token string

        Returns:
            Decoded payload

        Raises:
            AuthenticationError: If token is invalid or expired
        """
        ...

    def refresh(self, token: str, expires_in: timedelta | None = None) -> str:
        """
        Refresh a token with a new expiration.

        Args:
            token: Existing token
            expires_in: New expiration time

        Returns:
            New token with extended expiration
        """
        payload = self.decode(token)
        # Remove old expiration claims
        payload.pop("exp", None)
        payload.pop("iat", None)
        return self.encode(payload, expires_in)


__all__ = ["TokenEncoder"]
