// Package testing provides testing utilities for the goboot framework.
//
// This module provides:
//   - API layer: Mock, Expectation, Assertion interfaces
//   - Core layer: BaseMock, FakeClock, Assertions
//
// Example:
//
//	import gotest "dev.engineeringlabs/goboot/testing"
//
//	func TestExample(t *testing.T) {
//	    // Assertions
//	    assert := gotest.NewAssertions(t)
//	    assert.Equal(1, 1)
//	    assert.NoError(nil)
//
//	    // Mock
//	    mock := gotest.NewMock()
//	    mock.On("GetUser", 1).Return(&User{Name: "John"}, nil)
//
//	    // Fake clock
//	    clock := gotest.NewFakeClock(time.Now())
//	    clock.Advance(time.Hour)
//	}
package testing

import (
	"dev.engineeringlabs/goboot/testing/api"
	"dev.engineeringlabs/goboot/testing/core"
)

// Re-export API types
type (
	// Mock is a generic mock interface.
	Mock = api.Mock
	// Expectation represents a method call expectation.
	Expectation = api.Expectation
	// TestingT is the interface for testing.T.
	TestingT = api.TestingT
	// Fixture represents test fixture data.
	Fixture = api.Fixture
	// Clock is a mockable clock interface.
	Clock = api.Clock
	// Ticker is a mockable ticker interface.
	Ticker = api.Ticker
	// Assertion represents an assertion helper.
	Assertion = api.Assertion
)

// Re-export Core types
type (
	// BaseMock is a base implementation for mocks.
	BaseMock = core.BaseMock
	// FakeClock is a fake clock for testing.
	FakeClock = core.FakeClock
	// Assertions provides assertion helpers.
	Assertions = core.Assertions
)

// Re-export Core functions
var (
	NewMock       = core.NewMock
	NewFakeClock  = core.NewFakeClock
	NewAssertions = core.NewAssertions
)
