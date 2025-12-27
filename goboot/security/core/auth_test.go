package core

import (
	"context"
	"testing"
	"time"

	"dev.engineeringlabs/goboot/security/api"
)

func TestBcryptHasher_HashAndVerify(t *testing.T) {
	hasher := NewDefaultHasher()

	password := "mysecretpassword123"

	hash, err := hasher.Hash(password)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	if hash == password {
		t.Error("Hash should not equal password")
	}

	// Verify correct password
	valid, err := hasher.Verify(password, hash)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if !valid {
		t.Error("Password should verify correctly")
	}

	// Verify wrong password
	valid, err = hasher.Verify("wrongpassword", hash)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if valid {
		t.Error("Wrong password should not verify")
	}
}

func TestBcryptHasher_DifferentHashes(t *testing.T) {
	hasher := NewDefaultHasher()
	password := "samepassword"

	hash1, _ := hasher.Hash(password)
	hash2, _ := hasher.Hash(password)

	if hash1 == hash2 {
		t.Error("Same password should produce different hashes (salt)")
	}

	// Both should verify
	valid1, _ := hasher.Verify(password, hash1)
	valid2, _ := hasher.Verify(password, hash2)

	if !valid1 || !valid2 {
		t.Error("Both hashes should verify")
	}
}

func TestInMemoryTokenService_GenerateAndValidate(t *testing.T) {
	service := NewInMemoryTokenService(time.Hour)
	ctx := context.Background()

	principal := &api.Principal{
		ID:       "user-123",
		Username: "testuser",
		Email:    "test@example.com",
		Roles:    []string{"user", "admin"},
	}

	token, err := service.Generate(ctx, principal)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	if token.Value == "" {
		t.Error("Token value should not be empty")
	}
	if token.Type != "Bearer" {
		t.Errorf("Expected Bearer type, got %s", token.Type)
	}
	if token.ExpiresAt.Before(time.Now()) {
		t.Error("Token should not be expired")
	}

	// Validate token
	validatedPrincipal, err := service.Validate(ctx, token.Value)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	if validatedPrincipal.ID != principal.ID {
		t.Error("Validated principal should match original")
	}
}

func TestInMemoryTokenService_InvalidToken(t *testing.T) {
	service := NewInMemoryTokenService(time.Hour)
	ctx := context.Background()

	_, err := service.Validate(ctx, "invalid-token")
	if err == nil {
		t.Error("Expected error for invalid token")
	}
}

func TestInMemoryTokenService_ExpiredToken(t *testing.T) {
	service := NewInMemoryTokenService(50 * time.Millisecond)
	ctx := context.Background()

	principal := &api.Principal{ID: "user-123"}
	token, _ := service.Generate(ctx, principal)

	// Wait for expiry
	time.Sleep(100 * time.Millisecond)

	_, err := service.Validate(ctx, token.Value)
	if err == nil {
		t.Error("Expected error for expired token")
	}
}

func TestInMemoryTokenService_Revoke(t *testing.T) {
	service := NewInMemoryTokenService(time.Hour)
	ctx := context.Background()

	principal := &api.Principal{ID: "user-123"}
	token, _ := service.Generate(ctx, principal)

	// Revoke
	service.Revoke(ctx, token.Value)

	// Should not validate
	_, err := service.Validate(ctx, token.Value)
	if err == nil {
		t.Error("Expected error for revoked token")
	}
}

func TestRBACAuthorizer(t *testing.T) {
	authorizer := NewRBACAuthorizer()
	ctx := context.Background()

	// Define roles
	authorizer.AddRole(&api.Role{
		Name: "user",
		Permissions: []api.Permission{
			{Action: "read", Resource: "posts"},
			{Action: "create", Resource: "comments"},
		},
	})

	authorizer.AddRole(&api.Role{
		Name: "admin",
		Permissions: []api.Permission{
			{Action: "*", Resource: "*"},
		},
	})

	t.Run("UserPermissions", func(t *testing.T) {
		principal := &api.Principal{
			ID:    "user-1",
			Roles: []string{"user"},
		}

		// Allowed
		allowed, _ := authorizer.Authorize(ctx, principal, "read", "posts")
		if !allowed {
			t.Error("User should be able to read posts")
		}

		// Not allowed
		allowed, _ = authorizer.Authorize(ctx, principal, "delete", "posts")
		if allowed {
			t.Error("User should not be able to delete posts")
		}
	})

	t.Run("AdminPermissions", func(t *testing.T) {
		principal := &api.Principal{
			ID:    "admin-1",
			Roles: []string{"admin"},
		}

		// Admin can do anything
		allowed, _ := authorizer.Authorize(ctx, principal, "delete", "users")
		if !allowed {
			t.Error("Admin should be able to do anything")
		}
	})

	t.Run("NilPrincipal", func(t *testing.T) {
		allowed, _ := authorizer.Authorize(ctx, nil, "read", "posts")
		if allowed {
			t.Error("Nil principal should not be authorized")
		}
	})
}

func TestPrincipal_RoleChecks(t *testing.T) {
	principal := &api.Principal{
		Roles: []string{"user", "editor", "moderator"},
	}

	t.Run("HasRole", func(t *testing.T) {
		if !principal.HasRole("user") {
			t.Error("Should have user role")
		}
		if principal.HasRole("admin") {
			t.Error("Should not have admin role")
		}
	})

	t.Run("HasAnyRole", func(t *testing.T) {
		if !principal.HasAnyRole("admin", "editor") {
			t.Error("Should have at least one of the roles")
		}
		if principal.HasAnyRole("admin", "superadmin") {
			t.Error("Should not have any of these roles")
		}
	})

	t.Run("HasAllRoles", func(t *testing.T) {
		if !principal.HasAllRoles("user", "editor") {
			t.Error("Should have all these roles")
		}
		if principal.HasAllRoles("user", "admin") {
			t.Error("Should not have all these roles")
		}
	})
}

func TestSimpleAuthenticator(t *testing.T) {
	hasher := NewDefaultHasher()
	auth := NewSimpleAuthenticator(hasher)
	ctx := context.Background()

	principal := &api.Principal{
		ID:       "user-1",
		Username: "testuser",
		Email:    "test@example.com",
		Roles:    []string{"user"},
	}

	// Add user
	err := auth.AddUser(principal, "password123")
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	t.Run("ValidCredentials", func(t *testing.T) {
		creds := api.Credentials{
			Username: "testuser",
			Password: "password123",
		}

		authed, err := auth.Authenticate(ctx, creds)
		if err != nil {
			t.Errorf("Unexpected error: %v", err)
		}
		if authed.ID != principal.ID {
			t.Error("Should return correct principal")
		}
	})

	t.Run("InvalidPassword", func(t *testing.T) {
		creds := api.Credentials{
			Username: "testuser",
			Password: "wrongpassword",
		}

		_, err := auth.Authenticate(ctx, creds)
		if err == nil {
			t.Error("Expected error for invalid password")
		}
	})

	t.Run("UnknownUser", func(t *testing.T) {
		creds := api.Credentials{
			Username: "unknown",
			Password: "password",
		}

		_, err := auth.Authenticate(ctx, creds)
		if err == nil {
			t.Error("Expected error for unknown user")
		}
	})
}

func TestGenerateSecureToken(t *testing.T) {
	token1, err := GenerateSecureToken(32)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	token2, err := GenerateSecureToken(32)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	if token1 == token2 {
		t.Error("Tokens should be unique")
	}

	if len(token1) == 0 {
		t.Error("Token should not be empty")
	}
}
