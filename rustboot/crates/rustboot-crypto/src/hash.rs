//! Hashing utilities

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

/// Compute SHA256 hash of data
pub fn sha256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Compute HMAC-SHA256 of data with key
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key)
        .expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let hash = sha256(b"hello world");
        assert_eq!(hash.len(), 32); // SHA256 is 256 bits = 32 bytes
    }

    #[test]
    fn test_sha256_deterministic() {
        let hash1 = sha256(b"test");
        let hash2 = sha256(b"test");
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hmac_sha256() {
        let mac = hmac_sha256(b"secret-key", b"message");
        assert_eq!(mac.len(), 32);
    }

    #[test]
    fn test_hmac_different_keys() {
        let mac1 = hmac_sha256(b"key1", b"data");
        let mac2 = hmac_sha256(b"key2", b"data");
        assert_ne!(mac1, mac2);
    }
}
