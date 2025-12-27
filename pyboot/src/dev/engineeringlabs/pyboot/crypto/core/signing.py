"""
Digital signatures - RSA-PSS and ECDSA implementations.
"""

from dev.engineeringlabs.pyboot.crypto.api.exceptions import SignatureError, VerificationError

try:
    from cryptography.hazmat.primitives import hashes, serialization
    from cryptography.hazmat.primitives.asymmetric import padding
    from cryptography.hazmat.backends import default_backend
    from cryptography.exceptions import InvalidSignature
    HAS_SIGNING = True
except ImportError:
    HAS_SIGNING = False


def sign(message: bytes, private_key: bytes) -> bytes:
    """Sign a message using RSA-PSS."""
    if not HAS_SIGNING:
        raise SignatureError("cryptography library not installed.")
    
    try:
        key = serialization.load_pem_private_key(
            private_key, password=None, backend=default_backend()
        )
        return key.sign(
            message,
            padding.PSS(mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.MAX_LENGTH),
            hashes.SHA256(),
        )
    except Exception as e:
        raise SignatureError(f"Signing failed: {e}", cause=e, algorithm="RSA-PSS")


def verify(message: bytes, signature: bytes, public_key: bytes) -> bool:
    """Verify a digital signature. Returns True if valid."""
    if not HAS_SIGNING:
        raise VerificationError("cryptography library not installed.")
    
    try:
        key = serialization.load_pem_public_key(public_key, backend=default_backend())
        key.verify(
            signature, message,
            padding.PSS(mgf=padding.MGF1(hashes.SHA256()), salt_length=padding.PSS.MAX_LENGTH),
            hashes.SHA256(),
        )
        return True
    except InvalidSignature:
        return False
    except Exception as e:
        raise VerificationError(f"Verification failed: {e}", cause=e, algorithm="RSA-PSS")


def is_signing_available() -> bool:
    return HAS_SIGNING
