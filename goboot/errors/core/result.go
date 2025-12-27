// Package core contains the implementation details for the errors module.
package core

// Result is a monad for functional error handling.
//
// A Result is either Ok(value) or Err(error).
//
// Example:
//
//	func divide(a, b int) Result[float64] {
//	    if b == 0 {
//	        return Err[float64]("Division by zero")
//	    }
//	    return Ok(float64(a) / float64(b))
//	}
//
//	result := divide(10, 2)
//	if result.IsOk() {
//	    fmt.Printf("Result: %v\n", result.Unwrap())
//	} else {
//	    fmt.Printf("Error: %v\n", result.UnwrapErr())
//	}
type Result[T any] struct {
	value T
	err   string
	isOk  bool
}

// IsOk returns true if the result is Ok.
func (r Result[T]) IsOk() bool {
	return r.isOk
}

// IsErr returns true if the result is Err.
func (r Result[T]) IsErr() bool {
	return !r.isOk
}

// Unwrap returns the value, panics if Err.
func (r Result[T]) Unwrap() T {
	if !r.isOk {
		panic("called Unwrap on Err: " + r.err)
	}
	return r.value
}

// UnwrapOr returns the value or the provided default.
func (r Result[T]) UnwrapOr(defaultVal T) T {
	if r.isOk {
		return r.value
	}
	return defaultVal
}

// UnwrapErr returns the error, panics if Ok.
func (r Result[T]) UnwrapErr() string {
	if r.isOk {
		panic("called UnwrapErr on Ok")
	}
	return r.err
}

// Map transforms the Ok value using the provided function.
func (r Result[T]) Map(fn func(T) T) Result[T] {
	if r.isOk {
		return Ok(fn(r.value))
	}
	return r
}

// AndThen chains Result-returning functions.
func (r Result[T]) AndThen(fn func(T) Result[T]) Result[T] {
	if r.isOk {
		return fn(r.value)
	}
	return r
}

// Ok creates a successful Result.
func Ok[T any](value T) Result[T] {
	return Result[T]{
		value: value,
		isOk:  true,
	}
}

// Err creates a failed Result.
func Err[T any](err string) Result[T] {
	var zero T
	return Result[T]{
		value: zero,
		err:   err,
		isOk:  false,
	}
}

// ErrFrom creates a failed Result from an error.
func ErrFrom[T any](err error) Result[T] {
	var zero T
	if err == nil {
		return Result[T]{
			value: zero,
			err:   "unknown error",
			isOk:  false,
		}
	}
	return Result[T]{
		value: zero,
		err:   err.Error(),
		isOk:  false,
	}
}

// ChainErrors chains multiple errors together.
// Returns the last error with the others wrapped as causes.
func ChainErrors(errors ...error) error {
	if len(errors) == 0 {
		return nil
	}
	return &chainedError{errors: errors}
}

type chainedError struct {
	errors []error
}

func (c *chainedError) Error() string {
	if len(c.errors) == 0 {
		return ""
	}
	return c.errors[0].Error()
}

func (c *chainedError) Unwrap() error {
	if len(c.errors) <= 1 {
		return nil
	}
	return &chainedError{errors: c.errors[1:]}
}

// WrapError wraps an error with a new message.
type wrappedError struct {
	message string
	cause   error
}

func (w *wrappedError) Error() string {
	return w.message
}

func (w *wrappedError) Unwrap() error {
	return w.cause
}

// WrapError wraps an error with a new error type and message.
func WrapError(original error, message string) error {
	return &wrappedError{
		message: message,
		cause:   original,
	}
}
