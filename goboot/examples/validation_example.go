//go:build ignore

// Package main demonstrates the validation module usage.
package main

import (
	"fmt"

	"dev.engineeringlabs/goboot/validation"
)

func main() {
	fmt.Println("=== Goboot Validation Module Example ===\n")

	// Example 1: Simple field validation
	fmt.Println("1. Single Field Validation:")
	emailValidator := validation.NewFieldValidator("email").
		AddRule(validation.Required()).
		AddRule(validation.Email())

	result := emailValidator.Validate("invalid-email")
	if !result.IsValid() {
		for _, err := range result.Errors {
			fmt.Printf("   %s: %s (%s)\n", err.Field, err.Message, err.Code)
		}
	}

	result = emailValidator.Validate("test@example.com")
	fmt.Printf("   Valid email: %v\n", result.IsValid())

	// Example 2: Object validation
	fmt.Println("\n2. Object Validation:")
	userValidator := validation.NewObjectValidator()
	userValidator.Field("username").
		AddRule(validation.Required()).
		AddRule(validation.MinLength(3)).
		AddRule(validation.MaxLength(20))
	userValidator.Field("email").
		AddRule(validation.Required()).
		AddRule(validation.Email())
	userValidator.Field("age").
		AddRule(validation.Min(18)).
		AddRule(validation.Max(120))
	userValidator.Field("website").
		AddRule(validation.URL())

	// Invalid data
	invalidData := map[string]any{
		"username": "ab",
		"email":    "not-an-email",
		"age":      15,
		"website":  "not-a-url",
	}

	result = userValidator.Validate(invalidData)
	fmt.Println("   Invalid data errors:")
	for _, err := range result.Errors {
		fmt.Printf("   - %s: %s\n", err.Field, err.Message)
	}

	// Valid data
	validData := map[string]any{
		"username": "johndoe",
		"email":    "john@example.com",
		"age":      25,
		"website":  "https://example.com",
	}

	result = userValidator.Validate(validData)
	fmt.Printf("\n   Valid data: %v (errors: %d)\n", result.IsValid(), len(result.Errors))

	// Example 3: Custom patterns
	fmt.Println("\n3. Pattern Validation:")
	phoneValidator := validation.NewFieldValidator("phone").
		AddRule(validation.Required()).
		AddRule(validation.Pattern(`^\+?[0-9]{10,14}$`))

	result = phoneValidator.Validate("+1234567890")
	fmt.Printf("   Valid phone: %v\n", result.IsValid())

	result = phoneValidator.Validate("invalid")
	fmt.Printf("   Invalid phone: %v (errors: %d)\n", result.IsValid(), len(result.Errors))

	// Example 4: In validation
	fmt.Println("\n4. In Validation:")
	statusValidator := validation.NewFieldValidator("status").
		AddRule(validation.In("active", "pending", "inactive"))

	result = statusValidator.Validate("active")
	fmt.Printf("   Status 'active': %v\n", result.IsValid())

	result = statusValidator.Validate("unknown")
	if !result.IsValid() {
		fmt.Printf("   Status 'unknown': %s\n", result.Errors[0].Message)
	}
}
