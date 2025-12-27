"""
Crypto API - Public interfaces and types.
"""

from dev.engineeringlabs.pyboot.crypto.api.protocols import (
    SymmetricCipher,
    AsymmetricCipher,
    Signer,
    KeyDerivation,
)

from dev.engineeringlabs.pyboot.crypto.api.types import (
    CryptoAlgorithm,
    KeyType,
    EncryptedData,
    KeyPair,
)

from dev.engineeringlabs.pyboot.crypto.api.exceptions import (
    CryptoError,
    EncryptionError,
    DecryptionError,
    KeyError,
    SignatureError,
    VerificationError,
)

__all__ = [
    # Protocols
    "SymmetricCipher",
    "AsymmetricCipher",
    "Signer",
    "KeyDerivation",
    # Types
    "CryptoAlgorithm",
    "KeyType",
    "EncryptedData",
    "KeyPair",
    # Exceptions
    "CryptoError",
    "EncryptionError",
    "DecryptionError",
    "KeyError",
    "SignatureError",
    "VerificationError",
]
