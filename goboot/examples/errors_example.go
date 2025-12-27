//go:build ignore

// Package main demonstrates the errors module usage.
package main

import (
	"fmt"

	"dev.engineeringlabs/goboot/errors"
	"dev.engineeringlabs/goboot/errors/core"
)

func main() {
	fmt.Println("=== Goboot Errors Module Example ===\n")

	// Example 1: Using the Result monad
	fmt.Println("1. Result Monad:")
	result := divide(10, 2)
	if result.IsOk() {
		fmt.Printf("   10 / 2 = %v\n", result.Unwrap())
	}

	result = divide(10, 0)
	if result.IsErr() {
		fmt.Printf("   10 / 0 = Error: %s\n", result.UnwrapErr())
	}

	// Example 2: Using UnwrapOr for defaults
	fmt.Println("\n2. UnwrapOr:")
	value := divide(10, 0).UnwrapOr(-1)
	fmt.Printf("   Value with default: %v\n", value)

	// Example 3: Chaining with AndThen
	fmt.Println("\n3. Chaining with AndThen:")
	chainResult := divide(100, 2).AndThen(func(v float64) core.Result[float64] {
		return divide(v, 5)
	})
	fmt.Printf("   100 / 2 / 5 = %v\n", chainResult.Unwrap())

	// Example 4: Using GobootError
	fmt.Println("\n4. GobootError:")
	err := errors.NewGobootError(
		"validation failed",
		errors.Validation,
		errors.WithDetail("field", "email"),
		errors.WithDetail("value", "invalid-email"),
	)
	fmt.Printf("   Error: %s\n", err)
	fmt.Printf("   Details: %v\n", err.ToMap())

	// Example 5: Error with cause
	fmt.Println("\n5. Error with cause:")
	originalErr := fmt.Errorf("connection refused")
	wrappedErr := errors.NewGobootError(
		"failed to connect to database",
		errors.ExternalService,
		errors.WithCause(originalErr),
	)
	fmt.Printf("   Error: %s\n", wrappedErr)
	fmt.Printf("   Cause: %v\n", wrappedErr.Cause)
}

func divide(a, b float64) core.Result[float64] {
	if b == 0 {
		return core.Err[float64]("division by zero")
	}
	return core.Ok(a / b)
}
