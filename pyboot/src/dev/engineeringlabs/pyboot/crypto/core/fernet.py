"""
Fernet encryption - High-level symmetric encryption.
"""

from dev.engineeringlabs.pyboot.crypto.api.exceptions import EncryptionError, DecryptionError

# Try to use cryptography library
try:
    from cryptography.fernet import Fernet, InvalidToken
    HAS_FERNET = True
except ImportError:
    HAS_FERNET = False
    Fernet = None  # type: ignore
    InvalidToken = Exception  # type: ignore


class FernetCipher:
    """High-level symmetric encryption using Fernet.
    
    Fernet is a symmetric encryption method that provides:
    - AES-128-CBC encryption
    - HMAC-SHA256 authentication
    - Timestamp for TTL support
    - URL-safe base64 encoding
    
    Fernet is ideal for:
    - Encrypting tokens
    - Simple data encryption
    - Cases where you want a single-method solution
    
    Example:
        # Generate new cipher
        cipher = FernetCipher.generate()
        
        # Encrypt data
        token = cipher.encrypt(b"secret message")
        
        # Decrypt data
        message = cipher.decrypt(token)
        
        # Get key for storage
        key = cipher.key_string()
        
        # Restore from key
        cipher2 = FernetCipher.from_string(key)
    """
    
    def __init__(self, key: bytes) -> None:
        """Initialize with a Fernet key.
        
        Args:
            key: 32 bytes URL-safe base64-encoded key.
        """
        if not HAS_FERNET:
            raise EncryptionError(
                "cryptography library not installed. Install with: pip install cryptography"
            )
        
        try:
            self._fernet = Fernet(key)
            self._key = key
        except Exception as e:
            raise EncryptionError(
                f"Invalid Fernet key: {e}",
                cause=e,
                algorithm="Fernet",
            )
    
    @classmethod
    def generate(cls) -> "FernetCipher":
        """Generate a new Fernet cipher with random key.
        
        Returns:
            New FernetCipher instance.
            
        Example:
            cipher = FernetCipher.generate()
            # Store cipher.key_string() securely
        """
        if not HAS_FERNET:
            raise EncryptionError(
                "cryptography library not installed. Install with: pip install cryptography"
            )
        key = Fernet.generate_key()
        return cls(key)
    
    @classmethod
    def from_string(cls, key: str) -> "FernetCipher":
        """Create cipher from key string.
        
        Args:
            key: Base64-encoded Fernet key.
            
        Returns:
            FernetCipher instance.
        """
        return cls(key.encode("utf-8"))
    
    @property
    def key(self) -> bytes:
        """Get the Fernet key bytes."""
        return self._key
    
    def key_string(self) -> str:
        """Get the Fernet key as string for storage."""
        return self._key.decode("utf-8")
    
    def encrypt(self, data: bytes) -> bytes:
        """Encrypt data.
        
        Args:
            data: Data to encrypt.
            
        Returns:
            Fernet token (URL-safe base64).
            
        Raises:
            EncryptionError: If encryption fails.
        """
        try:
            return self._fernet.encrypt(data)
        except Exception as e:
            raise EncryptionError(
                f"Fernet encryption failed: {e}",
                cause=e,
                algorithm="Fernet",
            )
    
    def decrypt(self, token: bytes, *, ttl: int | None = None) -> bytes:
        """Decrypt a Fernet token.
        
        Args:
            token: Fernet token to decrypt.
            ttl: Optional time-to-live in seconds. If the token is
                 older than ttl seconds, decryption will fail.
                 
        Returns:
            Decrypted data.
            
        Raises:
            DecryptionError: If decryption fails or token expired.
            
        Example:
            # Decrypt with TTL check
            data = cipher.decrypt(token, ttl=3600)  # Valid for 1 hour
        """
        try:
            if ttl is not None:
                return self._fernet.decrypt(token, ttl=ttl)
            return self._fernet.decrypt(token)
        except InvalidToken:
            raise DecryptionError(
                "Invalid or expired Fernet token",
                algorithm="Fernet",
            )
        except Exception as e:
            raise DecryptionError(
                f"Fernet decryption failed: {e}",
                cause=e,
                algorithm="Fernet",
            )
    
    def encrypt_string(self, data: str, encoding: str = "utf-8") -> str:
        """Encrypt a string and return base64 string.
        
        Args:
            data: String to encrypt.
            encoding: String encoding.
            
        Returns:
            Base64-encoded encrypted string.
        """
        encrypted = self.encrypt(data.encode(encoding))
        return encrypted.decode("utf-8")
    
    def decrypt_string(
        self,
        token: str,
        encoding: str = "utf-8",
        ttl: int | None = None,
    ) -> str:
        """Decrypt a base64 string token.
        
        Args:
            token: Base64 token string.
            encoding: String encoding.
            ttl: Optional TTL in seconds.
            
        Returns:
            Decrypted string.
        """
        decrypted = self.decrypt(token.encode("utf-8"), ttl=ttl)
        return decrypted.decode(encoding)


def is_fernet_available() -> bool:
    """Check if Fernet is available."""
    return HAS_FERNET
