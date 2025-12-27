// Package api contains the public interfaces and types for the security module.
package api

import (
	"context"
	"time"
)

// Principal represents an authenticated entity.
type Principal struct {
	ID         string
	Username   string
	Email      string
	Roles      []string
	Attributes map[string]any
}

// HasRole checks if the principal has a specific role.
func (p *Principal) HasRole(role string) bool {
	for _, r := range p.Roles {
		if r == role {
			return true
		}
	}
	return false
}

// HasAnyRole checks if the principal has any of the specified roles.
func (p *Principal) HasAnyRole(roles ...string) bool {
	for _, role := range roles {
		if p.HasRole(role) {
			return true
		}
	}
	return false
}

// HasAllRoles checks if the principal has all of the specified roles.
func (p *Principal) HasAllRoles(roles ...string) bool {
	for _, role := range roles {
		if !p.HasRole(role) {
			return false
		}
	}
	return true
}

// Credentials represents authentication credentials.
type Credentials struct {
	Username string
	Password string
	Token    string
}

// Token represents an authentication token.
type Token struct {
	Value     string
	Type      string
	ExpiresAt time.Time
	Claims    map[string]any
}

// IsExpired returns true if the token has expired.
func (t *Token) IsExpired() bool {
	return time.Now().After(t.ExpiresAt)
}

// Authenticator is the interface for authentication.
type Authenticator interface {
	// Authenticate validates credentials and returns a principal.
	Authenticate(ctx context.Context, credentials Credentials) (*Principal, error)
}

// Authorizer is the interface for authorization.
type Authorizer interface {
	// Authorize checks if a principal is authorized for an action.
	Authorize(ctx context.Context, principal *Principal, action string, resource string) (bool, error)
}

// TokenService is the interface for token management.
type TokenService interface {
	// Generate creates a new token for a principal.
	Generate(ctx context.Context, principal *Principal) (*Token, error)

	// Validate validates a token and returns the principal.
	Validate(ctx context.Context, tokenValue string) (*Principal, error)

	// Revoke revokes a token.
	Revoke(ctx context.Context, tokenValue string) error
}

// PasswordHasher is the interface for password hashing.
type PasswordHasher interface {
	// Hash hashes a password.
	Hash(password string) (string, error)

	// Verify verifies a password against a hash.
	Verify(password, hash string) (bool, error)
}

// Permission represents a permission.
type Permission struct {
	Action   string
	Resource string
}

// Role represents a role with permissions.
type Role struct {
	Name        string
	Permissions []Permission
}

// HasPermission checks if a role has a specific permission.
func (r *Role) HasPermission(action, resource string) bool {
	for _, p := range r.Permissions {
		if p.Action == action && (p.Resource == resource || p.Resource == "*") {
			return true
		}
		if p.Action == "*" && (p.Resource == resource || p.Resource == "*") {
			return true
		}
	}
	return false
}

// SecurityContext holds security-related information for a request.
type SecurityContext struct {
	Principal   *Principal
	Token       *Token
	Permissions []Permission
}

// ContextKey for security context.
type ContextKey string

const (
	// SecurityContextKey is the context key for security context.
	SecurityContextKey ContextKey = "security_context"
	// PrincipalKey is the context key for principal.
	PrincipalKey ContextKey = "principal"
)

// GetSecurityContext retrieves the security context from a standard context.
func GetSecurityContext(ctx context.Context) (*SecurityContext, bool) {
	sc, ok := ctx.Value(SecurityContextKey).(*SecurityContext)
	return sc, ok
}

// GetPrincipal retrieves the principal from a standard context.
func GetPrincipal(ctx context.Context) (*Principal, bool) {
	p, ok := ctx.Value(PrincipalKey).(*Principal)
	return p, ok
}
