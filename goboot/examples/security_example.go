//go:build ignore

// Package main demonstrates the security module usage.
package main

import (
	"context"
	"fmt"
	"time"

	"dev.engineeringlabs/goboot/security"
)

func main() {
	fmt.Println("=== Goboot Security Module Example ===\n")
	ctx := context.Background()

	// Example 1: Password hashing
	fmt.Println("1. Password Hashing:")
	hasher := security.NewDefaultHasher()

	password := "supersecret123"
	hash, _ := hasher.Hash(password)
	fmt.Printf("   Password: %s\n", password)
	fmt.Printf("   Hash: %s...\n", hash[:30])

	valid, _ := hasher.Verify(password, hash)
	fmt.Printf("   Verification: %v\n", valid)

	invalid, _ := hasher.Verify("wrongpassword", hash)
	fmt.Printf("   Wrong password: %v\n", invalid)

	// Example 2: Token service
	fmt.Println("\n2. Token Service:")
	tokenService := security.NewInMemoryTokenService(24 * time.Hour)

	principal := &security.Principal{
		ID:       "user-123",
		Username: "johndoe",
		Email:    "john@example.com",
		Roles:    []string{"user", "admin"},
		Attributes: map[string]any{
			"department": "engineering",
		},
	}

	token, _ := tokenService.Generate(ctx, principal)
	fmt.Printf("   Token: %s...\n", token.Value[:20])
	fmt.Printf("   Type: %s\n", token.Type)
	fmt.Printf("   Expires: %v\n", token.ExpiresAt.Format(time.RFC3339))

	// Validate token
	validatedPrincipal, err := tokenService.Validate(ctx, token.Value)
	if err != nil {
		fmt.Printf("   Validation failed: %v\n", err)
	} else {
		fmt.Printf("   Validated user: %s\n", validatedPrincipal.Username)
	}

	// Example 3: Principal role checks
	fmt.Println("\n3. Principal Role Checks:")
	fmt.Printf("   Has 'user' role: %v\n", principal.HasRole("user"))
	fmt.Printf("   Has 'superadmin' role: %v\n", principal.HasRole("superadmin"))
	fmt.Printf("   Has any of ['admin', 'moderator']: %v\n", principal.HasAnyRole("admin", "moderator"))
	fmt.Printf("   Has all of ['user', 'admin']: %v\n", principal.HasAllRoles("user", "admin"))

	// Example 4: RBAC Authorization
	fmt.Println("\n4. RBAC Authorization:")
	authorizer := security.NewRBACAuthorizer()

	// Define roles
	authorizer.AddRole(&security.Role{
		Name: "user",
		Permissions: []security.Permission{
			{Action: "read", Resource: "posts"},
			{Action: "create", Resource: "comments"},
		},
	})

	authorizer.AddRole(&security.Role{
		Name: "admin",
		Permissions: []security.Permission{
			{Action: "*", Resource: "*"},
		},
	})

	// Check permissions
	canReadPosts, _ := authorizer.Authorize(ctx, principal, "read", "posts")
	fmt.Printf("   Can read posts: %v\n", canReadPosts)

	canDeleteUsers, _ := authorizer.Authorize(ctx, principal, "delete", "users")
	fmt.Printf("   Can delete users (admin): %v\n", canDeleteUsers)

	// Example 5: Simple Authenticator
	fmt.Println("\n5. Simple Authenticator:")
	authenticator := security.NewSimpleAuthenticator(hasher)

	// Add user
	authenticator.AddUser(principal, "password123")

	// Authenticate
	credentials := security.Credentials{
		Username: "johndoe",
		Password: "password123",
	}

	authedPrincipal, err := authenticator.Authenticate(ctx, credentials)
	if err != nil {
		fmt.Printf("   Authentication failed: %v\n", err)
	} else {
		fmt.Printf("   Authenticated: %s (%s)\n", authedPrincipal.Username, authedPrincipal.Email)
	}

	// Wrong password
	credentials.Password = "wrongpassword"
	_, err = authenticator.Authenticate(ctx, credentials)
	fmt.Printf("   Wrong password: %v\n", err)

	// Example 6: Secure token generation
	fmt.Println("\n6. Secure Token Generation:")
	secureToken, _ := security.GenerateSecureToken(32)
	fmt.Printf("   Secure token: %s\n", secureToken)
}
