"""
Symmetric encryption - AES-GCM implementation.
"""

import os
from typing import overload

from dev.engineeringlabs.pyboot.crypto.api.types import CryptoAlgorithm, EncryptedData
from dev.engineeringlabs.pyboot.crypto.api.exceptions import EncryptionError, DecryptionError

# Try to use cryptography library
try:
    from cryptography.hazmat.primitives.ciphers.aead import AESGCM
    HAS_CRYPTO = True
except ImportError:
    HAS_CRYPTO = False
    AESGCM = None  # type: ignore


# Key sizes for AES
AES_KEY_SIZES = {128: 16, 192: 24, 256: 32}
AES_IV_SIZE = 12  # 96 bits for GCM
AES_TAG_SIZE = 16  # 128 bits


def generate_key(length: int = 32) -> bytes:
    """Generate a cryptographically secure random key.
    
    Args:
        length: Key length in bytes (16, 24, or 32 for AES).
        
    Returns:
        Random bytes suitable for use as an encryption key.
        
    Example:
        key_128 = generate_key(16)  # AES-128
        key_256 = generate_key(32)  # AES-256
    """
    if length not in AES_KEY_SIZES.values():
        raise EncryptionError(
            f"Invalid key length: {length}. Must be 16, 24, or 32 bytes."
        )
    return os.urandom(length)


def generate_iv(length: int = AES_IV_SIZE) -> bytes:
    """Generate a random initialization vector.
    
    Args:
        length: IV length in bytes (12 for GCM is recommended).
        
    Returns:
        Random bytes for use as IV/nonce.
    """
    return os.urandom(length)


def encrypt(
    plaintext: bytes,
    key: bytes,
    *,
    associated_data: bytes | None = None,
) -> bytes:
    """Encrypt data using AES-GCM.
    
    AES-GCM provides authenticated encryption, which means it
    protects both confidentiality and integrity.
    
    Args:
        plaintext: Data to encrypt.
        key: Encryption key (16, 24, or 32 bytes).
        associated_data: Additional data to authenticate but not encrypt.
        
    Returns:
        Encrypted data with IV prepended: [IV][ciphertext][tag].
        
    Raises:
        EncryptionError: If encryption fails.
        
    Example:
        key = generate_key(32)
        encrypted = encrypt(b"secret message", key)
        
        # With associated data
        encrypted = encrypt(b"message", key, associated_data=b"header")
    """
    if not HAS_CRYPTO:
        raise EncryptionError(
            "cryptography library not installed. Install with: pip install cryptography"
        )
    
    # Validate key size
    if len(key) not in AES_KEY_SIZES.values():
        raise EncryptionError.invalid_key_size(
            expected=32,
            actual=len(key),
            algorithm="AES-GCM",
        )
    
    try:
        # Generate random IV
        iv = generate_iv()
        
        # Create cipher and encrypt
        aesgcm = AESGCM(key)
        ciphertext = aesgcm.encrypt(iv, plaintext, associated_data)
        
        # Return IV + ciphertext (tag is appended by encrypt)
        return iv + ciphertext
        
    except Exception as e:
        raise EncryptionError(
            f"Encryption failed: {e}",
            cause=e,
            algorithm="AES-GCM",
        )


