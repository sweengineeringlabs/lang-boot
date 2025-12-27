package core

import (
	"testing"
)

func TestSliceStream_Map(t *testing.T) {
	result := Of(1, 2, 3, 4, 5).
		Map(func(n int) int { return n * 2 }).
		Collect()

	expected := []int{2, 4, 6, 8, 10}
	if len(result) != len(expected) {
		t.Fatalf("Length mismatch: %d vs %d", len(result), len(expected))
	}
	for i, v := range result {
		if v != expected[i] {
			t.Errorf("Element %d: expected %d, got %d", i, expected[i], v)
		}
	}
}

func TestSliceStream_Filter(t *testing.T) {
	result := Of(1, 2, 3, 4, 5).
		Filter(func(n int) bool { return n%2 == 0 }).
		Collect()

	expected := []int{2, 4}
	if len(result) != len(expected) {
		t.Fatalf("Length mismatch: %d vs %d", len(result), len(expected))
	}
}

func TestSliceStream_Reduce(t *testing.T) {
	sum := Of(1, 2, 3, 4, 5).
		Reduce(0, func(a, b int) int { return a + b })

	if sum != 15 {
		t.Errorf("Expected 15, got %d", sum)
	}
}

func TestSliceStream_Chain(t *testing.T) {
	result := Of(1, 2, 3, 4, 5).
		Filter(func(n int) bool { return n%2 == 0 }).
		Map(func(n int) int { return n * 10 }).
		Collect()

	expected := []int{20, 40}
	if len(result) != len(expected) {
		t.Fatalf("Length mismatch")
	}
	for i, v := range result {
		if v != expected[i] {
			t.Errorf("Element %d: expected %d, got %d", i, expected[i], v)
		}
	}
}

func TestSliceStream_First(t *testing.T) {
	first, ok := Of(1, 2, 3).First()
	if !ok {
		t.Error("First should return true for non-empty stream")
	}
	if first != 1 {
		t.Errorf("Expected 1, got %d", first)
	}

	_, ok = Of[int]().First()
	if ok {
		t.Error("First should return false for empty stream")
	}
}

func TestSliceStream_Count(t *testing.T) {
	count := Of(1, 2, 3, 4, 5).Count()
	if count != 5 {
		t.Errorf("Expected 5, got %d", count)
	}
}

func TestSliceStream_Take(t *testing.T) {
	result := Of(1, 2, 3, 4, 5).Take(3).Collect()
	if len(result) != 3 {
		t.Errorf("Expected 3 elements, got %d", len(result))
	}
}

func TestSliceStream_Skip(t *testing.T) {
	result := Of(1, 2, 3, 4, 5).Skip(2).Collect()
	expected := []int{3, 4, 5}
	if len(result) != len(expected) {
		t.Fatalf("Length mismatch")
	}
}

func TestSliceStream_Any(t *testing.T) {
	hasEven := Of(1, 2, 3).Any(func(n int) bool { return n%2 == 0 })
	if !hasEven {
		t.Error("Should have even number")
	}

	hasNegative := Of(1, 2, 3).Any(func(n int) bool { return n < 0 })
	if hasNegative {
		t.Error("Should not have negative number")
	}
}

func TestSliceStream_All(t *testing.T) {
	allPositive := Of(1, 2, 3).All(func(n int) bool { return n > 0 })
	if !allPositive {
		t.Error("All should be positive")
	}

	allEven := Of(1, 2, 3).All(func(n int) bool { return n%2 == 0 })
	if allEven {
		t.Error("Not all are even")
	}
}

func TestRange(t *testing.T) {
	result := Range(1, 5).Collect()
	expected := []int{1, 2, 3, 4}
	if len(result) != len(expected) {
		t.Fatalf("Length mismatch")
	}
}

func TestGenerate(t *testing.T) {
	result := Generate(3, func(i int) string {
		return string(rune('a' + i))
	}).Collect()

	expected := []string{"a", "b", "c"}
	if len(result) != len(expected) {
		t.Fatalf("Length mismatch")
	}
}
