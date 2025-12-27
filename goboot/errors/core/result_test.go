package core

import (
	"testing"
)

func TestOk(t *testing.T) {
	result := Ok(42)

	if !result.IsOk() {
		t.Error("Expected IsOk to be true")
	}
	if result.IsErr() {
		t.Error("Expected IsErr to be false")
	}
	if result.Unwrap() != 42 {
		t.Errorf("Expected 42, got %v", result.Unwrap())
	}
}

func TestErr(t *testing.T) {
	result := Err[int]("something went wrong")

	if result.IsOk() {
		t.Error("Expected IsOk to be false")
	}
	if !result.IsErr() {
		t.Error("Expected IsErr to be true")
	}
	if result.UnwrapErr() != "something went wrong" {
		t.Errorf("Unexpected error message: %s", result.UnwrapErr())
	}
}

func TestUnwrapOr(t *testing.T) {
	okResult := Ok(42)
	errResult := Err[int]("error")

	if okResult.UnwrapOr(0) != 42 {
		t.Error("UnwrapOr should return value for Ok")
	}
	if errResult.UnwrapOr(100) != 100 {
		t.Error("UnwrapOr should return default for Err")
	}
}

func TestUnwrapPanicsOnErr(t *testing.T) {
	defer func() {
		if r := recover(); r == nil {
			t.Error("Expected Unwrap to panic on Err")
		}
	}()

	result := Err[int]("error")
	result.Unwrap()
}

func TestUnwrapErrPanicsOnOk(t *testing.T) {
	defer func() {
		if r := recover(); r == nil {
			t.Error("Expected UnwrapErr to panic on Ok")
		}
	}()

	result := Ok(42)
	result.UnwrapErr()
}

func TestMap(t *testing.T) {
	result := Ok(10)
	mapped := result.Map(func(v int) int { return v * 2 })

	if mapped.Unwrap() != 20 {
		t.Errorf("Expected 20, got %v", mapped.Unwrap())
	}

	errResult := Err[int]("error")
	mappedErr := errResult.Map(func(v int) int { return v * 2 })

	if !mappedErr.IsErr() {
		t.Error("Map should preserve Err")
	}
}

func TestAndThen(t *testing.T) {
	result := Ok(10)
	chained := result.AndThen(func(v int) Result[int] {
		if v > 5 {
			return Ok(v * 2)
		}
		return Err[int]("value too small")
	})

	if chained.Unwrap() != 20 {
		t.Errorf("Expected 20, got %v", chained.Unwrap())
	}

	result2 := Ok(3)
	chained2 := result2.AndThen(func(v int) Result[int] {
		if v > 5 {
			return Ok(v * 2)
		}
		return Err[int]("value too small")
	})

	if !chained2.IsErr() {
		t.Error("AndThen should return Err when function returns Err")
	}
}

func TestChainErrors(t *testing.T) {
	err1 := &testError{"first"}
	err2 := &testError{"second"}
	err3 := &testError{"third"}

	chained := ChainErrors(err1, err2, err3)

	if chained.Error() != "first" {
		t.Errorf("Expected 'first', got '%s'", chained.Error())
	}
}

func TestWrapError(t *testing.T) {
	original := &testError{"original error"}
	wrapped := WrapError(original, "wrapped error")

	if wrapped.Error() != "wrapped error" {
		t.Errorf("Expected 'wrapped error', got '%s'", wrapped.Error())
	}

	// Check unwrap
	if unwrapper, ok := wrapped.(interface{ Unwrap() error }); ok {
		if unwrapper.Unwrap().Error() != "original error" {
			t.Error("Unwrap should return original error")
		}
	} else {
		t.Error("Wrapped error should implement Unwrap")
	}
}

func TestErrFrom(t *testing.T) {
	err := &testError{"test error"}
	result := ErrFrom[int](err)

	if !result.IsErr() {
		t.Error("ErrFrom should create Err result")
	}
	if result.UnwrapErr() != "test error" {
		t.Errorf("Unexpected error message: %s", result.UnwrapErr())
	}
}

func TestErrFromNil(t *testing.T) {
	result := ErrFrom[int](nil)

	if !result.IsErr() {
		t.Error("ErrFrom with nil should create Err result")
	}
	if result.UnwrapErr() != "unknown error" {
		t.Errorf("Expected 'unknown error', got '%s'", result.UnwrapErr())
	}
}

type testError struct {
	msg string
}

func (e *testError) Error() string {
	return e.msg
}
