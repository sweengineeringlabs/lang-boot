//! Password hashing utilities

use crate::error::CryptoResult;

/// Hash a password using bcrypt
pub fn hash_password(password: &str) -> CryptoResult<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(Into::into)
}

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> CryptoResult<bool> {
    bcrypt::verify(password, hash).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let password = "my_secure_password_123";
        let hash = hash_password(password).unwrap();
        
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_hash_different_each_time() {
        let password = "test";
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();
        
        // Hashes should be different due to salt
        assert_ne!(hash1, hash2);
        
        // But both should verify
        assert!(verify_password(password, &hash1).unwrap());
        assert!(verify_password(password, &hash2).unwrap());
    }
}
