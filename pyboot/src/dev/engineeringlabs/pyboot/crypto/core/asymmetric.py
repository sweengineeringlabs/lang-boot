"""
Asymmetric encryption - RSA implementation.
"""

from dev.engineeringlabs.pyboot.crypto.api.types import CryptoAlgorithm, KeyPair
from dev.engineeringlabs.pyboot.crypto.api.exceptions import (
    EncryptionError,
    DecryptionError,
    KeyError as CryptoKeyError,
)

# Try to use cryptography library
try:
    from cryptography.hazmat.primitives import hashes, serialization
    from cryptography.hazmat.primitives.asymmetric import rsa, padding
    from cryptography.hazmat.backends import default_backend
    HAS_RSA = True
except ImportError:
    HAS_RSA = False
    rsa = None  # type: ignore


def generate_keypair(
    key_size: int = 2048,
    *,
    public_exponent: int = 65537,
) -> KeyPair:
    """Generate an RSA key pair.
    
    Args:
        key_size: Key size in bits (2048, 3072, or 4096 recommended).
        public_exponent: Public exponent (65537 is standard).
        
    Returns:
        KeyPair with PEM-encoded private and public keys.
        
    Example:
        keypair = generate_keypair(2048)
        
        # Save keys
        with open("private.pem", "wb") as f:
            f.write(keypair.private_key)
        
        with open("public.pem", "wb") as f:
            f.write(keypair.public_key)
    """
    if not HAS_RSA:
        raise CryptoKeyError(
            "cryptography library not installed. Install with: pip install cryptography",
            key_type="RSA",
        )
    
    if key_size < 2048:
        raise CryptoKeyError(
            f"Key size {key_size} is too small. Minimum is 2048 bits.",
            key_type="RSA",
        )
    
    try:
        # Generate private key
        private_key = rsa.generate_private_key(
            public_exponent=public_exponent,
            key_size=key_size,
            backend=default_backend(),
        )
        
        # Serialize private key
        private_pem = private_key.private_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PrivateFormat.PKCS8,
            encryption_algorithm=serialization.NoEncryption(),
        )
        
        # Serialize public key
        public_key = private_key.public_key()
        public_pem = public_key.public_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PublicFormat.SubjectPublicKeyInfo,
        )
        
        return KeyPair(
            private_key=private_pem,
            public_key=public_pem,
            algorithm=CryptoAlgorithm.RSA_OAEP,
            key_size=key_size,
        )
        
    except Exception as e:
        raise CryptoKeyError.generation_failed("RSA", e)


