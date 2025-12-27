// Package spi contains the Service Provider Interface for the cache module.
package spi

import (
	"context"
	"time"
)

// CacheBackend is the interface for cache backend implementations.
//
// Implement this to create custom cache backends like:
//   - Redis
//   - Memcached
//   - Database-backed cache
//
// Example:
//
//	type RedisBackend struct {
//	    client *redis.Client
//	}
//
//	func (b *RedisBackend) Name() string {
//	    return "redis"
//	}
//
//	func (b *RedisBackend) Get(ctx context.Context, key string) ([]byte, bool, error) {
//	    val, err := b.client.Get(ctx, key).Bytes()
//	    if err == redis.Nil {
//	        return nil, false, nil
//	    }
//	    return val, true, err
//	}
type CacheBackend interface {
	// Name returns the backend name.
	Name() string

	// Get retrieves a value from the backend.
	Get(ctx context.Context, key string) ([]byte, bool, error)

	// Set stores a value in the backend.
	Set(ctx context.Context, key string, value []byte, ttl time.Duration) error

	// Delete removes a value from the backend.
	Delete(ctx context.Context, key string) error

	// Close closes the backend connection.
	Close() error
}

// Serializer is the interface for cache value serialization.
type Serializer interface {
	// Serialize converts a value to bytes.
	Serialize(value any) ([]byte, error)

	// Deserialize converts bytes back to a value.
	Deserialize(data []byte, target any) error
}
