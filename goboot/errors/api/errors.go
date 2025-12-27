// Package api contains the public interfaces and types for the errors module.
package api

import (
	"fmt"
)

// ErrorCode represents standard error codes for programmatic handling.
type ErrorCode int

const (
	// Generic
	Unknown ErrorCode = iota
	Internal

	// Validation
	Validation
	InvalidInput
	MissingRequired

	// Resources
	NotFound
	AlreadyExists
	Conflict

	// Access
	Unauthorized
	Forbidden
	PermissionDenied

	// Operations
	Timeout
	Cancelled
	Unavailable

	// Configuration
	Configuration

	// External
	ExternalService
	Network
)

// String returns the string representation of an ErrorCode.
func (c ErrorCode) String() string {
	names := map[ErrorCode]string{
		Unknown:          "UNKNOWN",
		Internal:         "INTERNAL",
		Validation:       "VALIDATION",
		InvalidInput:     "INVALID_INPUT",
		MissingRequired:  "MISSING_REQUIRED",
		NotFound:         "NOT_FOUND",
		AlreadyExists:    "ALREADY_EXISTS",
		Conflict:         "CONFLICT",
		Unauthorized:     "UNAUTHORIZED",
		Forbidden:        "FORBIDDEN",
		PermissionDenied: "PERMISSION_DENIED",
		Timeout:          "TIMEOUT",
		Cancelled:        "CANCELLED",
		Unavailable:      "UNAVAILABLE",
		Configuration:    "CONFIGURATION",
		ExternalService:  "EXTERNAL_SERVICE",
		Network:          "NETWORK",
	}
	if name, ok := names[c]; ok {
		return name
	}
	return "UNKNOWN"
}

// GobootError is the base error type for framework errors.
//
// All framework errors can embed or extend this type for consistency.
type GobootError struct {
	Message string            // Human-readable error description
	Code    ErrorCode         // Error code for programmatic handling
	Cause   error             // Original error that caused this error
	Details map[string]any    // Additional context as key-value pairs
}

// Error implements the error interface.
func (e *GobootError) Error() string {
	return fmt.Sprintf("[%s] %s", e.Code.String(), e.Message)
}

// Unwrap returns the underlying cause for error wrapping.
func (e *GobootError) Unwrap() error {
	return e.Cause
}

// ToMap converts the error to a map for serialization.
func (e *GobootError) ToMap() map[string]any {
	result := map[string]any{
		"code":    e.Code.String(),
		"message": e.Message,
	}
	if len(e.Details) > 0 {
		result["details"] = e.Details
	}
	if e.Cause != nil {
		result["cause"] = e.Cause.Error()
	}
	return result
}

// NewGobootError creates a new GobootError.
func NewGobootError(message string, code ErrorCode, options ...ErrorOption) *GobootError {
	e := &GobootError{
		Message: message,
		Code:    code,
		Details: make(map[string]any),
	}
	for _, opt := range options {
		opt(e)
	}
	return e
}

// ErrorOption is a functional option for configuring GobootError.
type ErrorOption func(*GobootError)

// WithCause sets the cause of the error.
func WithCause(cause error) ErrorOption {
	return func(e *GobootError) {
		e.Cause = cause
	}
}

// WithDetails sets the details of the error.
func WithDetails(details map[string]any) ErrorOption {
	return func(e *GobootError) {
		e.Details = details
	}
}

// WithDetail adds a single detail to the error.
func WithDetail(key string, value any) ErrorOption {
	return func(e *GobootError) {
		if e.Details == nil {
			e.Details = make(map[string]any)
		}
		e.Details[key] = value
	}
}
