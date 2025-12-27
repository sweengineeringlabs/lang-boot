// Package core contains the implementation details for the validation module.
package core

import (
	"fmt"
	"regexp"
	"strings"

	"dev.engineeringlabs/goboot/validation/api"
)

// Common validation rules

// Required creates a rule that checks if a value is not empty.
func Required() api.Rule {
	return func(value any) *api.ValidationError {
		switch v := value.(type) {
		case string:
			if strings.TrimSpace(v) == "" {
				return &api.ValidationError{
					Message: "is required",
					Code:    "REQUIRED",
				}
			}
		case nil:
			return &api.ValidationError{
				Message: "is required",
				Code:    "REQUIRED",
			}
		}
		return nil
	}
}

// MinLength creates a rule that checks minimum string length.
func MinLength(min int) api.Rule {
	return func(value any) *api.ValidationError {
		if str, ok := value.(string); ok {
			if len(str) < min {
				return &api.ValidationError{
					Message: fmt.Sprintf("must be at least %d characters", min),
					Code:    "MIN_LENGTH",
				}
			}
		}
		return nil
	}
}

// MaxLength creates a rule that checks maximum string length.
func MaxLength(max int) api.Rule {
	return func(value any) *api.ValidationError {
		if str, ok := value.(string); ok {
			if len(str) > max {
				return &api.ValidationError{
					Message: fmt.Sprintf("must be at most %d characters", max),
					Code:    "MAX_LENGTH",
				}
			}
		}
		return nil
	}
}

// Min creates a rule that checks minimum numeric value.
func Min(min float64) api.Rule {
	return func(value any) *api.ValidationError {
		var v float64
		switch n := value.(type) {
		case int:
			v = float64(n)
		case int32:
			v = float64(n)
		case int64:
			v = float64(n)
		case float32:
			v = float64(n)
		case float64:
			v = n
		default:
			return nil
		}
		if v < min {
			return &api.ValidationError{
				Message: fmt.Sprintf("must be at least %v", min),
				Code:    "MIN",
			}
		}
		return nil
	}
}

// Max creates a rule that checks maximum numeric value.
func Max(max float64) api.Rule {
	return func(value any) *api.ValidationError {
		var v float64
		switch n := value.(type) {
		case int:
			v = float64(n)
		case int32:
			v = float64(n)
		case int64:
			v = float64(n)
		case float32:
			v = float64(n)
		case float64:
			v = n
		default:
			return nil
		}
		if v > max {
			return &api.ValidationError{
				Message: fmt.Sprintf("must be at most %v", max),
				Code:    "MAX",
			}
		}
		return nil
	}
}

// Pattern creates a rule that checks if a string matches a regex pattern.
func Pattern(pattern string) api.Rule {
	re := regexp.MustCompile(pattern)
	return func(value any) *api.ValidationError {
		if str, ok := value.(string); ok {
			if !re.MatchString(str) {
				return &api.ValidationError{
					Message: fmt.Sprintf("must match pattern %s", pattern),
					Code:    "PATTERN",
				}
			}
		}
		return nil
	}
}

// Email creates a rule that validates email format.
func Email() api.Rule {
	// Simple email regex
	emailPattern := `^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$`
	re := regexp.MustCompile(emailPattern)
	return func(value any) *api.ValidationError {
		if str, ok := value.(string); ok {
			if str != "" && !re.MatchString(str) {
				return &api.ValidationError{
					Message: "must be a valid email address",
					Code:    "EMAIL",
				}
			}
		}
		return nil
	}
}

// URL creates a rule that validates URL format.
func URL() api.Rule {
	urlPattern := `^https?://[^\s/$.?#].[^\s]*$`
	re := regexp.MustCompile(urlPattern)
	return func(value any) *api.ValidationError {
		if str, ok := value.(string); ok {
			if str != "" && !re.MatchString(str) {
				return &api.ValidationError{
					Message: "must be a valid URL",
					Code:    "URL",
				}
			}
		}
		return nil
	}
}

// In creates a rule that checks if a value is in a list of allowed values.
func In(allowed ...any) api.Rule {
	return func(value any) *api.ValidationError {
		for _, a := range allowed {
			if value == a {
				return nil
			}
		}
		return &api.ValidationError{
			Message: fmt.Sprintf("must be one of: %v", allowed),
			Code:    "IN",
		}
	}
}

