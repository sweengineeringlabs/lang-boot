package core

import (
	"context"
	"sync"
	"testing"
	"time"

	"dev.engineeringlabs/goboot/security/api"
)

// Integration tests for security module

func TestAuthentication_FullFlow(t *testing.T) {
	hasher := NewDefaultHasher()
	auth := NewSimpleAuthenticator(hasher)
	tokenService := NewInMemoryTokenService(time.Hour)
	ctx := context.Background()

	// Create users
	users := []*api.Principal{
		{ID: "1", Username: "admin", Email: "admin@example.com", Roles: []string{"admin"}},
		{ID: "2", Username: "editor", Email: "editor@example.com", Roles: []string{"editor"}},
		{ID: "3", Username: "user", Email: "user@example.com", Roles: []string{"user"}},
	}

	passwords := map[string]string{
		"admin":  "admin123",
		"editor": "editor123",
		"user":   "user123",
	}

	// Register users
	for _, user := range users {
		err := auth.AddUser(user, passwords[user.Username])
		if err != nil {
			t.Fatalf("Failed to add user: %v", err)
		}
	}

	// Authenticate and get tokens
	for _, user := range users {
		t.Run("Auth_"+user.Username, func(t *testing.T) {
			// Authenticate
			principal, err := auth.Authenticate(ctx, api.Credentials{
				Username: user.Username,
				Password: passwords[user.Username],
			})
			if err != nil {
				t.Errorf("Auth failed: %v", err)
				return
			}

			// Get token
			token, err := tokenService.Generate(ctx, principal)
			if err != nil {
				t.Errorf("Token generation failed: %v", err)
				return
			}

			// Validate token
			validated, err := tokenService.Validate(ctx, token.Value)
			if err != nil {
				t.Errorf("Token validation failed: %v", err)
				return
			}

			if validated.ID != user.ID {
				t.Errorf("Expected ID %s, got %s", user.ID, validated.ID)
			}
		})
	}
}

func TestAuthorization_RBAC(t *testing.T) {
	authorizer := NewRBACAuthorizer()
	ctx := context.Background()

	// Define roles with permissions
	authorizer.AddRole(&api.Role{
		Name: "viewer",
		Permissions: []api.Permission{
			{Action: "read", Resource: "posts"},
			{Action: "read", Resource: "comments"},
		},
	})

	authorizer.AddRole(&api.Role{
		Name: "editor",
		Permissions: []api.Permission{
			{Action: "read", Resource: "posts"},
			{Action: "write", Resource: "posts"},
			{Action: "read", Resource: "comments"},
			{Action: "write", Resource: "comments"},
		},
	})

	authorizer.AddRole(&api.Role{
		Name: "admin",
		Permissions: []api.Permission{
			{Action: "*", Resource: "*"},
		},
	})

	tests := []struct {
		name     string
		roles    []string
		action   string
		resource string
		allowed  bool
	}{
		{"viewer_read_posts", []string{"viewer"}, "read", "posts", true},
		{"viewer_write_posts", []string{"viewer"}, "write", "posts", false},
		{"editor_write_posts", []string{"editor"}, "write", "posts", true},
		{"editor_delete_posts", []string{"editor"}, "delete", "posts", false},
		{"admin_delete_posts", []string{"admin"}, "delete", "posts", true},
		{"admin_anything", []string{"admin"}, "nuclear", "launch-codes", true},
		{"multi_role", []string{"viewer", "editor"}, "write", "posts", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			principal := &api.Principal{ID: "test", Roles: tt.roles}
			allowed, _ := authorizer.Authorize(ctx, principal, tt.action, tt.resource)
			if allowed != tt.allowed {
				t.Errorf("Expected allowed=%v, got %v", tt.allowed, allowed)
			}
		})
	}
}

func TestToken_Expiry(t *testing.T) {
	// Short-lived token
	tokenService := NewInMemoryTokenService(100 * time.Millisecond)
	ctx := context.Background()

	principal := &api.Principal{ID: "1", Username: "test"}
	token, _ := tokenService.Generate(ctx, principal)

	// Should be valid initially
	_, err := tokenService.Validate(ctx, token.Value)
	if err != nil {
		t.Error("Token should be valid initially")
	}

	// Wait for expiry
	time.Sleep(150 * time.Millisecond)

	// Should be expired now
	_, err = tokenService.Validate(ctx, token.Value)
	if err == nil {
		t.Error("Token should be expired")
	}
}

func TestToken_Revocation(t *testing.T) {
	tokenService := NewInMemoryTokenService(time.Hour)
	ctx := context.Background()

	principal := &api.Principal{ID: "1", Username: "test"}
	token, _ := tokenService.Generate(ctx, principal)

	// Valid before revocation
	_, err := tokenService.Validate(ctx, token.Value)
	if err != nil {
		t.Error("Token should be valid before revocation")
	}

	// Revoke
	tokenService.Revoke(ctx, token.Value)

	// Invalid after revocation
	_, err = tokenService.Validate(ctx, token.Value)
	if err == nil {
		t.Error("Token should be invalid after revocation")
	}
}

