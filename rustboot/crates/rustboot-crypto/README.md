# rustboot-crypto

Cryptography: hashing and passwords.

```rust
use dev_engineeringlabs_rustboot_crypto::*;

// SHA256 hash
let hash = sha256(b"data");

// HMAC
let mac = hmac_sha256(b"key", b"message");

// Password hashing
let hash = hash_password("secret")?;
verify_password("secret", &hash)?;
```

See [overview](docs/overview.md).