// NotIn creates a rule that checks if a value is NOT in a list of disallowed values.
func NotIn(disallowed ...any) api.Rule {
	return func(value any) *api.ValidationError {
		for _, d := range disallowed {
			if value == d {
				return &api.ValidationError{
					Message: fmt.Sprintf("must not be one of: %v", disallowed),
					Code:    "NOT_IN",
				}
			}
		}
		return nil
	}
}

// Between creates a rule that checks if a numeric value is between min and max (inclusive).
func Between(min, max float64) api.Rule {
	return func(value any) *api.ValidationError {
		var v float64
		switch n := value.(type) {
		case int:
			v = float64(n)
		case int32:
			v = float64(n)
		case int64:
			v = float64(n)
		case float32:
			v = float64(n)
		case float64:
			v = n
		default:
			return nil
		}
		if v < min || v > max {
			return &api.ValidationError{
				Message: fmt.Sprintf("must be between %v and %v", min, max),
				Code:    "BETWEEN",
			}
		}
		return nil
	}
}

// Length creates a rule that checks if a string length is between min and max (inclusive).
func Length(min, max int) api.Rule {
	return func(value any) *api.ValidationError {
		if str, ok := value.(string); ok {
			length := len(str)
			if length < min || length > max {
				return &api.ValidationError{
					Message: fmt.Sprintf("must be between %d and %d characters", min, max),
					Code:    "LENGTH",
				}
			}
		}
		return nil
	}
}

// Alphanumeric creates a rule that checks if a string contains only letters and numbers.
func Alphanumeric() api.Rule {
	re := regexp.MustCompile(`^[a-zA-Z0-9]*$`)
	return func(value any) *api.ValidationError {
		if str, ok := value.(string); ok {
			if str != "" && !re.MatchString(str) {
				return &api.ValidationError{
					Message: "must contain only letters and numbers",
					Code:    "ALPHANUMERIC",
				}
			}
		}
		return nil
	}
}

// Numeric creates a rule that checks if a string contains only digits.
func Numeric() api.Rule {
	re := regexp.MustCompile(`^[0-9]*$`)
	return func(value any) *api.ValidationError {
		if str, ok := value.(string); ok {
			if str != "" && !re.MatchString(str) {
				return &api.ValidationError{
					Message: "must contain only digits",
					Code:    "NUMERIC",
				}
			}
		}
		return nil
	}
}

// IP creates a rule that validates IP address format.
func IP() api.Rule {
	// Matches IPv4 and IPv6
	ipv4 := `^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$`
	ipv6 := `^([0-9a-fA-F]{0,4}:){2,7}[0-9a-fA-F]{0,4}$`
	reIPv4 := regexp.MustCompile(ipv4)
	reIPv6 := regexp.MustCompile(ipv6)
	return func(value any) *api.ValidationError {
		if str, ok := value.(string); ok {
			if str != "" && !reIPv4.MatchString(str) && !reIPv6.MatchString(str) {
				return &api.ValidationError{
					Message: "must be a valid IP address",
					Code:    "IP",
				}
			}
		}
		return nil
	}
}

// UUID creates a rule that validates UUID format.
func UUID() api.Rule {
	uuidPattern := `^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$`
	re := regexp.MustCompile(uuidPattern)
	return func(value any) *api.ValidationError {
		if str, ok := value.(string); ok {
			if str != "" && !re.MatchString(str) {
				return &api.ValidationError{
					Message: "must be a valid UUID",
					Code:    "UUID",
				}
			}
		}
		return nil
	}
}

// Custom creates a custom validation rule.
func Custom(code, message string, fn func(value any) bool) api.Rule {
	return func(value any) *api.ValidationError {
		if !fn(value) {
			return &api.ValidationError{
				Message: message,
				Code:    code,
			}
		}
		return nil
	}
}

// ObjectValidator validates multiple fields of an object.
type ObjectValidator struct {
	validators []*api.FieldValidator
}

// NewObjectValidator creates a new ObjectValidator.
func NewObjectValidator() *ObjectValidator {
	return &ObjectValidator{
		validators: make([]*api.FieldValidator, 0),
	}
}

// Field adds a field validator.
func (v *ObjectValidator) Field(field string) *api.FieldValidator {
	fv := api.NewFieldValidator(field)
	v.validators = append(v.validators, fv)
	return fv
}

// Validate validates a map of field values.
func (v *ObjectValidator) Validate(values map[string]any) *api.ValidationResult {
	result := api.NewValidationResult()
	for _, fv := range v.validators {
		value := values[fv.Field]
		fieldResult := fv.Validate(value)
		result.Merge(fieldResult)
	}
	return result
}
