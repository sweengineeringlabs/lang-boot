// Package api contains the public interfaces and types for the validation module.
package api

import (
	"fmt"
)

// ValidationError represents a single validation error.
type ValidationError struct {
	Field   string
	Message string
	Code    string
}

// Error implements the error interface.
func (e *ValidationError) Error() string {
	return fmt.Sprintf("%s: %s", e.Field, e.Message)
}

// ValidationResult holds the result of a validation.
type ValidationResult struct {
	Errors []ValidationError
}

// IsValid returns true if there are no validation errors.
func (r *ValidationResult) IsValid() bool {
	return len(r.Errors) == 0
}

// AddError adds a validation error.
func (r *ValidationResult) AddError(field, message, code string) {
	r.Errors = append(r.Errors, ValidationError{
		Field:   field,
		Message: message,
		Code:    code,
	})
}

// Merge merges another ValidationResult into this one.
func (r *ValidationResult) Merge(other *ValidationResult) {
	r.Errors = append(r.Errors, other.Errors...)
}

// Error implements the error interface.
func (r *ValidationResult) Error() string {
	if r.IsValid() {
		return ""
	}
	return fmt.Sprintf("validation failed with %d error(s)", len(r.Errors))
}

// NewValidationResult creates a new ValidationResult.
func NewValidationResult() *ValidationResult {
	return &ValidationResult{
		Errors: make([]ValidationError, 0),
	}
}

// Validator is the interface for validators.
type Validator interface {
	Validate(value any) *ValidationResult
}

// Rule is a validation rule function.
type Rule func(value any) *ValidationError

// FieldValidator validates a single field.
type FieldValidator struct {
	Field string
	Rules []Rule
}

// NewFieldValidator creates a new FieldValidator.
func NewFieldValidator(field string) *FieldValidator {
	return &FieldValidator{
		Field: field,
		Rules: make([]Rule, 0),
	}
}

// AddRule adds a validation rule.
func (v *FieldValidator) AddRule(rule Rule) *FieldValidator {
	v.Rules = append(v.Rules, rule)
	return v
}

// Validate validates a value against all rules.
func (v *FieldValidator) Validate(value any) *ValidationResult {
	result := NewValidationResult()
	for _, rule := range v.Rules {
		if err := rule(value); err != nil {
			err.Field = v.Field
			result.Errors = append(result.Errors, *err)
		}
	}
	return result
}
