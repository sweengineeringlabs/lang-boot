// Package api contains the public interfaces and types for the crypto module.
package api

// HashAlgorithm represents a hash algorithm.
type HashAlgorithm string

const (
	// MD5 hash algorithm.
	HashMD5 HashAlgorithm = "md5"
	// SHA1 hash algorithm.
	HashSHA1 HashAlgorithm = "sha1"
	// SHA256 hash algorithm.
	HashSHA256 HashAlgorithm = "sha256"
	// SHA512 hash algorithm.
	HashSHA512 HashAlgorithm = "sha512"
)

// EncryptionAlgorithm represents an encryption algorithm.
type EncryptionAlgorithm string

const (
	// AES-GCM encryption algorithm.
	EncryptionAESGCM EncryptionAlgorithm = "aes-gcm"
	// ChaCha20-Poly1305 encryption algorithm.
	EncryptionChaCha20 EncryptionAlgorithm = "chacha20-poly1305"
)

// Hasher is the interface for hash functions.
type Hasher interface {
	// Algorithm returns the hash algorithm.
	Algorithm() HashAlgorithm

	// Hash computes the hash of data.
	Hash(data []byte) []byte

	// HashString computes the hash as a hex string.
	HashString(data []byte) string

	// Verify verifies data against a hash.
	Verify(data, hash []byte) bool
}

// Encryptor is the interface for encryption.
type Encryptor interface {
	// Algorithm returns the encryption algorithm.
	Algorithm() EncryptionAlgorithm

	// Encrypt encrypts plaintext.
	Encrypt(plaintext []byte) ([]byte, error)

	// Decrypt decrypts ciphertext.
	Decrypt(ciphertext []byte) ([]byte, error)
}

// KeyGenerator is the interface for key generation.
type KeyGenerator interface {
	// GenerateKey generates a new key.
	GenerateKey() ([]byte, error)

	// GenerateKeyPair generates a new key pair.
	GenerateKeyPair() (publicKey, privateKey []byte, err error)
}

// HMAC is the interface for HMAC operations.
type HMAC interface {
	// Sign signs data with a key.
	Sign(data, key []byte) []byte

	// Verify verifies a signature.
	Verify(data, signature, key []byte) bool
}

// EncryptedData represents encrypted data with metadata.
type EncryptedData struct {
	Ciphertext []byte            `json:"ciphertext"`
	Nonce      []byte            `json:"nonce"`
	Algorithm  EncryptionAlgorithm `json:"algorithm"`
}
