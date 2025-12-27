package core

import (
	"testing"
)

func TestHashers(t *testing.T) {
	data := []byte("hello world")

	t.Run("MD5", func(t *testing.T) {
		hasher := NewMD5Hasher()
		hash := hasher.Hash(data)
		if len(hash) != 16 {
			t.Errorf("MD5 should be 16 bytes, got %d", len(hash))
		}
		
		hashStr := hasher.HashString(data)
		if len(hashStr) != 32 {
			t.Errorf("MD5 hex should be 32 chars, got %d", len(hashStr))
		}
	})

	t.Run("SHA256", func(t *testing.T) {
		hasher := NewSHA256Hasher()
		hash := hasher.Hash(data)
		if len(hash) != 32 {
			t.Errorf("SHA256 should be 32 bytes, got %d", len(hash))
		}
	})

	t.Run("SHA512", func(t *testing.T) {
		hasher := NewSHA512Hasher()
		hash := hasher.Hash(data)
		if len(hash) != 64 {
			t.Errorf("SHA512 should be 64 bytes, got %d", len(hash))
		}
	})

	t.Run("Verify", func(t *testing.T) {
		hasher := NewSHA256Hasher()
		hash := hasher.Hash(data)
		
		if !hasher.Verify(data, hash) {
			t.Error("Verify should return true for correct data")
		}
		
		if hasher.Verify([]byte("different"), hash) {
			t.Error("Verify should return false for different data")
		}
	})
}

func TestAESGCMEncryptor(t *testing.T) {
	key, err := GenerateAESKey(256)
	if err != nil {
		t.Fatalf("Failed to generate key: %v", err)
	}

	encryptor, err := NewAESGCMEncryptor(key)
	if err != nil {
		t.Fatalf("Failed to create encryptor: %v", err)
	}

	plaintext := []byte("secret message")

	t.Run("EncryptDecrypt", func(t *testing.T) {
		ciphertext, err := encryptor.Encrypt(plaintext)
		if err != nil {
			t.Fatalf("Encrypt failed: %v", err)
		}

		if string(ciphertext) == string(plaintext) {
			t.Error("Ciphertext should differ from plaintext")
		}

		decrypted, err := encryptor.Decrypt(ciphertext)
		if err != nil {
			t.Fatalf("Decrypt failed: %v", err)
		}

		if string(decrypted) != string(plaintext) {
			t.Error("Decrypted text should match original")
		}
	})

	t.Run("DifferentCiphertexts", func(t *testing.T) {
		c1, _ := encryptor.Encrypt(plaintext)
		c2, _ := encryptor.Encrypt(plaintext)

		if string(c1) == string(c2) {
			t.Error("Same plaintext should produce different ciphertexts (due to random nonce)")
		}
	})
}

func TestAESGCMEncryptor_InvalidKey(t *testing.T) {
	_, err := NewAESGCMEncryptor([]byte("short"))
	if err == nil {
		t.Error("Expected error for invalid key length")
	}
}

func TestHMAC(t *testing.T) {
	hmac := NewHMACSHA256()
	key := []byte("secret-key")
	data := []byte("message")

	t.Run("Sign", func(t *testing.T) {
		signature := hmac.Sign(data, key)
		if len(signature) == 0 {
			t.Error("Signature should not be empty")
		}
	})

	t.Run("Verify", func(t *testing.T) {
		signature := hmac.Sign(data, key)
		
		if !hmac.Verify(data, signature, key) {
			t.Error("Verify should return true for valid signature")
		}

		if hmac.Verify([]byte("different"), signature, key) {
			t.Error("Verify should return false for different data")
		}

		if hmac.Verify(data, signature, []byte("wrong-key")) {
			t.Error("Verify should return false for wrong key")
		}
	})
}

func TestGenerateRandomBytes(t *testing.T) {
	bytes1, err := GenerateRandomBytes(32)
	if err != nil {
		t.Fatalf("Failed to generate bytes: %v", err)
	}

	bytes2, err := GenerateRandomBytes(32)
	if err != nil {
		t.Fatalf("Failed to generate bytes: %v", err)
	}

	if string(bytes1) == string(bytes2) {
		t.Error("Random bytes should be different")
	}
}

func TestGenerateAESKey(t *testing.T) {
	t.Run("128bit", func(t *testing.T) {
		key, err := GenerateAESKey(128)
		if err != nil {
			t.Fatalf("Failed: %v", err)
		}
		if len(key) != 16 {
			t.Errorf("Expected 16 bytes, got %d", len(key))
		}
	})

	t.Run("256bit", func(t *testing.T) {
		key, err := GenerateAESKey(256)
		if err != nil {
			t.Fatalf("Failed: %v", err)
		}
		if len(key) != 32 {
			t.Errorf("Expected 32 bytes, got %d", len(key))
		}
	})

	t.Run("InvalidBits", func(t *testing.T) {
		_, err := GenerateAESKey(64)
		if err == nil {
			t.Error("Expected error for invalid bit size")
		}
	})
}
