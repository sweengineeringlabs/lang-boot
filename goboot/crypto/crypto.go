// Package crypto provides cryptographic utilities for the goboot framework.
//
// This module provides:
//   - API layer: HashAlgorithm, Hasher, Encryptor interfaces
//   - Core layer: DefaultHasher, AESGCMEncryptor, HMAC implementations
//
// Example:
//
//	import "dev.engineeringlabs/goboot/crypto"
//
//	// Hashing
//	hash := crypto.SHA256.HashString([]byte("data"))
//
//	// Encryption
//	key, _ := crypto.GenerateAESKey(256)
//	encryptor, _ := crypto.NewAESGCMEncryptor(key)
//	ciphertext, _ := encryptor.Encrypt([]byte("secret message"))
//	plaintext, _ := encryptor.Decrypt(ciphertext)
//
//	// HMAC
//	hmac := crypto.NewHMACSHA256()
//	signature := hmac.Sign([]byte("data"), key)
//	valid := hmac.Verify([]byte("data"), signature, key)
package crypto

import (
	"dev.engineeringlabs/goboot/crypto/api"
	"dev.engineeringlabs/goboot/crypto/core"
)

// Re-export API types
type (
	// HashAlgorithm represents a hash algorithm.
	HashAlgorithm = api.HashAlgorithm
	// EncryptionAlgorithm represents an encryption algorithm.
	EncryptionAlgorithm = api.EncryptionAlgorithm
	// Hasher is the interface for hash functions.
	Hasher = api.Hasher
	// Encryptor is the interface for encryption.
	Encryptor = api.Encryptor
	// KeyGenerator is the interface for key generation.
	KeyGenerator = api.KeyGenerator
	// HMAC is the interface for HMAC operations.
	HMAC = api.HMAC
	// EncryptedData represents encrypted data with metadata.
	EncryptedData = api.EncryptedData
)

// Re-export API constants
const (
	HashMD5            = api.HashMD5
	HashSHA1           = api.HashSHA1
	HashSHA256         = api.HashSHA256
	HashSHA512         = api.HashSHA512
	EncryptionAESGCM   = api.EncryptionAESGCM
	EncryptionChaCha20 = api.EncryptionChaCha20
)

// Re-export Core types
type (
	// DefaultHasher implements common hash algorithms.
	DefaultHasher = core.DefaultHasher
	// AESGCMEncryptor implements AES-GCM encryption.
	AESGCMEncryptor = core.AESGCMEncryptor
	// DefaultHMAC implements HMAC operations.
	DefaultHMAC = core.DefaultHMAC
)

// Re-export Core functions
var (
	NewMD5Hasher        = core.NewMD5Hasher
	NewSHA1Hasher       = core.NewSHA1Hasher
	NewSHA256Hasher     = core.NewSHA256Hasher
	NewSHA512Hasher     = core.NewSHA512Hasher
	NewAESGCMEncryptor  = core.NewAESGCMEncryptor
	NewHMACSHA256       = core.NewHMACSHA256
	NewHMACSHA512       = core.NewHMACSHA512
	GenerateRandomBytes = core.GenerateRandomBytes
	GenerateAESKey      = core.GenerateAESKey
)

// Default hashers
var (
	MD5    = core.MD5
	SHA1   = core.SHA1
	SHA256 = core.SHA256
	SHA512 = core.SHA512
)
