package core

import (
	"fmt"
	"testing"
	"time"
)

func TestBaseMock_On_Return(t *testing.T) {
	mock := NewMock()

	mock.On("GetUser", 1).Return("John", nil)

	result := mock.Called("GetUser", 1)
	if len(result) != 2 {
		t.Errorf("Expected 2 return values, got %d", len(result))
	}
	if result[0] != "John" {
		t.Errorf("Expected 'John', got %v", result[0])
	}
}

func TestBaseMock_AssertCalled(t *testing.T) {
	mock := NewMock()
	mockT := &testingT{}

	mock.On("Save", "data").Return(nil)
	mock.Called("Save", "data")

	if !mock.AssertCalled(mockT, "Save", "data") {
		t.Error("Should assert called")
	}
}

func TestBaseMock_AssertNotCalled(t *testing.T) {
	mock := NewMock()
	mockT := &testingT{}

	if !mock.AssertNotCalled(mockT, "Delete", "id") {
		t.Error("Should assert not called")
	}
}

func TestBaseMock_AssertExpectations(t *testing.T) {
	mock := NewMock()
	mockT := &testingT{}

	mock.On("Process").Return(nil).Once()
	mock.Called("Process")

	if !mock.AssertExpectations(mockT) {
		t.Error("Expectations should be met")
	}
}

func TestBaseMock_AssertExpectations_Fails(t *testing.T) {
	mock := NewMock()
	mockT := &testingT{}

	mock.On("Process").Return(nil).TimesN(2)
	mock.Called("Process") // Only called once

	if mock.AssertExpectations(mockT) {
		t.Error("Expectations should not be met")
	}
}

func TestBaseMock_Reset(t *testing.T) {
	mock := NewMock()

	mock.On("Method").Return("value")
	mock.Called("Method")

	mock.Reset()

	mockT := &testingT{}
	if !mock.AssertNotCalled(mockT, "Method") {
		t.Error("Should be reset")
	}
}

func TestExpectation_Run(t *testing.T) {
	mock := NewMock()
	called := false

	mock.On("Trigger").Run(func(args ...any) {
		called = true
	}).Return(nil)

	mock.Called("Trigger")

	if !called {
		t.Error("Run function should have been called")
	}
}

func TestFakeClock_Now(t *testing.T) {
	fixedTime := time.Date(2024, 1, 15, 10, 30, 0, 0, time.UTC)
	clock := NewFakeClock(fixedTime)

	if !clock.Now().Equal(fixedTime) {
		t.Error("Now should return fixed time")
	}
}

func TestFakeClock_Set(t *testing.T) {
	clock := NewFakeClock(time.Now())
	newTime := time.Date(2025, 6, 1, 0, 0, 0, 0, time.UTC)

	clock.Set(newTime)

	if !clock.Now().Equal(newTime) {
		t.Error("Should return set time")
	}
}

func TestFakeClock_Advance(t *testing.T) {
	start := time.Date(2024, 1, 1, 0, 0, 0, 0, time.UTC)
	clock := NewFakeClock(start)

	clock.Advance(time.Hour * 24)

	expected := time.Date(2024, 1, 2, 0, 0, 0, 0, time.UTC)
	if !clock.Now().Equal(expected) {
		t.Errorf("Expected %v, got %v", expected, clock.Now())
	}
}

func TestFakeClock_Since(t *testing.T) {
	now := time.Date(2024, 1, 15, 12, 0, 0, 0, time.UTC)
	clock := NewFakeClock(now)

	past := time.Date(2024, 1, 15, 10, 0, 0, 0, time.UTC)
	since := clock.Since(past)

	if since != 2*time.Hour {
		t.Errorf("Expected 2h, got %v", since)
	}
}

func TestFakeClock_Until(t *testing.T) {
	now := time.Date(2024, 1, 15, 12, 0, 0, 0, time.UTC)
	clock := NewFakeClock(now)

	future := time.Date(2024, 1, 15, 15, 0, 0, 0, time.UTC)
	until := clock.Until(future)

	if until != 3*time.Hour {
		t.Errorf("Expected 3h, got %v", until)
	}
}

func TestAssertions_Equal(t *testing.T) {
	mockT := &testingT{}
	assert := NewAssertions(mockT)

	if !assert.Equal(10, 10) {
		t.Error("10 should equal 10")
	}

	if assert.Equal(10, 20) {
		t.Error("10 should not equal 20")
	}
}

func TestAssertions_NotEqual(t *testing.T) {
	mockT := &testingT{}
	assert := NewAssertions(mockT)

	if !assert.NotEqual(10, 20) {
		t.Error("10 should not equal 20")
	}

	if assert.NotEqual(10, 10) {
		t.Error("10 equals 10")
	}
}

func TestAssertions_Nil(t *testing.T) {
	mockT := &testingT{}
	assert := NewAssertions(mockT)

	if !assert.Nil(nil) {
		t.Error("nil should be nil")
	}

	if assert.Nil("not nil") {
		t.Error("string should not be nil")
	}
}

func TestAssertions_NotNil(t *testing.T) {
	mockT := &testingT{}
	assert := NewAssertions(mockT)

	if !assert.NotNil("value") {
		t.Error("string should not be nil")
	}
}

func TestAssertions_True(t *testing.T) {
	mockT := &testingT{}
	assert := NewAssertions(mockT)

	if !assert.True(true) {
		t.Error("true should be true")
	}

	if assert.True(false) {
		t.Error("false should not be true")
	}
}

func TestAssertions_False(t *testing.T) {
	mockT := &testingT{}
	assert := NewAssertions(mockT)

	if !assert.False(false) {
		t.Error("false should be false")
	}

	if assert.False(true) {
		t.Error("true should not be false")
	}
}

func TestAssertions_Contains(t *testing.T) {
	mockT := &testingT{}
	assert := NewAssertions(mockT)

	if !assert.Contains("hello world", "world") {
		t.Error("Should contain 'world'")
	}

	if assert.Contains("hello", "world") {
		t.Error("Should not contain 'world'")
	}
}

func TestAssertions_Len(t *testing.T) {
	mockT := &testingT{}
	assert := NewAssertions(mockT)

	if !assert.Len([]int{1, 2, 3}, 3) {
		t.Error("Length should be 3")
	}
}

func TestAssertions_Error(t *testing.T) {
	mockT := &testingT{}
	assert := NewAssertions(mockT)

	err := fmt.Errorf("an error")
	if !assert.Error(err) {
		t.Error("Should be an error")
	}

	if assert.Error(nil) {
		t.Error("nil should not be an error")
	}
}

func TestAssertions_NoError(t *testing.T) {
	mockT := &testingT{}
	assert := NewAssertions(mockT)

	if !assert.NoError(nil) {
		t.Error("nil should be no error")
	}
}

// testingT is a mock implementation of api.TestingT
type testingT struct {
	errors []string
}

func (t *testingT) Errorf(format string, args ...any) {
	t.errors = append(t.errors, fmt.Sprintf(format, args...))
}

func (t *testingT) FailNow() {}

func (t *testingT) Helper() {}
