//! Rustboot Crypto - Cryptography utilities

pub mod error;
pub mod hash;
pub mod password;

pub use error::{CryptoError, CryptoResult};
pub use hash::{hmac_sha256, sha256};
pub use password::{hash_password, verify_password};
