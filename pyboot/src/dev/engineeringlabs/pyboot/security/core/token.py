"""JWT token utilities."""

import base64
import hashlib
import hmac
import json
import time
from datetime import timedelta
from typing import Any

from dev.engineeringlabs.pyboot.security.api.token import TokenEncoder
from dev.engineeringlabs.pyboot.security.api.exceptions import TokenExpiredError, InvalidTokenError


class HmacTokenEncoder(TokenEncoder):
    """Simple HMAC-based token encoder (no dependencies)."""

    def __init__(self, secret: str, algorithm: str = "HS256") -> None:
        self._secret = secret.encode() if isinstance(secret, str) else secret
        self._algorithm = algorithm

    @property
    def algorithm(self) -> str:
        return self._algorithm

    def encode(
        self,
        payload: dict[str, Any],
        expires_in: timedelta | None = None,
    ) -> str:
        """Encode a payload to a token."""
        # Add standard claims
        claims = dict(payload)
        claims["iat"] = int(time.time())

        if expires_in:
            claims["exp"] = int(time.time() + expires_in.total_seconds())

        # Create header
        header = {"alg": self._algorithm, "typ": "JWT"}

        # Encode
        header_b64 = self._base64_encode(json.dumps(header))
        payload_b64 = self._base64_encode(json.dumps(claims))

        # Sign
        message = f"{header_b64}.{payload_b64}"
        signature = self._sign(message)
        signature_b64 = self._base64_encode(signature)

        return f"{header_b64}.{payload_b64}.{signature_b64}"

    def decode(self, token: str) -> dict[str, Any]:
        """Decode a token to a payload."""
        try:
            parts = token.split(".")
            if len(parts) != 3:
                raise InvalidTokenError("Invalid token format")

            header_b64, payload_b64, signature_b64 = parts

            # Verify signature
            message = f"{header_b64}.{payload_b64}"
            expected_signature = self._sign(message)
            expected_b64 = self._base64_encode(expected_signature)

            if not hmac.compare_digest(signature_b64, expected_b64):
                raise InvalidTokenError("Invalid token signature")

            # Decode payload
            payload_json = self._base64_decode(payload_b64)
            payload = json.loads(payload_json)

            # Check expiration
            if "exp" in payload:
                if time.time() > payload["exp"]:
                    raise TokenExpiredError()

            return payload

        except (json.JSONDecodeError, ValueError) as e:
            raise InvalidTokenError(str(e))

    def _sign(self, message: str) -> bytes:
        """Create HMAC signature."""
        return hmac.new(
            self._secret,
            message.encode(),
            hashlib.sha256,
        ).digest()

    def _base64_encode(self, data: str | bytes) -> str:
        """URL-safe base64 encode."""
        if isinstance(data, str):
            data = data.encode()
        return base64.urlsafe_b64encode(data).rstrip(b"=").decode()

    def _base64_decode(self, data: str) -> str:
        """URL-safe base64 decode."""
        # Add padding
        padding = 4 - len(data) % 4
        if padding != 4:
            data += "=" * padding
        return base64.urlsafe_b64decode(data).decode()


# Default encoder
_default_encoder: TokenEncoder | None = None
_default_secret: str = ""


def configure_tokens(secret: str, algorithm: str = "HS256") -> None:
    """Configure the default token encoder."""
    global _default_encoder, _default_secret
    _default_secret = secret
    _default_encoder = HmacTokenEncoder(secret, algorithm)


def get_token_encoder() -> TokenEncoder:
    """Get the default token encoder."""
    if _default_encoder is None:
        raise RuntimeError(
            "Token encoder not configured. Call configure_tokens() first."
        )
    return _default_encoder


def create_token(
    payload: dict[str, Any],
    secret: str | None = None,
    expires_in: timedelta | None = None,
) -> str:
    """
    Create a JWT token.

    Args:
        payload: Data to encode
        secret: Secret key (or use default)
        expires_in: Expiration time

    Returns:
        JWT token string
    """
    if secret:
        encoder = HmacTokenEncoder(secret)
    else:
        encoder = get_token_encoder()

    return encoder.encode(payload, expires_in)


def decode_token(token: str, secret: str | None = None) -> dict[str, Any]:
    """
    Decode a JWT token.

    Args:
        token: JWT token string
        secret: Secret key (or use default)

    Returns:
        Decoded payload

    Raises:
        AuthenticationError: If token is invalid or expired
    """
    if secret:
        encoder = HmacTokenEncoder(secret)
    else:
        encoder = get_token_encoder()

    return encoder.decode(token)


__all__ = [
    "HmacTokenEncoder",
    "configure_tokens",
    "get_token_encoder",
    "create_token",
    "decode_token",
]
