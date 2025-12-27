package core

import (
	"fmt"
	"testing"
)

// Integration tests demonstrating Result usage patterns

func TestResult_ChainedOperations(t *testing.T) {
	// Test chaining multiple operations
	divide := func(a, b int) Result[float64] {
		if b == 0 {
			return Err[float64]("division by zero")
		}
		return Ok(float64(a) / float64(b))
	}

	// Success chain
	result := divide(10, 2).
		Map(func(v float64) float64 { return v * 2 }).
		AndThen(func(v float64) Result[float64] {
			if v > 100 {
				return Err[float64]("value too large")
			}
			return Ok(v)
		})

	if !result.IsOk() {
		t.Error("Chain should succeed")
	}
	if result.Unwrap() != 10.0 {
		t.Errorf("Expected 10.0, got %v", result.Unwrap())
	}

	// Failure in first step
	result = divide(10, 0).
		Map(func(v float64) float64 { return v * 2 })

	if !result.IsErr() {
		t.Error("Chain should fail on division by zero")
	}

	// Failure in AndThen
	result = divide(1000, 1).
		AndThen(func(v float64) Result[float64] {
			if v > 100 {
				return Err[float64]("value too large")
			}
			return Ok(v)
		})

	if !result.IsErr() {
		t.Error("Chain should fail on value too large")
	}
	if result.UnwrapErr() != "value too large" {
		t.Errorf("Wrong error: %s", result.UnwrapErr())
	}
}

func TestResult_MultipleAndThen(t *testing.T) {
	parseInt := func(s string) Result[int] {
		switch s {
		case "1":
			return Ok(1)
		case "2":
			return Ok(2)
		case "3":
			return Ok(3)
		default:
			return Err[int]("invalid number: " + s)
		}
	}

	multiplyByTwo := func(n int) Result[int] {
		return Ok(n * 2)
	}

	addTen := func(n int) Result[int] {
		return Ok(n + 10)
	}

	result := parseInt("2").
		AndThen(multiplyByTwo).
		AndThen(addTen)

	if result.Unwrap() != 14 {
		t.Errorf("Expected 14, got %d", result.Unwrap())
	}

	// Test failure propagation
	result = parseInt("invalid").
		AndThen(multiplyByTwo).
		AndThen(addTen)

	if !result.IsErr() {
		t.Error("Should propagate error")
	}
}

func TestResult_RecoveryPattern(t *testing.T) {
	riskyOperation := func(fail bool) Result[string] {
		if fail {
			return Err[string]("operation failed")
		}
		return Ok("success")
	}

	// Test recovery pattern using UnwrapOr
	result := riskyOperation(true).UnwrapOr("recovered")
	if result != "recovered" {
		t.Errorf("Expected 'recovered', got '%s'", result)
	}

	result = riskyOperation(false).UnwrapOr("recovered")
	if result != "success" {
		t.Errorf("Expected 'success', got '%s'", result)
	}
}

func TestChainErrors_MultipleErrors(t *testing.T) {
	err1 := &testError{"database error"}
	err2 := &testError{"network error"}
	err3 := &testError{"validation error"}

	chained := ChainErrors(err1, err2, err3)

	// First error should be returned
	if chained.Error() != "database error" {
		t.Errorf("Expected first error, got: %s", chained.Error())
	}

	// Should be able to unwrap
	if unwrapper, ok := chained.(interface{ Unwrap() error }); ok {
		next := unwrapper.Unwrap()
		if next.Error() != "network error" {
			t.Errorf("Unwrap should return second error, got: %s", next.Error())
		}
	}
}

func TestChainErrors_Empty(t *testing.T) {
	chained := ChainErrors()
	if chained != nil {
		t.Error("Empty chain should return nil")
	}
}

func TestChainErrors_Single(t *testing.T) {
	err := &testError{"only error"}
	chained := ChainErrors(err)

	if chained.Error() != "only error" {
		t.Errorf("Expected 'only error', got '%s'", chained.Error())
	}
}

func TestWrapError_NestedWrapping(t *testing.T) {
	original := &testError{"original"}
	wrapped1 := WrapError(original, "first wrap")
	wrapped2 := WrapError(wrapped1, "second wrap")

	if wrapped2.Error() != "second wrap" {
		t.Errorf("Top error should be 'second wrap', got '%s'", wrapped2.Error())
	}

	// Unwrap chain
	if unwrapper, ok := wrapped2.(interface{ Unwrap() error }); ok {
		level1 := unwrapper.Unwrap()
		if level1.Error() != "first wrap" {
			t.Error("First unwrap should return 'first wrap'")
		}

		if unwrapper2, ok := level1.(interface{ Unwrap() error }); ok {
			level2 := unwrapper2.Unwrap()
			if level2.Error() != "original" {
				t.Error("Second unwrap should return 'original'")
			}
		}
	}
}

// Benchmark tests

func BenchmarkResult_Ok(b *testing.B) {
	for i := 0; i < b.N; i++ {
		r := Ok(42)
		_ = r.Unwrap()
	}
}

func BenchmarkResult_Err(b *testing.B) {
	for i := 0; i < b.N; i++ {
		r := Err[int]("error")
		_ = r.UnwrapOr(0)
	}
}

func BenchmarkResult_Map(b *testing.B) {
	r := Ok(42)
	for i := 0; i < b.N; i++ {
		_ = r.Map(func(v int) int { return v * 2 })
	}
}

func BenchmarkResult_AndThen(b *testing.B) {
	r := Ok(42)
	for i := 0; i < b.N; i++ {
		_ = r.AndThen(func(v int) Result[int] { return Ok(v * 2) })
	}
}

func BenchmarkChainErrors(b *testing.B) {
	err1 := fmt.Errorf("error 1")
	err2 := fmt.Errorf("error 2")
	err3 := fmt.Errorf("error 3")

	for i := 0; i < b.N; i++ {
		_ = ChainErrors(err1, err2, err3)
	}
}

func BenchmarkWrapError(b *testing.B) {
	original := fmt.Errorf("original")
	for i := 0; i < b.N; i++ {
		_ = WrapError(original, "wrapped")
	}
}