func TestToken_MultipleTokens(t *testing.T) {
	tokenService := NewInMemoryTokenService(time.Hour)
	ctx := context.Background()

	principal := &api.Principal{ID: "1", Username: "test"}

	// Generate multiple tokens
	token1, _ := tokenService.Generate(ctx, principal)
	token2, _ := tokenService.Generate(ctx, principal)
	token3, _ := tokenService.Generate(ctx, principal)

	// All should be valid
	for _, token := range []string{token1.Value, token2.Value, token3.Value} {
		_, err := tokenService.Validate(ctx, token)
		if err != nil {
			t.Errorf("Token should be valid: %s", token)
		}
	}

	// Revoke one
	tokenService.Revoke(ctx, token2.Value)

	// token1 and token3 should still be valid
	_, err := tokenService.Validate(ctx, token1.Value)
	if err != nil {
		t.Error("token1 should still be valid")
	}
	_, err = tokenService.Validate(ctx, token3.Value)
	if err != nil {
		t.Error("token3 should still be valid")
	}

	// token2 should be invalid
	_, err = tokenService.Validate(ctx, token2.Value)
	if err == nil {
		t.Error("token2 should be invalid")
	}
}

func TestHasher_Strength(t *testing.T) {
	hasher := NewDefaultHasher()

	// Long password
	longPassword := "ThisIsAVeryLongPasswordThatShouldStillWorkCorrectly123!@#"
	hash, err := hasher.Hash(longPassword)
	if err != nil {
		t.Errorf("Failed to hash long password: %v", err)
	}

	valid, _ := hasher.Verify(longPassword, hash)
	if !valid {
		t.Error("Long password should verify")
	}

	// Password with special characters
	specialPassword := "P@$$w0rd!#$%^&*()"
	hash, err = hasher.Hash(specialPassword)
	if err != nil {
		t.Errorf("Failed to hash special password: %v", err)
	}

	valid, _ = hasher.Verify(specialPassword, hash)
	if !valid {
		t.Error("Special characters password should verify")
	}

	// Unicode password
	unicodePassword := "密码Password密码"
	hash, err = hasher.Hash(unicodePassword)
	if err != nil {
		t.Errorf("Failed to hash unicode password: %v", err)
	}

	valid, _ = hasher.Verify(unicodePassword, hash)
	if !valid {
		t.Error("Unicode password should verify")
	}
}

func TestHasher_ConcurrentHashing(t *testing.T) {
	hasher := NewDefaultHasher()
	var wg sync.WaitGroup

	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func(n int) {
			defer wg.Done()
			password := "password" + string(rune('0'+n))
			hash, err := hasher.Hash(password)
			if err != nil {
				t.Errorf("Concurrent hash failed: %v", err)
				return
			}

			valid, _ := hasher.Verify(password, hash)
			if !valid {
				t.Error("Concurrent verification failed")
			}
		}(i)
	}

	wg.Wait()
}

func TestPrincipal_Attributes(t *testing.T) {
	principal := &api.Principal{
		ID:       "123",
		Username: "john",
		Email:    "john@example.com",
		Roles:    []string{"user", "editor", "reviewer"},
		Attributes: map[string]any{
			"department": "engineering",
			"level":      5,
		},
	}

	if principal.ID != "123" {
		t.Error("ID mismatch")
	}
	if !principal.HasRole("editor") {
		t.Error("Should have editor role")
	}
	if !principal.HasAnyRole("admin", "reviewer") {
		t.Error("Should have at least one of these roles")
	}
	if !principal.HasAllRoles("user", "editor") {
		t.Error("Should have all these roles")
	}

	if principal.Attributes["department"] != "engineering" {
		t.Error("Attribute mismatch")
	}
}

func TestSecureToken_Uniqueness(t *testing.T) {
	tokens := make(map[string]bool)

	for i := 0; i < 1000; i++ {
		token, err := GenerateSecureToken(32)
		if err != nil {
			t.Fatalf("Failed to generate token: %v", err)
		}
		if tokens[token] {
			t.Fatalf("Duplicate token generated at iteration %d", i)
		}
		tokens[token] = true
	}
}

// Benchmark tests

func BenchmarkHasher_Hash(b *testing.B) {
	hasher := NewDefaultHasher()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		hasher.Hash("password123")
	}
}

func BenchmarkHasher_Verify(b *testing.B) {
	hasher := NewDefaultHasher()
	hash, _ := hasher.Hash("password123")
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		hasher.Verify("password123", hash)
	}
}

func BenchmarkTokenService_Generate(b *testing.B) {
	tokenService := NewInMemoryTokenService(time.Hour)
	ctx := context.Background()
	principal := &api.Principal{ID: "1", Username: "test"}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		tokenService.Generate(ctx, principal)
	}
}

func BenchmarkTokenService_Validate(b *testing.B) {
	tokenService := NewInMemoryTokenService(time.Hour)
	ctx := context.Background()
	principal := &api.Principal{ID: "1", Username: "test"}
	token, _ := tokenService.Generate(ctx, principal)
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		tokenService.Validate(ctx, token.Value)
	}
}

func BenchmarkRBACAuthorizer(b *testing.B) {
	authorizer := NewRBACAuthorizer()
	authorizer.AddRole(&api.Role{
		Name: "user",
		Permissions: []api.Permission{
			{Action: "read", Resource: "posts"},
		},
	})

	ctx := context.Background()
	principal := &api.Principal{ID: "1", Roles: []string{"user"}}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		authorizer.Authorize(ctx, principal, "read", "posts")
	}
}

func BenchmarkGenerateSecureToken(b *testing.B) {
	for i := 0; i < b.N; i++ {
		GenerateSecureToken(32)
	}
}
