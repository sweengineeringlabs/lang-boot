package api

import (
	"testing"
)

func TestErrorCodeString(t *testing.T) {
	tests := []struct {
		code     ErrorCode
		expected string
	}{
		{Unknown, "UNKNOWN"},
		{Internal, "INTERNAL"},
		{Validation, "VALIDATION"},
		{InvalidInput, "INVALID_INPUT"},
		{NotFound, "NOT_FOUND"},
		{Unauthorized, "UNAUTHORIZED"},
		{Timeout, "TIMEOUT"},
	}

	for _, tt := range tests {
		if tt.code.String() != tt.expected {
			t.Errorf("Expected %s, got %s", tt.expected, tt.code.String())
		}
	}
}

func TestGobootError(t *testing.T) {
	err := NewGobootError("test error", Validation)

	if err.Message != "test error" {
		t.Errorf("Unexpected message: %s", err.Message)
	}
	if err.Code != Validation {
		t.Errorf("Unexpected code: %v", err.Code)
	}
	if err.Error() != "[VALIDATION] test error" {
		t.Errorf("Unexpected Error(): %s", err.Error())
	}
}

func TestGobootErrorWithCause(t *testing.T) {
	cause := &simpleError{"original"}
	err := NewGobootError("wrapped", Internal, WithCause(cause))

	if err.Cause != cause {
		t.Error("Cause not set correctly")
	}
	if err.Unwrap() != cause {
		t.Error("Unwrap should return cause")
	}
}

func TestGobootErrorWithDetails(t *testing.T) {
	err := NewGobootError("error", Validation,
		WithDetails(map[string]any{"field": "email"}),
	)

	if err.Details["field"] != "email" {
		t.Error("Details not set correctly")
	}
}

func TestGobootErrorWithDetail(t *testing.T) {
	err := NewGobootError("error", Validation,
		WithDetail("field", "email"),
		WithDetail("value", "invalid"),
	)

	if err.Details["field"] != "email" {
		t.Error("Field detail not set")
	}
	if err.Details["value"] != "invalid" {
		t.Error("Value detail not set")
	}
}

func TestGobootErrorToMap(t *testing.T) {
	cause := &simpleError{"cause"}
	err := NewGobootError("error", NotFound,
		WithCause(cause),
		WithDetail("id", 123),
	)

	m := err.ToMap()

	if m["code"] != "NOT_FOUND" {
		t.Errorf("Unexpected code in map: %v", m["code"])
	}
	if m["message"] != "error" {
		t.Errorf("Unexpected message in map: %v", m["message"])
	}
	if m["cause"] != "cause" {
		t.Errorf("Unexpected cause in map: %v", m["cause"])
	}
	if details, ok := m["details"].(map[string]any); ok {
		if details["id"] != 123 {
			t.Error("Details not in map correctly")
		}
	} else {
		t.Error("Details should be in map")
	}
}

type simpleError struct {
	msg string
}

func (e *simpleError) Error() string {
	return e.msg
}
