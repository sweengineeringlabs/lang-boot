"""
Crypto protocols - Interfaces for cryptographic operations.
"""

from typing import Protocol, runtime_checkable


@runtime_checkable
class SymmetricCipher(Protocol):
    """Protocol for symmetric encryption/decryption.
    
    Symmetric ciphers use the same key for encryption and decryption.
    Common algorithms: AES, ChaCha20, Fernet.
    
    Example:
        class MyCipher:
            def encrypt(self, plaintext: bytes, key: bytes) -> bytes:
                # AES-GCM encryption
                return encrypted_data
            
            def decrypt(self, ciphertext: bytes, key: bytes) -> bytes:
                # AES-GCM decryption
                return plaintext
    """
    
    def encrypt(self, plaintext: bytes, key: bytes) -> bytes:
        """Encrypt plaintext using the provided key.
        
        Args:
            plaintext: Data to encrypt.
            key: Encryption key (length depends on algorithm).
            
        Returns:
            Encrypted data (may include IV/nonce).
            
        Raises:
            EncryptionError: If encryption fails.
        """
        ...
    
    def decrypt(self, ciphertext: bytes, key: bytes) -> bytes:
        """Decrypt ciphertext using the provided key.
        
        Args:
            ciphertext: Data to decrypt.
            key: Decryption key (same as encryption key).
            
        Returns:
            Decrypted plaintext.
            
        Raises:
            DecryptionError: If decryption fails (wrong key, tampered data).
        """
        ...


@runtime_checkable
class AsymmetricCipher(Protocol):
    """Protocol for asymmetric encryption/decryption.
    
    Asymmetric ciphers use a public key for encryption and
    a private key for decryption.
    Common algorithms: RSA, ECIES.
    
    Example:
        class MyAsymmetricCipher:
            def encrypt(self, plaintext: bytes, public_key: bytes) -> bytes:
                return encrypted_data
            
            def decrypt(self, ciphertext: bytes, private_key: bytes) -> bytes:
                return plaintext
    """
    
    def encrypt(self, plaintext: bytes, public_key: bytes) -> bytes:
        """Encrypt plaintext using the public key.
        
        Args:
            plaintext: Data to encrypt.
            public_key: PEM-encoded public key.
            
        Returns:
            Encrypted data.
            
        Raises:
            EncryptionError: If encryption fails.
        """
        ...
    
    def decrypt(self, ciphertext: bytes, private_key: bytes) -> bytes:
        """Decrypt ciphertext using the private key.
        
        Args:
            ciphertext: Data to decrypt.
            private_key: PEM-encoded private key.
            
        Returns:
            Decrypted plaintext.
            
        Raises:
            DecryptionError: If decryption fails.
        """
        ...


@runtime_checkable
class Signer(Protocol):
    """Protocol for digital signatures.
    
    Digital signatures provide authentication and integrity.
    Common algorithms: RSA-PSS, ECDSA, Ed25519.
    
    Example:
        class MySigner:
            def sign(self, message: bytes, private_key: bytes) -> bytes:
                return signature
            
            def verify(self, message: bytes, signature: bytes, public_key: bytes) -> bool:
                return is_valid
    """
    
    def sign(self, message: bytes, private_key: bytes) -> bytes:
        """Create a digital signature.
        
        Args:
            message: Data to sign.
            private_key: PEM-encoded private key.
            
        Returns:
            Digital signature.
            
        Raises:
            SignatureError: If signing fails.
        """
        ...
    
    def verify(self, message: bytes, signature: bytes, public_key: bytes) -> bool:
        """Verify a digital signature.
        
        Args:
            message: Original data.
            signature: Signature to verify.
            public_key: PEM-encoded public key.
            
        Returns:
            True if signature is valid.
            
        Raises:
            VerificationError: If verification fails (not just invalid signature).
        """
        ...


@runtime_checkable
class KeyDerivation(Protocol):
    """Protocol for key derivation functions.
    
    Key derivation functions convert passwords or other input
    into cryptographic keys.
    Common algorithms: PBKDF2, Scrypt, Argon2.
    
    Example:
        class MyKDF:
            def derive(self, password: bytes, salt: bytes, key_length: int) -> bytes:
                return derived_key
    """
    
    def derive(
        self,
        password: bytes,
        salt: bytes,
        key_length: int = 32,
    ) -> bytes:
        """Derive a key from a password.
        
        Args:
            password: Input password.
            salt: Random salt (should be stored with the derived key).
            key_length: Desired key length in bytes.
            
        Returns:
            Derived key.
        """
        ...
    
    def derive_with_params(
        self,
        password: bytes,
        salt: bytes,
        key_length: int = 32,
        iterations: int = 100000,
    ) -> bytes:
        """Derive a key with custom parameters.
        
        Args:
            password: Input password.
            salt: Random salt.
            key_length: Desired key length.
            iterations: Number of iterations (higher = slower, more secure).
            
        Returns:
            Derived key.
        """
        ...
