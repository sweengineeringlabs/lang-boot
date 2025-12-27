// Package core contains the implementation details for the crypto module.
package core

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/hmac"
	"crypto/md5"
	"crypto/rand"
	"crypto/sha1"
	"crypto/sha256"
	"crypto/sha512"
	"encoding/hex"
	"fmt"
	"hash"

	"dev.engineeringlabs/goboot/crypto/api"
)

// DefaultHasher implements common hash algorithms.
type DefaultHasher struct {
	algorithm api.HashAlgorithm
	newHash   func() hash.Hash
}

// NewMD5Hasher creates an MD5 hasher.
func NewMD5Hasher() *DefaultHasher {
	return &DefaultHasher{algorithm: api.HashMD5, newHash: md5.New}
}

// NewSHA1Hasher creates a SHA1 hasher.
func NewSHA1Hasher() *DefaultHasher {
	return &DefaultHasher{algorithm: api.HashSHA1, newHash: sha1.New}
}

// NewSHA256Hasher creates a SHA256 hasher.
func NewSHA256Hasher() *DefaultHasher {
	return &DefaultHasher{algorithm: api.HashSHA256, newHash: sha256.New}
}

// NewSHA512Hasher creates a SHA512 hasher.
func NewSHA512Hasher() *DefaultHasher {
	return &DefaultHasher{algorithm: api.HashSHA512, newHash: sha512.New}
}

// Algorithm returns the hash algorithm.
func (h *DefaultHasher) Algorithm() api.HashAlgorithm {
	return h.algorithm
}

// Hash computes the hash of data.
func (h *DefaultHasher) Hash(data []byte) []byte {
	hasher := h.newHash()
	hasher.Write(data)
	return hasher.Sum(nil)
}

// HashString computes the hash as a hex string.
func (h *DefaultHasher) HashString(data []byte) string {
	return hex.EncodeToString(h.Hash(data))
}

// Verify verifies data against a hash.
func (h *DefaultHasher) Verify(data, expectedHash []byte) bool {
	actual := h.Hash(data)
	return hmac.Equal(actual, expectedHash)
}

// AESGCMEncryptor implements AES-GCM encryption.
type AESGCMEncryptor struct {
	key []byte
}

// NewAESGCMEncryptor creates a new AES-GCM encryptor.
func NewAESGCMEncryptor(key []byte) (*AESGCMEncryptor, error) {
	if len(key) != 16 && len(key) != 24 && len(key) != 32 {
		return nil, fmt.Errorf("key must be 16, 24, or 32 bytes")
	}
	return &AESGCMEncryptor{key: key}, nil
}

// Algorithm returns the encryption algorithm.
func (e *AESGCMEncryptor) Algorithm() api.EncryptionAlgorithm {
	return api.EncryptionAESGCM
}

// Encrypt encrypts plaintext.
func (e *AESGCMEncryptor) Encrypt(plaintext []byte) ([]byte, error) {
	block, err := aes.NewCipher(e.key)
	if err != nil {
		return nil, err
	}

	gcm, err := cipher.NewGCM(block)
	if err != nil {
		return nil, err
	}

	nonce := make([]byte, gcm.NonceSize())
	if _, err := rand.Read(nonce); err != nil {
		return nil, err
	}

	ciphertext := gcm.Seal(nonce, nonce, plaintext, nil)
	return ciphertext, nil
}

// Decrypt decrypts ciphertext.
func (e *AESGCMEncryptor) Decrypt(ciphertext []byte) ([]byte, error) {
	block, err := aes.NewCipher(e.key)
	if err != nil {
		return nil, err
	}

	gcm, err := cipher.NewGCM(block)
	if err != nil {
		return nil, err
	}

	nonceSize := gcm.NonceSize()
	if len(ciphertext) < nonceSize {
		return nil, fmt.Errorf("ciphertext too short")
	}

	nonce, ciphertext := ciphertext[:nonceSize], ciphertext[nonceSize:]
	plaintext, err := gcm.Open(nil, nonce, ciphertext, nil)
	if err != nil {
		return nil, err
	}

	return plaintext, nil
}

// DefaultHMAC implements HMAC operations.
type DefaultHMAC struct {
	newHash func() hash.Hash
}

// NewHMACSHA256 creates an HMAC-SHA256 implementation.
func NewHMACSHA256() *DefaultHMAC {
	return &DefaultHMAC{newHash: sha256.New}
}

// NewHMACSHA512 creates an HMAC-SHA512 implementation.
func NewHMACSHA512() *DefaultHMAC {
	return &DefaultHMAC{newHash: sha512.New}
}

// Sign signs data with a key.
func (h *DefaultHMAC) Sign(data, key []byte) []byte {
	mac := hmac.New(h.newHash, key)
	mac.Write(data)
	return mac.Sum(nil)
}

// Verify verifies a signature.
func (h *DefaultHMAC) Verify(data, signature, key []byte) bool {
	expected := h.Sign(data, key)
	return hmac.Equal(expected, signature)
}

// GenerateRandomBytes generates cryptographically secure random bytes.
func GenerateRandomBytes(n int) ([]byte, error) {
	b := make([]byte, n)
	_, err := rand.Read(b)
	return b, err
}

// GenerateAESKey generates a random AES key.
func GenerateAESKey(bits int) ([]byte, error) {
	if bits != 128 && bits != 192 && bits != 256 {
		return nil, fmt.Errorf("bits must be 128, 192, or 256")
	}
	return GenerateRandomBytes(bits / 8)
}

// Default instances
var (
	MD5    = NewMD5Hasher()
	SHA1   = NewSHA1Hasher()
	SHA256 = NewSHA256Hasher()
	SHA512 = NewSHA512Hasher()
)
