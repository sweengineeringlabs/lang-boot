# Crypto Module Overview

## WHAT: Cryptography Primitives

Secure hashing, encryption, and key management.

Key capabilities:
- **Hashing** - SHA-256, bcrypt, argon2
- **Encryption** - AES-GCM
- **Random** - Secure random generation
- **HMAC** - Message authentication

## WHY: Security Best Practices

**Problems Solved**: Insecure defaults, complexity

**When to Use**: Password hashing, data encryption

## HOW: Usage Guide

```go
// Password hashing
hash, _ := crypto.HashPassword("secret")
valid := crypto.VerifyPassword("secret", hash)

// Encryption
encrypted := crypto.Encrypt(data, key)
decrypted := crypto.Decrypt(encrypted, key)

// Random
token := crypto.RandomToken(32)
```

---

**Status**: Stable
