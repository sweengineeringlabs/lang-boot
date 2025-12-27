// Package core contains the implementation details for the security module.
package core

import (
	"context"
	"crypto/rand"
	"crypto/subtle"
	"encoding/base64"
	"fmt"
	"sync"
	"time"

	"dev.engineeringlabs/goboot/security/api"
	"golang.org/x/crypto/bcrypt"
)

// BcryptHasher implements PasswordHasher using bcrypt.
type BcryptHasher struct {
	cost int
}

// NewBcryptHasher creates a new BcryptHasher.
func NewBcryptHasher(cost int) *BcryptHasher {
	if cost < bcrypt.MinCost {
		cost = bcrypt.DefaultCost
	}
	return &BcryptHasher{cost: cost}
}

// NewDefaultHasher creates a BcryptHasher with default cost.
func NewDefaultHasher() *BcryptHasher {
	return NewBcryptHasher(bcrypt.DefaultCost)
}

// Hash hashes a password using bcrypt.
func (h *BcryptHasher) Hash(password string) (string, error) {
	hash, err := bcrypt.GenerateFromPassword([]byte(password), h.cost)
	if err != nil {
		return "", fmt.Errorf("failed to hash password: %w", err)
	}
	return string(hash), nil
}

// Verify verifies a password against a bcrypt hash.
func (h *BcryptHasher) Verify(password, hash string) (bool, error) {
	err := bcrypt.CompareHashAndPassword([]byte(hash), []byte(password))
	if err == bcrypt.ErrMismatchedHashAndPassword {
		return false, nil
	}
	if err != nil {
		return false, fmt.Errorf("failed to verify password: %w", err)
	}
	return true, nil
}

// InMemoryTokenService implements TokenService with in-memory storage.
type InMemoryTokenService struct {
	tokens     map[string]*tokenEntry
	expiration time.Duration
	mu         sync.RWMutex
}

type tokenEntry struct {
	principal *api.Principal
	expiresAt time.Time
}

// NewInMemoryTokenService creates a new InMemoryTokenService.
func NewInMemoryTokenService(expiration time.Duration) *InMemoryTokenService {
	return &InMemoryTokenService{
		tokens:     make(map[string]*tokenEntry),
		expiration: expiration,
	}
}

// Generate creates a new token for a principal.
func (s *InMemoryTokenService) Generate(ctx context.Context, principal *api.Principal) (*api.Token, error) {
	tokenBytes := make([]byte, 32)
	if _, err := rand.Read(tokenBytes); err != nil {
		return nil, fmt.Errorf("failed to generate token: %w", err)
	}

	tokenValue := base64.URLEncoding.EncodeToString(tokenBytes)
	expiresAt := time.Now().Add(s.expiration)

	s.mu.Lock()
	s.tokens[tokenValue] = &tokenEntry{
		principal: principal,
		expiresAt: expiresAt,
	}
	s.mu.Unlock()

	return &api.Token{
		Value:     tokenValue,
		Type:      "Bearer",
		ExpiresAt: expiresAt,
		Claims: map[string]any{
			"sub":   principal.ID,
			"name":  principal.Username,
			"roles": principal.Roles,
		},
	}, nil
}

// Validate validates a token and returns the principal.
func (s *InMemoryTokenService) Validate(ctx context.Context, tokenValue string) (*api.Principal, error) {
	s.mu.RLock()
	entry, ok := s.tokens[tokenValue]
	s.mu.RUnlock()

	if !ok {
		return nil, fmt.Errorf("token not found")
	}

	if time.Now().After(entry.expiresAt) {
		s.Revoke(ctx, tokenValue)
		return nil, fmt.Errorf("token expired")
	}

	return entry.principal, nil
}

// Revoke revokes a token.
func (s *InMemoryTokenService) Revoke(ctx context.Context, tokenValue string) error {
	s.mu.Lock()
	delete(s.tokens, tokenValue)
	s.mu.Unlock()
	return nil
}

// RBACAuthorizer implements role-based access control.
type RBACAuthorizer struct {
	roles map[string]*api.Role
	mu    sync.RWMutex
}

// NewRBACAuthorizer creates a new RBACAuthorizer.
func NewRBACAuthorizer() *RBACAuthorizer {
	return &RBACAuthorizer{
		roles: make(map[string]*api.Role),
	}
}

// AddRole adds a role.
func (a *RBACAuthorizer) AddRole(role *api.Role) {
	a.mu.Lock()
	a.roles[role.Name] = role
	a.mu.Unlock()
}

// Authorize checks if a principal is authorized for an action.
func (a *RBACAuthorizer) Authorize(ctx context.Context, principal *api.Principal, action string, resource string) (bool, error) {
	if principal == nil {
		return false, nil
	}

	a.mu.RLock()
	defer a.mu.RUnlock()

	for _, roleName := range principal.Roles {
		role, ok := a.roles[roleName]
		if ok && role.HasPermission(action, resource) {
			return true, nil
		}
	}

	return false, nil
}

// ConstantTimeCompare performs a constant-time string comparison.
func ConstantTimeCompare(a, b string) bool {
	return subtle.ConstantTimeCompare([]byte(a), []byte(b)) == 1
}

// GenerateSecureToken generates a cryptographically secure random token.
func GenerateSecureToken(length int) (string, error) {
	bytes := make([]byte, length)
	if _, err := rand.Read(bytes); err != nil {
		return "", err
	}
	return base64.URLEncoding.EncodeToString(bytes), nil
}

// SimpleAuthenticator is a basic in-memory authenticator for testing.
type SimpleAuthenticator struct {
	users  map[string]*userEntry
	hasher api.PasswordHasher
	mu     sync.RWMutex
}

type userEntry struct {
	principal    *api.Principal
	passwordHash string
}

// NewSimpleAuthenticator creates a new SimpleAuthenticator.
func NewSimpleAuthenticator(hasher api.PasswordHasher) *SimpleAuthenticator {
	return &SimpleAuthenticator{
		users:  make(map[string]*userEntry),
		hasher: hasher,
	}
}

// AddUser adds a user.
func (a *SimpleAuthenticator) AddUser(principal *api.Principal, password string) error {
	hash, err := a.hasher.Hash(password)
	if err != nil {
		return err
	}

	a.mu.Lock()
	a.users[principal.Username] = &userEntry{
		principal:    principal,
		passwordHash: hash,
	}
	a.mu.Unlock()

	return nil
}

// Authenticate validates credentials and returns a principal.
func (a *SimpleAuthenticator) Authenticate(ctx context.Context, credentials api.Credentials) (*api.Principal, error) {
	a.mu.RLock()
	entry, ok := a.users[credentials.Username]
	a.mu.RUnlock()

	if !ok {
		return nil, fmt.Errorf("user not found")
	}

	valid, err := a.hasher.Verify(credentials.Password, entry.passwordHash)
	if err != nil {
		return nil, err
	}
	if !valid {
		return nil, fmt.Errorf("invalid password")
	}

	return entry.principal, nil
}
