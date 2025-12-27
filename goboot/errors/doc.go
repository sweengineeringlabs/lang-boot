// Package errors provides error types and utilities for the goboot framework.
//
// This module provides:
//   - API layer: ErrorCode, GobootError
//   - Core layer: Result monad, error chaining utilities
//
// Example:
//
//	import "dev.engineeringlabs/goboot/errors"
//
//	func divide(a, b float64) errors.Result[float64] {
//	    if b == 0 {
//	        return errors.Err[float64]("Division by zero")
//	    }
//	    return errors.Ok(a / b)
//	}
package errors

// Re-export API types
// ErrorCode represents standard error codes for programmatic handling.
// GobootError is the base error type for framework errors.
//
// Re-export Core types
// Result is a monad for functional error handling.
// Ok creates a successful result.
// Err creates a failed result.
