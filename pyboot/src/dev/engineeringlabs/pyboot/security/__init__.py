"""
Security Module - Authentication, authorization, and encryption.

This module provides:
- Password hashing (bcrypt, argon2)
- JWT token handling
- Authorization decorators
- Secret management

Example:
    from dev.engineeringlabs.pyboot.security import hash_password, verify_password
    from dev.engineeringlabs.pyboot.security import create_token, decode_token
    from dev.engineeringlabs.pyboot.security import require_permission

    # Password hashing
    hashed = hash_password("mysecret")
    is_valid = verify_password("mysecret", hashed)

    # JWT tokens
    token = create_token({"user_id": "123"}, secret="key")
    payload = decode_token(token, secret="key")

    # Authorization
    @require_permission("admin")
    async def admin_action():
        ...
"""

from dev.engineeringlabs.pyboot.security.api import (
    PasswordHasher,
    TokenEncoder,
    Principal,
    Permission,
    SecurityError,
    AuthenticationError,
    AuthorizationError,
)

from dev.engineeringlabs.pyboot.security.core import (
    hash_password,
    verify_password,
    create_token,
    decode_token,
    require_permission,
    require_role,
    get_current_principal,
    set_current_principal,
)

__all__ = [
    # API
    "PasswordHasher",
    "TokenEncoder",
    "Principal",
    "Permission",
    "SecurityError",
    "AuthenticationError",
    "AuthorizationError",
    # Core
    "hash_password",
    "verify_password",
    "create_token",
    "decode_token",
    "require_permission",
    "require_role",
    "get_current_principal",
    "set_current_principal",
]
