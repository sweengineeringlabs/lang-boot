//! Secrets management module
//!
//! Secure secret loading, encryption, rotation

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Simple XOR-based encryption (for demonstration - use proper encryption in production)
/// For production, use AES-GCM or similar from the `aes-gcm` crate
const ENCRYPTION_KEY: &[u8] = b"rustboot_secret_key_32_bytes_12";

/// In-memory secret store
#[derive(Debug, Clone)]
pub struct SecretStore {
    secrets: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl SecretStore {
    /// Create a new secret store
    pub fn new() -> Self {
        Self {
            secrets: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Store a secret
    pub fn store(&self, key: &str, value: Vec<u8>) -> crate::SecurityResult<()> {
        let mut secrets = self.secrets.write()
            .map_err(|e| crate::SecurityError::SecretNotFound(format!("Lock error: {}", e)))?;
        
        // Encrypt before storing
        let encrypted = encrypt_secret(&value)?;
        secrets.insert(key.to_string(), encrypted);
        Ok(())
    }
    
    /// Retrieve a secret
    pub fn retrieve(&self, key: &str) -> crate::SecurityResult<Vec<u8>> {
        let secrets = self.secrets.read()
            .map_err(|e| crate::SecurityError::SecretNotFound(format!("Lock error: {}", e)))?;
        
        let encrypted = secrets.get(key)
            .ok_or_else(|| crate::SecurityError::SecretNotFound(format!("Secret '{}' not found", key)))?;
        
        // Decrypt before returning
        decrypt_secret(encrypted)
    }
    
    /// Delete a secret
    pub fn delete(&self, key: &str) -> crate::SecurityResult<()> {
        let mut secrets = self.secrets.write()
            .map_err(|e| crate::SecurityError::SecretNotFound(format!("Lock error: {}", e)))?;
        
        secrets.remove(key)
            .ok_or_else(|| crate::SecurityError::SecretNotFound(format!("Secret '{}' not found", key)))?;
        Ok(())
    }
    
    /// Check if a secret exists
    pub fn exists(&self, key: &str) -> bool {
        self.secrets.read()
            .map(|s| s.contains_key(key))
            .unwrap_or(false)
    }
    
    /// List all secret keys
    pub fn list_keys(&self) -> Vec<String> {
        self.secrets.read()
            .map(|s| s.keys().cloned().collect())
            .unwrap_or_default()
    }
}

impl Default for SecretStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Load secret from environment variable
pub fn load_secret(key: &str) -> crate::SecurityResult<String> {
    std::env::var(key)
        .map_err(|_| crate::SecurityError::SecretNotFound(
            format!("Environment variable '{}' not found", key)
        ))
}

/// Load secret from environment or use default
pub fn load_secret_or_default(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

/// Encrypt secret data
/// 
/// Note: This uses a simple XOR cipher for demonstration.
/// For production, use proper encryption like AES-GCM from the `aes-gcm` crate.
pub fn encrypt_secret(secret: &[u8]) -> crate::SecurityResult<Vec<u8>> {
    if secret.is_empty() {
        return Err(crate::SecurityError::EncryptionError("Cannot encrypt empty data".to_string()));
    }
    
    let encrypted: Vec<u8> = secret.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ ENCRYPTION_KEY[i % ENCRYPTION_KEY.len()])
        .collect();
    
    Ok(encrypted)
}

/// Decrypt secret data
///
/// Note: This uses a simple XOR cipher for demonstration.
/// For production, use proper encryption like AES-GCM from the `aes-gcm` crate.
pub fn decrypt_secret(encrypted: &[u8]) -> crate::SecurityResult<Vec<u8>> {
    if encrypted.is_empty() {
        return Err(crate::SecurityError::EncryptionError("Cannot decrypt empty data".to_string()));
    }
    
    // XOR is symmetric, so encryption and decryption are the same
    let decrypted: Vec<u8> = encrypted.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ ENCRYPTION_KEY[i % ENCRYPTION_KEY.len()])
        .collect();
    
    Ok(decrypted)
}

/// Securely zero out memory
pub fn zero_memory(data: &mut [u8]) {
    for byte in data.iter_mut() {
        // Volatile write to prevent compiler optimization
        unsafe {
            std::ptr::write_volatile(byte, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let secret = b"my_secret_password";
        let encrypted = encrypt_secret(secret).unwrap();
        
        // Encrypted should be different from original
        assert_ne!(encrypted, secret);
        
        let decrypted = decrypt_secret(&encrypted).unwrap();
        assert_eq!(decrypted, secret);
    }

    #[test]
    fn test_secret_store() {
        let store = SecretStore::new();
        
        // Store a secret
        let secret = b"super_secret".to_vec();
        store.store("api_key", secret.clone()).unwrap();
        
        // Check it exists
        assert!(store.exists("api_key"));
        
        // Retrieve it
        let retrieved = store.retrieve("api_key").unwrap();
        assert_eq!(retrieved, secret);
        
        // Delete it
        store.delete("api_key").unwrap();
        assert!(!store.exists("api_key"));
    }

    #[test]
    fn test_secret_not_found() {
        let store = SecretStore::new();
        let result = store.retrieve("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_keys() {
        let store = SecretStore::new();
        store.store("key1", b"value1".to_vec()).unwrap();
        store.store("key2", b"value2".to_vec()).unwrap();
        
        let keys = store.list_keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"key2".to_string()));
    }

    #[test]
    fn test_load_secret_or_default() {
        // Test with non-existent env var
        let secret = load_secret_or_default("NONEXISTENT_VAR_123", "default_value");
        assert_eq!(secret, "default_value");
    }

    #[test]
    fn test_encrypt_empty() {
        let result = encrypt_secret(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_empty() {
        let result = decrypt_secret(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_zero_memory() {
        let mut data = vec![1, 2, 3, 4, 5];
        zero_memory(&mut data);
        assert_eq!(data, vec![0, 0, 0, 0, 0]);
    }
}
