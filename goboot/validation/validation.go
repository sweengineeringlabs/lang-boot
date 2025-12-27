// Package validation provides input validation for the goboot framework.
//
// This module provides:
//   - API layer: ValidationError, ValidationResult, Validator interface
//   - Core layer: Common validation rules, ObjectValidator
//
// Example:
//
//	import "dev.engineeringlabs/goboot/validation"
//
//	validator := validation.NewObjectValidator()
//	validator.Field("email").
//	    AddRule(validation.Required()).
//	    AddRule(validation.Email())
//	validator.Field("age").
//	    AddRule(validation.Min(18))
//
//	result := validator.Validate(map[string]any{
//	    "email": "test@example.com",
//	    "age":   25,
//	})
//
//	if !result.IsValid() {
//	    fmt.Println(result.Errors)
//	}
package validation

import (
	"dev.engineeringlabs/goboot/validation/api"
	"dev.engineeringlabs/goboot/validation/core"
)

// Re-export API types
type (
	// ValidationError represents a single validation error.
	ValidationError = api.ValidationError
	// ValidationResult holds the result of a validation.
	ValidationResult = api.ValidationResult
	// Validator is the interface for validators.
	Validator = api.Validator
	// Rule is a validation rule function.
	Rule = api.Rule
	// FieldValidator validates a single field.
	FieldValidator = api.FieldValidator
)

// Re-export API functions
var (
	NewValidationResult = api.NewValidationResult
	NewFieldValidator   = api.NewFieldValidator
)

// Re-export Core types
type ObjectValidator = core.ObjectValidator

// Re-export Core functions
var (
	NewObjectValidator = core.NewObjectValidator
	Required           = core.Required
	MinLength          = core.MinLength
	MaxLength          = core.MaxLength
	Min                = core.Min
	Max                = core.Max
	Pattern            = core.Pattern
	Email              = core.Email
	URL                = core.URL
	In                 = core.In
)
