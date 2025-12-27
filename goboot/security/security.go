// Package security provides authentication and authorization for the goboot framework.
//
// This module provides:
//   - API layer: Principal, Token, Authenticator, Authorizer interfaces
//   - Core layer: BcryptHasher, InMemoryTokenService, RBACAuthorizer
//   - SPI layer: UserRepository, TokenStore, AuthenticationProvider interfaces
//
// Example:
//
//	import "dev.engineeringlabs/goboot/security"
//
//	// Password hashing
//	hasher := security.NewDefaultHasher()
//	hash, _ := hasher.Hash("password123")
//	valid, _ := hasher.Verify("password123", hash)
//
//	// Token service
//	tokenService := security.NewInMemoryTokenService(24 * time.Hour)
//	token, _ := tokenService.Generate(ctx, &security.Principal{
//	    ID:       "user-123",
//	    Username: "john",
//	    Roles:    []string{"user"},
//	})
//
//	// RBAC
//	authorizer := security.NewRBACAuthorizer()
//	authorizer.AddRole(&security.Role{
//	    Name: "admin",
//	    Permissions: []security.Permission{
//	        {Action: "*", Resource: "*"},
//	    },
//	})
package security

import (
	"dev.engineeringlabs/goboot/security/api"
	"dev.engineeringlabs/goboot/security/core"
	"dev.engineeringlabs/goboot/security/spi"
)

// Re-export API types
type (
	// Principal represents an authenticated entity.
	Principal = api.Principal
	// Credentials represents authentication credentials.
	Credentials = api.Credentials
	// Token represents an authentication token.
	Token = api.Token
	// Authenticator is the interface for authentication.
	Authenticator = api.Authenticator
	// Authorizer is the interface for authorization.
	Authorizer = api.Authorizer
	// TokenService is the interface for token management.
	TokenService = api.TokenService
	// PasswordHasher is the interface for password hashing.
	PasswordHasher = api.PasswordHasher
	// Permission represents a permission.
	Permission = api.Permission
	// Role represents a role with permissions.
	Role = api.Role
	// SecurityContext holds security-related information.
	SecurityContext = api.SecurityContext
	// ContextKey for security context.
	ContextKey = api.ContextKey
)

// Re-export API constants
const (
	SecurityContextKey = api.SecurityContextKey
	PrincipalKey       = api.PrincipalKey
)

// Re-export API functions
var (
	GetSecurityContext = api.GetSecurityContext
	GetPrincipal       = api.GetPrincipal
)

// Re-export Core types
type (
	// BcryptHasher implements PasswordHasher using bcrypt.
	BcryptHasher = core.BcryptHasher
	// InMemoryTokenService implements TokenService with in-memory storage.
	InMemoryTokenService = core.InMemoryTokenService
	// RBACAuthorizer implements role-based access control.
	RBACAuthorizer = core.RBACAuthorizer
	// SimpleAuthenticator is a basic in-memory authenticator.
	SimpleAuthenticator = core.SimpleAuthenticator
)

// Re-export Core functions
var (
	NewBcryptHasher         = core.NewBcryptHasher
	NewDefaultHasher        = core.NewDefaultHasher
	NewInMemoryTokenService = core.NewInMemoryTokenService
	NewRBACAuthorizer       = core.NewRBACAuthorizer
	NewSimpleAuthenticator  = core.NewSimpleAuthenticator
	ConstantTimeCompare     = core.ConstantTimeCompare
	GenerateSecureToken     = core.GenerateSecureToken
)

// Re-export SPI types
type (
	// UserRepository is the interface for user storage.
	UserRepository = spi.UserRepository
	// TokenStore is the interface for token storage.
	TokenStore = spi.TokenStore
	// PermissionResolver is the interface for resolving permissions.
	PermissionResolver = spi.PermissionResolver
	// AuthenticationProvider is the interface for custom authentication providers.
	AuthenticationProvider = spi.AuthenticationProvider
)
