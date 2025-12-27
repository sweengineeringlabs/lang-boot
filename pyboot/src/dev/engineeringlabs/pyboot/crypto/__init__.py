"""
Crypto Module - Encryption, decryption, and cryptographic utilities.

This module provides:
- Symmetric encryption (AES-GCM, Fernet)
- Asymmetric encryption (RSA)
- Digital signatures
- Key derivation
- Secure random generation
- Envelope encryption

Example:
    from dev.engineeringlabs.pyboot.crypto import encrypt, decrypt, generate_key
    from dev.engineeringlabs.pyboot.crypto import sign, verify
    from dev.engineeringlabs.pyboot.crypto import AESCipher, RSACipher
    
    # Symmetric encryption (AES)
    key = generate_key(32)  # 256-bit key
    encrypted = encrypt(b"secret data", key)
    decrypted = decrypt(encrypted, key)
    
    # Using Fernet (simpler, recommended)
    from dev.engineeringlabs.pyboot.crypto import FernetCipher
    cipher = FernetCipher.generate()
    token = cipher.encrypt(b"hello world")
    data = cipher.decrypt(token)
    
    # Digital signatures
    from dev.engineeringlabs.pyboot.crypto import generate_keypair
    private_key, public_key = generate_keypair()
    signature = sign(b"message", private_key)
    is_valid = verify(b"message", signature, public_key)
"""

from dev.engineeringlabs.pyboot.crypto.api import (
    # Protocols
    SymmetricCipher,
    AsymmetricCipher,
    Signer,
    KeyDerivation,
    # Types
    CryptoAlgorithm,
    KeyType,
    EncryptedData,
    KeyPair,
    # Exceptions
    CryptoError,
    EncryptionError,
    DecryptionError,
    KeyError as CryptoKeyError,
    SignatureError,
    VerificationError,
)

from dev.engineeringlabs.pyboot.crypto.core import (
    # Symmetric encryption
    encrypt,
    decrypt,
    generate_key,
    generate_iv,
    # AES
    AESCipher,
    # Fernet
    FernetCipher,
    # Asymmetric
    RSACipher,
    generate_keypair,
    # Signatures
    sign,
    verify,
    # Key derivation
    derive_key,
    # Utilities
    secure_random_bytes,
    secure_random_string,
    constant_time_compare,
)

__all__ = [
    # API - Protocols
    "SymmetricCipher",
    "AsymmetricCipher",
    "Signer",
    "KeyDerivation",
    # API - Types
    "CryptoAlgorithm",
    "KeyType",
    "EncryptedData",
    "KeyPair",
    # API - Exceptions
    "CryptoError",
    "EncryptionError",
    "DecryptionError",
    "CryptoKeyError",
    "SignatureError",
    "VerificationError",
    # Core - Symmetric
    "encrypt",
    "decrypt",
    "generate_key",
    "generate_iv",
    "AESCipher",
    "FernetCipher",
    # Core - Asymmetric
    "RSACipher",
    "generate_keypair",
    # Core - Signatures
    "sign",
    "verify",
    # Core - Key derivation
    "derive_key",
    # Core - Utilities
    "secure_random_bytes",
    "secure_random_string",
    "constant_time_compare",
]
