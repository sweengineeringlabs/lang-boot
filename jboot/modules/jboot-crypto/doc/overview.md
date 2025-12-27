# Crypto Module Overview

## WHAT: Cryptography Primitives

Secure cryptographic operations including hashing, encryption, and key management.

Key capabilities:
- **Hashing** - SHA-256, SHA-512, bcrypt, argon2
- **Encryption** - AES-GCM, ChaCha20-Poly1305
- **Key Derivation** - PBKDF2, scrypt
- **Random** - Secure random generation

## WHY: Security Best Practices

**Problems Solved**:
1. **Insecure Defaults** - Safe defaults for all operations
2. **Complexity** - Simple API for complex operations
3. **Key Management** - Secure key handling

**When to Use**: Password hashing, data encryption, secure tokens

## HOW: Usage Guide

```java
// Password hashing
String hash = Crypto.hashPassword("secret");
boolean valid = Crypto.verifyPassword("secret", hash);

// Encryption
byte[] encrypted = Crypto.encrypt(data, key);
byte[] decrypted = Crypto.decrypt(encrypted, key);

// Secure random
String token = Crypto.randomToken(32);
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-security | Password and token handling |

---

**Status**: Stable
