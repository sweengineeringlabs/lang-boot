"""Crypto exceptions - Standalone error types for cryptographic operations."""


class CryptoError(Exception):
    """Base exception for cryptographic errors.
    
    Parent class for all crypto-related exceptions.
    
    Example:
        try:
            decrypted = decrypt(data, key)
        except CryptoError as e:
            logger.error(f"Crypto operation failed: {e}")
    """
    
    def __init__(
        self,
        message: str,
        *,
        cause: Exception | None = None,
        algorithm: str | None = None,
    ) -> None:
        super().__init__(message)
        self.message = message
        self.cause = cause
        self.algorithm = algorithm


class EncryptionError(CryptoError):
    """Exception for encryption failures.
    
    Raised when encryption cannot be completed.
    
    Causes:
    - Invalid key length
    - Invalid plaintext
    - Algorithm not available
    """
    
    @classmethod
    def invalid_key_size(cls, expected: int, actual: int, algorithm: str) -> "EncryptionError":
        """Create error for invalid key size."""
        return cls(
            f"Invalid key size for {algorithm}: expected {expected} bytes, got {actual}",
            algorithm=algorithm,
        )


class DecryptionError(CryptoError):
    """Exception for decryption failures.
    
    Raised when decryption cannot be completed.
    
    Causes:
    - Wrong decryption key
    - Tampered ciphertext
    - Invalid ciphertext format
    - Authentication tag mismatch
    """
    
    @classmethod
    def authentication_failed(cls, algorithm: str) -> "DecryptionError":
        """Create error for authentication failure."""
        return cls(
            f"Authentication failed - data may be tampered or wrong key ({algorithm})",
            algorithm=algorithm,
        )
    
    @classmethod
    def invalid_ciphertext(cls, reason: str, algorithm: str) -> "DecryptionError":
        """Create error for invalid ciphertext."""
        return cls(
            f"Invalid ciphertext: {reason} ({algorithm})",
            algorithm=algorithm,
        )


class CryptoKeyError(CryptoError):
    """Exception for key-related errors.
    
    Causes:
    - Invalid key format
    - Key generation failure
    - Key derivation failure
    """
    
    def __init__(
        self,
        message: str,
        *,
        cause: Exception | None = None,
        key_type: str | None = None,
    ) -> None:
        super().__init__(message, cause=cause)
        self.key_type = key_type
    
    @classmethod
    def invalid_format(cls, key_type: str) -> "CryptoKeyError":
        """Create error for invalid key format."""
        return cls(
            f"Invalid {key_type} key format",
            key_type=key_type,
        )
    
    @classmethod
    def generation_failed(cls, algorithm: str, cause: Exception) -> "CryptoKeyError":
        """Create error for key generation failure."""
        return cls(
            f"Failed to generate {algorithm} key: {cause}",
            cause=cause,
        )


class SignatureError(CryptoError):
    """Exception for signature creation failures.
    
    Raised when signing a message fails.
    
    Causes:
    - Invalid private key
    - Algorithm not available
    """
    pass


class VerificationError(CryptoError):
    """Exception for signature verification errors.
    
    Note: This is for verification process errors, not
    for invalid signatures (which return False).
    
    Causes:
    - Invalid public key
    - Invalid signature format
    - Algorithm not available
    """
    pass


__all__ = [
    "CryptoError",
    "EncryptionError",
    "DecryptionError",
    "CryptoKeyError",
    "SignatureError",
    "VerificationError",
]
