package core

import (
	"testing"
)

func TestRequired(t *testing.T) {
	rule := Required()

	// Empty string should fail
	err := rule("")
	if err == nil {
		t.Error("Expected error for empty string")
	}
	if err.Code != "REQUIRED" {
		t.Errorf("Expected REQUIRED code, got %s", err.Code)
	}

	// Whitespace-only should fail
	err = rule("   ")
	if err == nil {
		t.Error("Expected error for whitespace-only string")
	}

	// Valid value should pass
	err = rule("hello")
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	// nil should fail
	err = rule(nil)
	if err == nil {
		t.Error("Expected error for nil")
	}
}

func TestMinLength(t *testing.T) {
	rule := MinLength(5)

	// Too short
	err := rule("abcd")
	if err == nil {
		t.Error("Expected error for short string")
	}
	if err.Code != "MIN_LENGTH" {
		t.Errorf("Expected MIN_LENGTH code, got %s", err.Code)
	}

	// Exact length
	err = rule("abcde")
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	// Longer
	err = rule("abcdef")
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
}

func TestMaxLength(t *testing.T) {
	rule := MaxLength(5)

	// Valid
	err := rule("abc")
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	// Exact length
	err = rule("abcde")
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	// Too long
	err = rule("abcdef")
	if err == nil {
		t.Error("Expected error for long string")
	}
	if err.Code != "MAX_LENGTH" {
		t.Errorf("Expected MAX_LENGTH code, got %s", err.Code)
	}
}

func TestMin(t *testing.T) {
	rule := Min(10)

	tests := []struct {
		value    any
		expected bool
	}{
		{5, false},
		{10, true},
		{15, true},
		{int32(5), false},
		{int64(15), true},
		{float32(5.0), false},
		{float64(15.0), true},
	}

	for _, tt := range tests {
		err := rule(tt.value)
		if (err == nil) != tt.expected {
			t.Errorf("Min(10) for %v: expected valid=%v, got err=%v", tt.value, tt.expected, err)
		}
	}
}

func TestMax(t *testing.T) {
	rule := Max(10)

	tests := []struct {
		value    any
		expected bool
	}{
		{5, true},
		{10, true},
		{15, false},
		{int32(15), false},
		{float64(5.0), true},
	}

	for _, tt := range tests {
		err := rule(tt.value)
		if (err == nil) != tt.expected {
			t.Errorf("Max(10) for %v: expected valid=%v, got err=%v", tt.value, tt.expected, err)
		}
	}
}

func TestPattern(t *testing.T) {
	rule := Pattern(`^[a-z]+$`)

	// Valid
	err := rule("abc")
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	// Invalid
	err = rule("ABC123")
	if err == nil {
		t.Error("Expected error for invalid pattern")
	}
	if err.Code != "PATTERN" {
		t.Errorf("Expected PATTERN code, got %s", err.Code)
	}
}

func TestEmail(t *testing.T) {
	rule := Email()

	validEmails := []string{
		"test@example.com",
		"user.name@domain.org",
		"user+tag@example.co.uk",
	}

	for _, email := range validEmails {
		err := rule(email)
		if err != nil {
			t.Errorf("Email %s should be valid: %v", email, err)
		}
	}

	invalidEmails := []string{
		"notanemail",
		"missing@tld",
		"@nodomain.com",
	}

	for _, email := range invalidEmails {
		err := rule(email)
		if err == nil {
			t.Errorf("Email %s should be invalid", email)
		}
	}

	// Empty should pass (use Required for mandatory)
	err := rule("")
	if err != nil {
		t.Error("Empty email should pass (not required)")
	}
}

func TestURL(t *testing.T) {
	rule := URL()

	validURLs := []string{
		"http://example.com",
		"https://example.com/path",
		"https://example.com/path?query=1",
	}

	for _, url := range validURLs {
		err := rule(url)
		if err != nil {
			t.Errorf("URL %s should be valid: %v", url, err)
		}
	}

	invalidURLs := []string{
		"notaurl",
		"ftp://example.com",
		"//missing-protocol.com",
	}

	for _, url := range invalidURLs {
		err := rule(url)
		if err == nil {
			t.Errorf("URL %s should be invalid", url)
		}
	}
}

func TestIn(t *testing.T) {
	rule := In("active", "pending", "inactive")

	// Valid values
	for _, v := range []string{"active", "pending", "inactive"} {
		err := rule(v)
		if err != nil {
			t.Errorf("Value %s should be valid: %v", v, err)
		}
	}

	// Invalid value
	err := rule("unknown")
	if err == nil {
		t.Error("Expected error for invalid value")
	}
	if err.Code != "IN" {
		t.Errorf("Expected IN code, got %s", err.Code)
	}
}

func TestObjectValidator(t *testing.T) {
	validator := NewObjectValidator()
	validator.Field("email").
		AddRule(Required()).
		AddRule(Email())
	validator.Field("age").
		AddRule(Min(18)).
		AddRule(Max(120))
	validator.Field("status").
		AddRule(In("active", "inactive"))

	t.Run("ValidData", func(t *testing.T) {
		data := map[string]any{
			"email":  "test@example.com",
			"age":    25,
			"status": "active",
		}

		result := validator.Validate(data)
		if !result.IsValid() {
			t.Errorf("Expected valid, got errors: %v", result.Errors)
		}
	})

	t.Run("InvalidData", func(t *testing.T) {
		data := map[string]any{
			"email":  "invalid",
			"age":    15,
			"status": "unknown",
		}

		result := validator.Validate(data)
		if result.IsValid() {
			t.Error("Expected invalid")
		}
		if len(result.Errors) != 3 {
			t.Errorf("Expected 3 errors, got %d", len(result.Errors))
		}
	})

	t.Run("MissingFields", func(t *testing.T) {
		data := map[string]any{}

		result := validator.Validate(data)
		if result.IsValid() {
			t.Error("Expected invalid for missing required fields")
		}
	})
}
