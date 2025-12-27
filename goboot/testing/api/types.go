// Package api contains the public interfaces and types for the testing module.
package api

import (
	"sync"
	"time"
)

// Mock is a generic mock interface.
type Mock interface {
	// Called records that a method was called.
	Called(method string, args ...any) []any

	// On sets up an expectation.
	On(method string, args ...any) *Expectation

	// AssertExpectations asserts all expectations were met.
	AssertExpectations(t TestingT) bool

	// AssertCalled asserts a method was called.
	AssertCalled(t TestingT, method string, args ...any) bool

	// AssertNotCalled asserts a method was not called.
	AssertNotCalled(t TestingT, method string, args ...any) bool

	// Reset resets the mock.
	Reset()
}

// Expectation represents a method call expectation.
type Expectation struct {
	Method      string
	Args        []any
	ReturnArgs  []any
	RunFunc     func(args ...any)
	Times       int
	Called      int
	Mu          sync.Mutex
}

// Return sets the return values.
func (e *Expectation) Return(args ...any) *Expectation {
	e.ReturnArgs = args
	return e
}

// Run sets a function to run when called.
func (e *Expectation) Run(fn func(args ...any)) *Expectation {
	e.RunFunc = fn
	return e
}

// Once expects the call once.
func (e *Expectation) Once() *Expectation {
	e.Times = 1
	return e
}

// Twice expects the call twice.
func (e *Expectation) Twice() *Expectation {
	e.Times = 2
	return e
}

// TimesN expects the call n times.
func (e *Expectation) TimesN(n int) *Expectation {
	e.Times = n
	return e
}

// TestingT is the interface for testing.T.
type TestingT interface {
	Errorf(format string, args ...any)
	FailNow()
	Helper()
}

// Fixture represents test fixture data.
type Fixture struct {
	Name     string
	Data     map[string]any
	Setup    func() error
	Teardown func() error
}

// Clock is a mockable clock interface.
type Clock interface {
	Now() time.Time
	Since(t time.Time) time.Duration
	Until(t time.Time) time.Duration
	Sleep(d time.Duration)
	After(d time.Duration) <-chan time.Time
	NewTicker(d time.Duration) Ticker
}

// Ticker is a mockable ticker interface.
type Ticker interface {
	C() <-chan time.Time
	Stop()
}

// Assertion represents an assertion helper.
type Assertion interface {
	Equal(expected, actual any, msgAndArgs ...any) bool
	NotEqual(expected, actual any, msgAndArgs ...any) bool
	Nil(object any, msgAndArgs ...any) bool
	NotNil(object any, msgAndArgs ...any) bool
	True(value bool, msgAndArgs ...any) bool
	False(value bool, msgAndArgs ...any) bool
	Contains(s, contains any, msgAndArgs ...any) bool
	Len(object any, length int, msgAndArgs ...any) bool
	Empty(object any, msgAndArgs ...any) bool
	NotEmpty(object any, msgAndArgs ...any) bool
	Error(err error, msgAndArgs ...any) bool
	NoError(err error, msgAndArgs ...any) bool
	Panics(fn func(), msgAndArgs ...any) bool
}