def decrypt(
    ciphertext: bytes,
    key: bytes,
    *,
    associated_data: bytes | None = None,
) -> bytes:
    """Decrypt data encrypted with AES-GCM.
    
    Args:
        ciphertext: Encrypted data (IV + ciphertext + tag).
        key: Decryption key (same key used for encryption).
        associated_data: Same associated data used during encryption.
        
    Returns:
        Decrypted plaintext.
        
    Raises:
        DecryptionError: If decryption fails (wrong key, tampered data).
        
    Example:
        key = generate_key(32)
        encrypted = encrypt(b"secret", key)
        decrypted = decrypt(encrypted, key)
        assert decrypted == b"secret"
    """
    if not HAS_CRYPTO:
        raise DecryptionError(
            "cryptography library not installed. Install with: pip install cryptography"
        )
    
    # Validate key size
    if len(key) not in AES_KEY_SIZES.values():
        raise DecryptionError(
            f"Invalid key size: expected 16, 24, or 32 bytes, got {len(key)}",
            algorithm="AES-GCM",
        )
    
    # Validate ciphertext length
    if len(ciphertext) < AES_IV_SIZE + AES_TAG_SIZE:
        raise DecryptionError.invalid_ciphertext(
            "Data too short",
            algorithm="AES-GCM",
        )
    
    try:
        # Extract IV and ciphertext
        iv = ciphertext[:AES_IV_SIZE]
        encrypted_data = ciphertext[AES_IV_SIZE:]
        
        # Create cipher and decrypt
        aesgcm = AESGCM(key)
        plaintext = aesgcm.decrypt(iv, encrypted_data, associated_data)
        
        return plaintext
        
    except Exception as e:
        # Catch authentication failures
        if "tag" in str(e).lower() or "authentication" in str(e).lower():
            raise DecryptionError.authentication_failed("AES-GCM")
        raise DecryptionError(
            f"Decryption failed: {e}",
            cause=e,
            algorithm="AES-GCM",
        )


class AESCipher:
    """AES-GCM cipher with optional key management.
    
    Provides a convenient class-based interface for AES encryption.
    
    Example:
        # With auto-generated key
        cipher = AESCipher.generate()
        encrypted = cipher.encrypt(b"hello")
        decrypted = cipher.decrypt(encrypted)
        
        # With existing key
        key = generate_key(32)
        cipher = AESCipher(key)
        encrypted = cipher.encrypt(b"hello")
    """
    
    def __init__(self, key: bytes) -> None:
        """Initialize with encryption key.
        
        Args:
            key: AES key (16, 24, or 32 bytes).
        """
        if len(key) not in AES_KEY_SIZES.values():
            raise EncryptionError.invalid_key_size(
                expected=32,
                actual=len(key),
                algorithm="AES-GCM",
            )
        self._key = key
    
    @classmethod
    def generate(cls, key_size: int = 256) -> "AESCipher":
        """Create cipher with new random key.
        
        Args:
            key_size: Key size in bits (128, 192, or 256).
            
        Returns:
            New AESCipher instance.
        """
        if key_size not in AES_KEY_SIZES:
            raise EncryptionError(
                f"Invalid key size: {key_size}. Must be 128, 192, or 256."
            )
        return cls(generate_key(AES_KEY_SIZES[key_size]))
    
    @property
    def key(self) -> bytes:
        """Get the encryption key."""
        return self._key
    
    def encrypt(
        self,
        plaintext: bytes,
        *,
        associated_data: bytes | None = None,
    ) -> bytes:
        """Encrypt data.
        
        Args:
            plaintext: Data to encrypt.
            associated_data: Additional authenticated data.
            
        Returns:
            Encrypted data.
        """
        return encrypt(plaintext, self._key, associated_data=associated_data)
    
    def decrypt(
        self,
        ciphertext: bytes,
        *,
        associated_data: bytes | None = None,
    ) -> bytes:
        """Decrypt data.
        
        Args:
            ciphertext: Data to decrypt.
            associated_data: Associated data used during encryption.
            
        Returns:
            Decrypted plaintext.
        """
        return decrypt(ciphertext, self._key, associated_data=associated_data)
    
    def encrypt_to_data(
        self,
        plaintext: bytes,
        *,
        associated_data: bytes | None = None,
    ) -> EncryptedData:
        """Encrypt and return structured data.
        
        Returns:
            EncryptedData with ciphertext, IV, and tag.
        """
        result = self.encrypt(plaintext, associated_data=associated_data)
        iv = result[:AES_IV_SIZE]
        ciphertext_with_tag = result[AES_IV_SIZE:]
        tag = ciphertext_with_tag[-AES_TAG_SIZE:]
        ciphertext = ciphertext_with_tag[:-AES_TAG_SIZE]
        
        return EncryptedData(
            ciphertext=ciphertext,
            iv=iv,
            tag=tag,
            algorithm=CryptoAlgorithm.AES_GCM,
            associated_data=associated_data or b"",
        )


def is_crypto_available() -> bool:
    """Check if cryptography library is available."""
    return HAS_CRYPTO
