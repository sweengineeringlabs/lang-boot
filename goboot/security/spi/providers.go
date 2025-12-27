// Package spi contains the Service Provider Interface for the security module.
package spi

import (
	"context"

	"dev.engineeringlabs/goboot/security/api"
)

// UserRepository is the interface for user storage.
//
// Implement this to integrate with your user database.
//
// Example:
//
//	type PostgresUserRepository struct {
//	    db *sql.DB
//	}
//
//	func (r *PostgresUserRepository) FindByUsername(ctx context.Context, username string) (*api.Principal, error) {
//	    // Query database
//	}
type UserRepository interface {
	// FindByID finds a user by ID.
	FindByID(ctx context.Context, id string) (*api.Principal, error)

	// FindByUsername finds a user by username.
	FindByUsername(ctx context.Context, username string) (*api.Principal, error)

	// FindByEmail finds a user by email.
	FindByEmail(ctx context.Context, email string) (*api.Principal, error)

	// GetPasswordHash retrieves the password hash for a user.
	GetPasswordHash(ctx context.Context, userID string) (string, error)
}

// TokenStore is the interface for token storage.
//
// Implement this for persistent or distributed token storage.
//
// Example:
//
//	type RedisTokenStore struct {
//	    client *redis.Client
//	}
//
//	func (s *RedisTokenStore) Store(ctx context.Context, token *api.Token, principal *api.Principal) error {
//	    // Store in Redis
//	}
type TokenStore interface {
	// Store stores a token.
	Store(ctx context.Context, token *api.Token, principal *api.Principal) error

	// Get retrieves a token.
	Get(ctx context.Context, tokenValue string) (*api.Token, *api.Principal, error)

	// Delete deletes a token.
	Delete(ctx context.Context, tokenValue string) error

	// DeleteByPrincipal deletes all tokens for a principal.
	DeleteByPrincipal(ctx context.Context, principalID string) error
}

// PermissionResolver is the interface for resolving permissions.
//
// Implement this for dynamic permission resolution.
type PermissionResolver interface {
	// ResolvePermissions resolves permissions for a principal.
	ResolvePermissions(ctx context.Context, principal *api.Principal) ([]api.Permission, error)
}

// AuthenticationProvider is the interface for custom authentication providers.
//
// Implement this for OAuth, LDAP, SAML, etc.
type AuthenticationProvider interface {
	// Name returns the provider name.
	Name() string

	// Authenticate authenticates using this provider.
	Authenticate(ctx context.Context, credentials api.Credentials) (*api.Principal, error)

	// Supports checks if this provider supports the given credentials.
	Supports(credentials api.Credentials) bool
}
