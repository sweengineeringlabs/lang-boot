package core

import (
	"testing"
)

// Additional edge case tests for validation rules

func TestBetween(t *testing.T) {
	rule := Between(10, 20)

	tests := []struct {
		value    any
		expected bool
	}{
		{5, false},
		{10, true},
		{15, true},
		{20, true},
		{25, false},
		{int64(15), true},
		{float64(15.5), true},
	}

	for _, tt := range tests {
		err := rule(tt.value)
		if (err == nil) != tt.expected {
			t.Errorf("Between(10,20) for %v: expected valid=%v, got err=%v", tt.value, tt.expected, err)
		}
	}
}

func TestLength(t *testing.T) {
	rule := Length(5, 10)

	tests := []struct {
		value    string
		expected bool
	}{
		{"abcd", false},      // too short
		{"abcde", true},      // min
		{"abcdefgh", true},   // middle
		{"abcdefghij", true}, // max
		{"abcdefghijk", false}, // too long
	}

	for _, tt := range tests {
		err := rule(tt.value)
		if (err == nil) != tt.expected {
			t.Errorf("Length(5,10) for '%s': expected valid=%v, got err=%v", tt.value, tt.expected, err)
		}
	}
}

func TestNotIn(t *testing.T) {
	rule := NotIn("admin", "root", "superuser")

	tests := []struct {
		value    string
		expected bool
	}{
		{"admin", false},
		{"root", false},
		{"superuser", false},
		{"user", true},
		{"guest", true},
	}

	for _, tt := range tests {
		err := rule(tt.value)
		if (err == nil) != tt.expected {
			t.Errorf("NotIn() for '%s': expected valid=%v, got err=%v", tt.value, tt.expected, err)
		}
	}
}

func TestAlphanumeric(t *testing.T) {
	rule := Alphanumeric()

	tests := []struct {
		value    string
		expected bool
	}{
		{"abc123", true},
		{"ABC123", true},
		{"hello", true},
		{"123", true},
		{"hello-world", false},
		{"hello_world", false},
		{"hello world", false},
		{"", true}, // empty is valid (use Required for mandatory)
	}

	for _, tt := range tests {
		err := rule(tt.value)
		if (err == nil) != tt.expected {
			t.Errorf("Alphanumeric() for '%s': expected valid=%v, got err=%v", tt.value, tt.expected, err)
		}
	}
}

func TestNumeric(t *testing.T) {
	rule := Numeric()

	tests := []struct {
		value    string
		expected bool
	}{
		{"12345", true},
		{"0", true},
		{"123.45", false},
		{"12a34", false},
		{"", true}, // empty is valid
	}

	for _, tt := range tests {
		err := rule(tt.value)
		if (err == nil) != tt.expected {
			t.Errorf("Numeric() for '%s': expected valid=%v, got err=%v", tt.value, tt.expected, err)
		}
	}
}

func TestIP(t *testing.T) {
	rule := IP()

	validIPs := []string{
		"192.168.1.1",
		"10.0.0.0",
		"255.255.255.255",
		"0.0.0.0",
		"::1",
		"2001:db8::1",
	}

	for _, ip := range validIPs {
		err := rule(ip)
		if err != nil {
			t.Errorf("IP '%s' should be valid: %v", ip, err)
		}
	}

	invalidIPs := []string{
		"256.1.1.1",
		"192.168.1",
		"not an ip",
		"192.168.1.1.1",
	}

	for _, ip := range invalidIPs {
		err := rule(ip)
		if err == nil {
			t.Errorf("IP '%s' should be invalid", ip)
		}
	}
}

func TestUUID(t *testing.T) {
	rule := UUID()

	validUUIDs := []string{
		"123e4567-e89b-12d3-a456-426614174000",
		"550e8400-e29b-41d4-a716-446655440000",
		"6ba7b810-9dad-11d1-80b4-00c04fd430c8",
	}

	for _, uuid := range validUUIDs {
		err := rule(uuid)
		if err != nil {
			t.Errorf("UUID '%s' should be valid: %v", uuid, err)
		}
	}

	invalidUUIDs := []string{
		"not-a-uuid",
		"123e4567-e89b-12d3-a456",
		"123e4567-e89b-12d3-a456-42661417400g",
	}

	for _, uuid := range invalidUUIDs {
		err := rule(uuid)
		if err == nil {
			t.Errorf("UUID '%s' should be invalid", uuid)
		}
	}
}

func TestCustomRule(t *testing.T) {
	isPrime := Custom("PRIME", "must be a prime number", func(v any) bool {
		n, ok := v.(int)
		if !ok {
			return false
		}
		if n < 2 {
			return false
		}
		for i := 2; i*i <= n; i++ {
			if n%i == 0 {
				return false
			}
		}
		return true
	})

	primes := []int{2, 3, 5, 7, 11, 13, 17, 19}
	nonPrimes := []int{0, 1, 4, 6, 8, 9, 10, 12}

	for _, p := range primes {
		err := isPrime(p)
		if err != nil {
			t.Errorf("%d should be prime", p)
		}
	}

	for _, np := range nonPrimes {
		err := isPrime(np)
		if err == nil {
			t.Errorf("%d should not be prime", np)
		}
	}
}