class RSACipher:
    """RSA asymmetric encryption using OAEP padding.
    
    RSA is used for:
    - Encrypting small amounts of data (e.g., keys)
    - Key exchange
    - Digital signatures
    
    Limitations:
    - Can only encrypt data smaller than key size minus padding
    - For 2048-bit key: max ~190 bytes
    - Use for encrypting symmetric keys, not large data
    
    Example:
        # Generate keypair
        keypair = generate_keypair(2048)
        cipher = RSACipher(keypair)
        
        # Encrypt with public key
        encrypted = cipher.encrypt(b"secret key")
        
        # Decrypt with private key
        decrypted = cipher.decrypt(encrypted)
        
        # Encrypt with just public key
        public_cipher = RSACipher.from_public_key(public_pem)
        encrypted = public_cipher.encrypt(b"message")
    """
    
    def __init__(self, keypair: KeyPair) -> None:
        """Initialize with key pair.
        
        Args:
            keypair: KeyPair containing private and/or public keys.
        """
        if not HAS_RSA:
            raise EncryptionError(
                "cryptography library not installed. Install with: pip install cryptography"
            )
        
        self._keypair = keypair
        self._private_key = None
        self._public_key = None
        
        # Load private key if available
        if keypair.private_key:
            try:
                self._private_key = serialization.load_pem_private_key(
                    keypair.private_key,
                    password=None,
                    backend=default_backend(),
                )
            except Exception as e:
                raise CryptoKeyError.invalid_format("RSA private")
        
        # Load public key
        if keypair.public_key:
            try:
                self._public_key = serialization.load_pem_public_key(
                    keypair.public_key,
                    backend=default_backend(),
                )
            except Exception as e:
                raise CryptoKeyError.invalid_format("RSA public")
    
    @classmethod
    def from_public_key(cls, public_key_pem: bytes) -> "RSACipher":
        """Create cipher with only public key (encrypt only).
        
        Args:
            public_key_pem: PEM-encoded public key.
            
        Returns:
            RSACipher that can only encrypt.
        """
        keypair = KeyPair(
            private_key=b"",
            public_key=public_key_pem,
            algorithm=CryptoAlgorithm.RSA_OAEP,
        )
        return cls(keypair)
    
    @classmethod
    def from_private_key(cls, private_key_pem: bytes) -> "RSACipher":
        """Create cipher from private key (derives public key).
        
        Args:
            private_key_pem: PEM-encoded private key.
            
        Returns:
            RSACipher with full capabilities.
        """
        if not HAS_RSA:
            raise EncryptionError(
                "cryptography library not installed."
            )
        
        # Load private key and derive public
        private_key = serialization.load_pem_private_key(
            private_key_pem,
            password=None,
            backend=default_backend(),
        )
        
        public_pem = private_key.public_key().public_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PublicFormat.SubjectPublicKeyInfo,
        )
        
        keypair = KeyPair(
            private_key=private_key_pem,
            public_key=public_pem,
            algorithm=CryptoAlgorithm.RSA_OAEP,
        )
        return cls(keypair)
    
    @classmethod
    def generate(cls, key_size: int = 2048) -> "RSACipher":
        """Generate new RSA cipher with random keypair.
        
        Args:
            key_size: Key size in bits.
            
        Returns:
            New RSACipher instance.
        """
        keypair = generate_keypair(key_size)
        return cls(keypair)
    
    @property
    def keypair(self) -> KeyPair:
        """Get the key pair."""
        return self._keypair
    
    @property
    def can_encrypt(self) -> bool:
        """Check if encryption is available (has public key)."""
        return self._public_key is not None
    
    @property
    def can_decrypt(self) -> bool:
        """Check if decryption is available (has private key)."""
        return self._private_key is not None
    
    def encrypt(self, plaintext: bytes) -> bytes:
        """Encrypt data with public key.
        
        Args:
            plaintext: Data to encrypt (limited size).
            
        Returns:
            Encrypted data.
            
        Raises:
            EncryptionError: If encryption fails.
        """
        if not self.can_encrypt:
            raise EncryptionError(
                "No public key available for encryption",
                algorithm="RSA-OAEP",
            )
        
        try:
            return self._public_key.encrypt(
                plaintext,
                padding.OAEP(
                    mgf=padding.MGF1(algorithm=hashes.SHA256()),
                    algorithm=hashes.SHA256(),
                    label=None,
                ),
            )
        except Exception as e:
            raise EncryptionError(
                f"RSA encryption failed: {e}",
                cause=e,
                algorithm="RSA-OAEP",
            )
    
    def decrypt(self, ciphertext: bytes) -> bytes:
        """Decrypt data with private key.
        
        Args:
            ciphertext: Encrypted data.
            
        Returns:
            Decrypted plaintext.
            
        Raises:
            DecryptionError: If decryption fails.
        """
        if not self.can_decrypt:
            raise DecryptionError(
                "No private key available for decryption",
                algorithm="RSA-OAEP",
            )
        
        try:
            return self._private_key.decrypt(
                ciphertext,
                padding.OAEP(
                    mgf=padding.MGF1(algorithm=hashes.SHA256()),
                    algorithm=hashes.SHA256(),
                    label=None,
                ),
            )
        except Exception as e:
            raise DecryptionError(
                f"RSA decryption failed: {e}",
                cause=e,
                algorithm="RSA-OAEP",
            )


def is_rsa_available() -> bool:
    """Check if RSA is available."""
    return HAS_RSA
