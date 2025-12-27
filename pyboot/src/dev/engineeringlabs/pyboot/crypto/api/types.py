"""
Crypto types - Data structures for cryptographic operations.
"""

from dataclasses import dataclass, field
from enum import Enum
from typing import Any


class CryptoAlgorithm(str, Enum):
    """Supported cryptographic algorithms."""
    
    # Symmetric encryption
    AES_GCM = "aes-gcm"
    """AES-GCM: Authenticated encryption with associated data."""
    
    AES_CBC = "aes-cbc"
    """AES-CBC: Block cipher mode (requires separate MAC)."""
    
    CHACHA20_POLY1305 = "chacha20-poly1305"
    """ChaCha20-Poly1305: Fast authenticated encryption."""
    
    FERNET = "fernet"
    """Fernet: High-level symmetric encryption (AES-128-CBC + HMAC)."""
    
    # Asymmetric encryption
    RSA_OAEP = "rsa-oaep"
    """RSA-OAEP: RSA with Optimal Asymmetric Encryption Padding."""
    
    RSA_PKCS1 = "rsa-pkcs1"
    """RSA-PKCS1: Legacy RSA padding (not recommended)."""
    
    # Signatures
    RSA_PSS = "rsa-pss"
    """RSA-PSS: RSA Probabilistic Signature Scheme."""
    
    ECDSA = "ecdsa"
    """ECDSA: Elliptic Curve Digital Signature Algorithm."""
    
    ED25519 = "ed25519"
    """Ed25519: Fast elliptic curve signatures."""
    
    # Key derivation
    PBKDF2 = "pbkdf2"
    """PBKDF2: Password-Based Key Derivation Function 2."""
    
    SCRYPT = "scrypt"
    """Scrypt: Memory-hard key derivation."""
    
    ARGON2 = "argon2"
    """Argon2: Modern memory-hard key derivation."""
    
    def __str__(self) -> str:
        return self.value


class KeyType(str, Enum):
    """Types of cryptographic keys."""
    
    SYMMETRIC = "symmetric"
    """Symmetric key for encryption/decryption."""
    
    RSA_PRIVATE = "rsa-private"
    """RSA private key."""
    
    RSA_PUBLIC = "rsa-public"
    """RSA public key."""
    
    EC_PRIVATE = "ec-private"
    """Elliptic curve private key."""
    
    EC_PUBLIC = "ec-public"
    """Elliptic curve public key."""
    
    ED25519_PRIVATE = "ed25519-private"
    """Ed25519 private key."""
    
    ED25519_PUBLIC = "ed25519-public"
    """Ed25519 public key."""
    
    def __str__(self) -> str:
        return self.value


@dataclass(frozen=True)
class EncryptedData:
    """Container for encrypted data with metadata.
    
    Attributes:
        ciphertext: The encrypted data.
        iv: Initialization vector (for block ciphers).
        tag: Authentication tag (for AEAD ciphers).
        algorithm: Algorithm used for encryption.
        key_id: Optional key identifier.
    
    Example:
        data = EncryptedData(
            ciphertext=encrypted_bytes,
            iv=iv_bytes,
            tag=auth_tag,
            algorithm=CryptoAlgorithm.AES_GCM,
        )
        
        # Serialize for storage
        serialized = data.to_bytes()
        
        # Restore
        restored = EncryptedData.from_bytes(serialized)
    """
    
    ciphertext: bytes
    """The encrypted data."""
    
    iv: bytes = field(default=b"")
    """Initialization vector or nonce."""
    
    tag: bytes = field(default=b"")
    """Authentication tag for AEAD algorithms."""
    
    algorithm: CryptoAlgorithm = CryptoAlgorithm.AES_GCM
    """Algorithm used for encryption."""
    
    key_id: str | None = None
    """Optional key identifier for key management."""
    
    associated_data: bytes = field(default=b"")
    """Additional authenticated data (AAD) if used."""
    
    def to_bytes(self) -> bytes:
        """Serialize to bytes for storage.
        
        Format: [iv_len:2][iv][tag_len:2][tag][ciphertext]
        """
        iv_len = len(self.iv).to_bytes(2, "big")
        tag_len = len(self.tag).to_bytes(2, "big")
        return iv_len + self.iv + tag_len + self.tag + self.ciphertext
    
    @classmethod
    def from_bytes(
        cls,
        data: bytes,
        algorithm: CryptoAlgorithm = CryptoAlgorithm.AES_GCM,
    ) -> "EncryptedData":
        """Deserialize from bytes."""
        iv_len = int.from_bytes(data[0:2], "big")
        iv = data[2:2 + iv_len]
        
        tag_len_start = 2 + iv_len
        tag_len = int.from_bytes(data[tag_len_start:tag_len_start + 2], "big")
        tag = data[tag_len_start + 2:tag_len_start + 2 + tag_len]
        
        ciphertext = data[tag_len_start + 2 + tag_len:]
        
        return cls(
            ciphertext=ciphertext,
            iv=iv,
            tag=tag,
            algorithm=algorithm,
        )


@dataclass(frozen=True)
class KeyPair:
    """Container for asymmetric key pair.
    
    Attributes:
        private_key: PEM-encoded private key.
        public_key: PEM-encoded public key.
        algorithm: Key algorithm.
        key_size: Key size in bits.
    
    Example:
        keypair = generate_keypair(algorithm=CryptoAlgorithm.RSA_OAEP, key_size=2048)
        
        # Access keys
        print(keypair.public_key.decode())
        
        # Save to files
        with open("private.pem", "wb") as f:
            f.write(keypair.private_key)
    """
    
    private_key: bytes
    """PEM-encoded private key."""
    
    public_key: bytes
    """PEM-encoded public key."""
    
    algorithm: CryptoAlgorithm = CryptoAlgorithm.RSA_OAEP
    """Key algorithm."""
    
    key_size: int = 2048
    """Key size in bits."""
    
    key_id: str | None = None
    """Optional key identifier."""
    
    def private_key_pem(self) -> str:
        """Get private key as PEM string."""
        return self.private_key.decode("utf-8")
    
    def public_key_pem(self) -> str:
        """Get public key as PEM string."""
        return self.public_key.decode("utf-8")