func TestValidationResult(t *testing.T) {
	validator := NewObjectValidator()
	validator.Field("name").AddRule(Required()).AddRule(MinLength(2))
	validator.Field("email").AddRule(Required()).AddRule(Email())
	validator.Field("age").AddRule(Min(0)).AddRule(Max(150))

	t.Run("AllValid", func(t *testing.T) {
		data := map[string]any{
			"name":  "John",
			"email": "john@example.com",
			"age":   25,
		}

		result := validator.Validate(data)
		if !result.IsValid() {
			t.Errorf("Should be valid: %v", result.Errors)
		}
	})

	t.Run("SingleError", func(t *testing.T) {
		data := map[string]any{
			"name":  "J", // too short
			"email": "john@example.com",
			"age":   25,
		}

		result := validator.Validate(data)
		if result.IsValid() {
			t.Error("Should be invalid")
		}
		if len(result.Errors) != 1 {
			t.Errorf("Expected 1 error, got %d", len(result.Errors))
		}
	})

	t.Run("MultipleErrors", func(t *testing.T) {
		data := map[string]any{
			"name":  "",        // required
			"email": "invalid", // invalid email
			"age":   200,       // too high
		}

		result := validator.Validate(data)
		if result.IsValid() {
			t.Error("Should be invalid")
		}
		if len(result.Errors) < 2 {
			t.Errorf("Expected multiple errors, got %d", len(result.Errors))
		}
	})

	t.Run("CheckSpecificField", func(t *testing.T) {
		data := map[string]any{
			"name":  "",
			"email": "john@example.com",
			"age":   25,
		}

		result := validator.Validate(data)

		// Count errors for name field (empty triggers both required AND minLength)
		nameErrors := 0
		for _, err := range result.Errors {
			if err.Field == "name" {
				nameErrors++
			}
		}

		if nameErrors < 1 {
			t.Errorf("Expected at least 1 error for 'name', got %d", nameErrors)
		}
	})
}

func TestFieldValidator_Multiple(t *testing.T) {
	validator := NewObjectValidator()
	validator.Field("password").
		AddRule(Required()).
		AddRule(MinLength(8)).
		AddRule(MaxLength(100)).
		AddRule(Pattern(`[A-Z]`)).  // must contain uppercase
		AddRule(Pattern(`[a-z]`)).  // must contain lowercase
		AddRule(Pattern(`[0-9]`))   // must contain number

	t.Run("StrongPassword", func(t *testing.T) {
		data := map[string]any{"password": "SecureP4ssword"}
		result := validator.Validate(data)
		if !result.IsValid() {
			t.Errorf("Should be valid: %v", result.Errors)
		}
	})

	t.Run("WeakPassword_NoNumber", func(t *testing.T) {
		data := map[string]any{"password": "SecurePassword"}
		result := validator.Validate(data)
		if result.IsValid() {
			t.Error("Should be invalid - no number")
		}
	})

	t.Run("WeakPassword_TooShort", func(t *testing.T) {
		data := map[string]any{"password": "Pa1"}
		result := validator.Validate(data)
		if result.IsValid() {
			t.Error("Should be invalid - too short")
		}
	})
}

// Benchmark tests

func BenchmarkValidator_Simple(b *testing.B) {
	validator := NewObjectValidator()
	validator.Field("email").AddRule(Required()).AddRule(Email())

	data := map[string]any{"email": "test@example.com"}
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		validator.Validate(data)
	}
}

func BenchmarkValidator_Complex(b *testing.B) {
	validator := NewObjectValidator()
	validator.Field("name").AddRule(Required()).AddRule(MinLength(2)).AddRule(MaxLength(50))
	validator.Field("email").AddRule(Required()).AddRule(Email())
	validator.Field("age").AddRule(Min(0)).AddRule(Max(150))
	validator.Field("status").AddRule(In("active", "pending", "inactive"))

	data := map[string]any{
		"name":   "John Doe",
		"email":  "john@example.com",
		"age":    30,
		"status": "active",
	}
	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		validator.Validate(data)
	}
}

func BenchmarkRequired(b *testing.B) {
	rule := Required()
	for i := 0; i < b.N; i++ {
		rule("test")
	}
}

func BenchmarkEmail(b *testing.B) {
	rule := Email()
	for i := 0; i < b.N; i++ {
		rule("test@example.com")
	}
}

func BenchmarkPattern(b *testing.B) {
	rule := Pattern(`^[a-zA-Z0-9]+$`)
	for i := 0; i < b.N; i++ {
		rule("test123")
	}
}
