// Package errors provides error types and utilities for the goboot framework.
package errors

import (
	"dev.engineeringlabs/goboot/errors/api"
	"dev.engineeringlabs/goboot/errors/core"
)

// Re-export API types
type (
	// ErrorCode represents standard error codes for programmatic handling.
	ErrorCode = api.ErrorCode
	// GobootError is the base error type for framework errors.
	GobootError = api.GobootError
	// ErrorOption is a functional option for configuring GobootError.
	ErrorOption = api.ErrorOption
)

// Re-export API constants
const (
	Unknown          = api.Unknown
	Internal         = api.Internal
	Validation       = api.Validation
	InvalidInput     = api.InvalidInput
	MissingRequired  = api.MissingRequired
	NotFound         = api.NotFound
	AlreadyExists    = api.AlreadyExists
	Conflict         = api.Conflict
	Unauthorized     = api.Unauthorized
	Forbidden        = api.Forbidden
	PermissionDenied = api.PermissionDenied
	Timeout          = api.Timeout
	Cancelled        = api.Cancelled
	Unavailable      = api.Unavailable
	Configuration    = api.Configuration
	ExternalService  = api.ExternalService
	Network          = api.Network
)

// Re-export API functions
var (
	NewGobootError = api.NewGobootError
	WithCause      = api.WithCause
	WithDetails    = api.WithDetails
	WithDetail     = api.WithDetail
)

// Note: For Result[T] type, import directly from core package:
//   import "dev.engineeringlabs/goboot/errors/core"
//   var r core.Result[string] = core.Ok("value")

// Re-export Core functions for non-generic error handling
var (
	ChainErrors = core.ChainErrors
	WrapError   = core.WrapError
)

// OkTyped creates a typed Ok result.
// Use this when type inference is needed.
func OkTyped[T any](value T) core.Result[T] {
	return core.Ok(value)
}

// ErrTyped creates a typed Err result.
// Use this when type inference is needed.
func ErrTyped[T any](err string) core.Result[T] {
	return core.Err[T](err)
}

// ErrFromTyped creates a typed Err result from an error.
func ErrFromTyped[T any](err error) core.Result[T] {
	return core.ErrFrom[T](err)
}
