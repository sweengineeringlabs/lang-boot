package core

import (
	"fmt"
	"testing"
	"time"
)

// Integration tests for testing utilities

func TestMock_ComplexScenario(t *testing.T) {
	// Simulating a user service mock
	mock := NewMock()

	// Setup expectations
	mock.On("GetUser", 1).Return(&testUser{ID: 1, Name: "Alice"}, nil)
	mock.On("GetUser", 2).Return(&testUser{ID: 2, Name: "Bob"}, nil)
	mock.On("GetUser", 999).Return(nil, fmt.Errorf("user not found"))
	mock.On("CreateUser", "Charlie").Return(&testUser{ID: 3, Name: "Charlie"}, nil).Once()
	mock.On("DeleteUser", 1).Return(nil).TimesN(2)

	mockT := &testingT{}

	// Simulate usage
	result := mock.Called("GetUser", 1)
	if result[0].(*testUser).Name != "Alice" {
		t.Error("Expected Alice")
	}

	result = mock.Called("GetUser", 2)
	if result[0].(*testUser).Name != "Bob" {
		t.Error("Expected Bob")
	}

	result = mock.Called("GetUser", 999)
	if result[1] == nil {
		t.Error("Expected error for non-existent user")
	}

	result = mock.Called("CreateUser", "Charlie")
	if result[0].(*testUser).Name != "Charlie" {
		t.Error("Expected Charlie")
	}

	mock.Called("DeleteUser", 1)
	mock.Called("DeleteUser", 1)

	// Assertions
	if !mock.AssertCalled(mockT, "GetUser", 1) {
		t.Error("GetUser(1) should be called")
	}
	if !mock.AssertNumberOfCalls(mockT, "GetUser", 3) {
		t.Error("GetUser should be called 3 times")
	}
	if !mock.AssertExpectations(mockT) {
		t.Error("All expectations should be met")
	}
}

func TestMock_RunCallback(t *testing.T) {
	mock := NewMock()

	callArgs := make([]any, 0)
	mock.On("Process", "item1").Run(func(args ...any) {
		callArgs = append(callArgs, args...)
	}).Return("processed1")

	mock.On("Process", "item2").Run(func(args ...any) {
		callArgs = append(callArgs, args...)
	}).Return("processed2")

	mock.Called("Process", "item1")
	mock.Called("Process", "item2")

	if len(callArgs) != 2 {
		t.Errorf("Expected 2 call recordings, got %d", len(callArgs))
	}
}

func TestFakeClock_TimeProgression(t *testing.T) {
	start := time.Date(2024, 1, 1, 0, 0, 0, 0, time.UTC)
	clock := NewFakeClock(start)

	// Track "events" at specific times
	type event struct {
		name string
		time time.Time
	}
	events := []event{}

	// Record first event
	events = append(events, event{"start", clock.Now()})

	// Advance 1 hour
	clock.Advance(time.Hour)
	events = append(events, event{"after-1h", clock.Now()})

	// Advance 1 day
	clock.Advance(24 * time.Hour)
	events = append(events, event{"after-1d", clock.Now()})

	// Verify durations
	duration := events[2].time.Sub(events[0].time)
	expected := 25 * time.Hour
	if duration != expected {
		t.Errorf("Expected %v duration, got %v", expected, duration)
	}
}

func TestFakeClock_Timer(t *testing.T) {
	start := time.Date(2024, 1, 1, 0, 0, 0, 0, time.UTC)
	clock := NewFakeClock(start)

	deadline := start.Add(5 * time.Second)

	// Check before deadline
	if clock.Until(deadline) != 5*time.Second {
		t.Error("Should be 5 seconds until deadline")
	}

	// Advance past deadline
	clock.Advance(10 * time.Second)

	if clock.Until(deadline) >= 0 {
		t.Error("Deadline should be in the past")
	}
}

func TestAssertions_Comprehensive(t *testing.T) {
	mockT := &testingT{}
	assert := NewAssertions(mockT)

	// Test all assertion types
	t.Run("Equal", func(t *testing.T) {
		if !assert.Equal(42, 42) {
			t.Error("Equal failed for same values")
		}
		if assert.Equal(42, 43) {
			t.Error("Equal should fail for different values")
		}
	})

	t.Run("NotEqual", func(t *testing.T) {
		if !assert.NotEqual(42, 43) {
			t.Error("NotEqual failed")
		}
	})

	t.Run("True", func(t *testing.T) {
		if !assert.True(true) {
			t.Error("True failed")
		}
	})

	t.Run("False", func(t *testing.T) {
		if !assert.False(false) {
			t.Error("False failed")
		}
	})

	t.Run("Nil", func(t *testing.T) {
		if !assert.Nil(nil) {
			t.Error("Nil failed")
		}
		var ptr *string
		if !assert.Nil(ptr) {
			t.Error("Nil failed for nil pointer")
		}
	})

	t.Run("NotNil", func(t *testing.T) {
		s := "test"
		if !assert.NotNil(&s) {
			t.Error("NotNil failed")
		}
	})

	t.Run("Contains", func(t *testing.T) {
		if !assert.Contains("hello world", "world") {
			t.Error("Contains failed")
		}
	})

	t.Run("Len", func(t *testing.T) {
		if !assert.Len([]int{1, 2, 3}, 3) {
			t.Error("Len failed")
		}
		if !assert.Len("hello", 5) {
			t.Error("Len failed for string")
		}
	})

	t.Run("Error", func(t *testing.T) {
		if !assert.Error(fmt.Errorf("err")) {
			t.Error("Error failed")
		}
	})

	t.Run("NoError", func(t *testing.T) {
		if !assert.NoError(nil) {
			t.Error("NoError failed")
		}
	})
}

func TestMock_PartialMatching(t *testing.T) {
	mock := NewMock()

	// Setup with different arguments
	mock.On("Query", "SELECT * FROM users").Return([]map[string]any{{"id": 1}}, nil)
	mock.On("Query", "SELECT * FROM posts").Return([]map[string]any{{"id": 1}}, nil)

	// Call matching
	result := mock.Called("Query", "SELECT * FROM users")
	if result[1] != nil {
		t.Error("Should not return error")
	}

	// Verify specific call
	mockT := &testingT{}
	if !mock.AssertCalled(mockT, "Query", "SELECT * FROM users") {
		t.Error("Should have been called")
	}
	if mock.AssertCalled(mockT, "Query", "DELETE FROM users") {
		t.Error("Should not have been called")
	}
}

func TestMock_CalledTimes(t *testing.T) {
	mock := NewMock()
	mock.On("Ping").Return("pong")

	// Call multiple times
	for i := 0; i < 5; i++ {
		mock.Called("Ping")
	}

	mockT := &testingT{}
	if !mock.AssertNumberOfCalls(mockT, "Ping", 5) {
		t.Error("Should be called 5 times")
	}
}

// Helper types for tests

type testUser struct {
	ID   int
	Name string
}

// Benchmark tests

func BenchmarkMock_Called(b *testing.B) {
	mock := NewMock()
	mock.On("Method", 123).Return("result")

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		mock.Called("Method", 123)
	}
}

func BenchmarkFakeClock_Now(b *testing.B) {
	clock := NewFakeClock(time.Now())
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		clock.Now()
	}
}

func BenchmarkFakeClock_Advance(b *testing.B) {
	clock := NewFakeClock(time.Now())
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		clock.Advance(time.Second)
	}
}

func BenchmarkAssertions_Equal(b *testing.B) {
	mockT := &testingT{}
	assert := NewAssertions(mockT)

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		assert.Equal(42, 42)
	}
}
