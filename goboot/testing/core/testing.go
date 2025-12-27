// Package core contains the implementation details for the testing module.
package core

import (
	"fmt"
	"reflect"
	"sync"
	"time"

	"dev.engineeringlabs/goboot/testing/api"
)

// BaseMock is a base implementation for mocks.
type BaseMock struct {
	expectations []*api.Expectation
	calls        []call
	mu           sync.RWMutex
}

type call struct {
	method string
	args   []any
}

// NewMock creates a new BaseMock.
func NewMock() *BaseMock {
	return &BaseMock{
		expectations: make([]*api.Expectation, 0),
		calls:        make([]call, 0),
	}
}

// Called records that a method was called.
func (m *BaseMock) Called(method string, args ...any) []any {
	m.mu.Lock()
	m.calls = append(m.calls, call{method: method, args: args})
	m.mu.Unlock()

	// Find matching expectation
	m.mu.RLock()
	defer m.mu.RUnlock()

	for _, exp := range m.expectations {
		if exp.Method == method && matchArgs(exp.Args, args) {
			exp.Mu.Lock()
			exp.Called++
			exp.Mu.Unlock()

			if exp.RunFunc != nil {
				exp.RunFunc(args...)
			}
			return exp.ReturnArgs
		}
	}

	return nil
}

// On sets up an expectation.
func (m *BaseMock) On(method string, args ...any) *api.Expectation {
	exp := &api.Expectation{
		Method: method,
		Args:   args,
		Times:  -1, // -1 means any number of times
	}

	m.mu.Lock()
	m.expectations = append(m.expectations, exp)
	m.mu.Unlock()

	return exp
}

// AssertExpectations asserts all expectations were met.
func (m *BaseMock) AssertExpectations(t api.TestingT) bool {
	t.Helper()
	m.mu.RLock()
	defer m.mu.RUnlock()

	success := true
	for _, exp := range m.expectations {
		exp.Mu.Lock()
		called := exp.Called
		exp.Mu.Unlock()

		if exp.Times >= 0 && called != exp.Times {
			t.Errorf("Expected %s to be called %d times, was called %d times",
				exp.Method, exp.Times, called)
			success = false
		}
	}
	return success
}

// AssertCalled asserts a method was called.
func (m *BaseMock) AssertCalled(t api.TestingT, method string, args ...any) bool {
	t.Helper()
	m.mu.RLock()
	defer m.mu.RUnlock()

	for _, c := range m.calls {
		if c.method == method && matchArgs(c.args, args) {
			return true
		}
	}

	t.Errorf("Expected %s to be called with %v", method, args)
	return false
}

// AssertNotCalled asserts a method was not called.
func (m *BaseMock) AssertNotCalled(t api.TestingT, method string, args ...any) bool {
	t.Helper()
	m.mu.RLock()
	defer m.mu.RUnlock()

	for _, c := range m.calls {
		if c.method == method && (len(args) == 0 || matchArgs(c.args, args)) {
			t.Errorf("Expected %s not to be called", method)
			return false
		}
	}
	return true
}

// AssertNumberOfCalls asserts a method was called a specific number of times.
func (m *BaseMock) AssertNumberOfCalls(t api.TestingT, method string, expected int) bool {
	t.Helper()
	m.mu.RLock()
	defer m.mu.RUnlock()

	count := 0
	for _, c := range m.calls {
		if c.method == method {
			count++
		}
	}

	if count != expected {
		t.Errorf("Expected %s to be called %d times, was called %d times", method, expected, count)
		return false
	}
	return true
}

// Reset resets the mock.
func (m *BaseMock) Reset() {
	m.mu.Lock()
	m.expectations = make([]*api.Expectation, 0)
	m.calls = make([]call, 0)
	m.mu.Unlock()
}

func matchArgs(expected, actual []any) bool {
	if len(expected) != len(actual) {
		return false
	}
	for i := range expected {
		if !reflect.DeepEqual(expected[i], actual[i]) {
			return false
		}
	}
	return true
}

// FakeClock is a fake clock for testing.
type FakeClock struct {
	now time.Time
	mu  sync.RWMutex
}

// NewFakeClock creates a new FakeClock.
func NewFakeClock(now time.Time) *FakeClock {
	return &FakeClock{now: now}
}

// Now returns the current fake time.
func (c *FakeClock) Now() time.Time {
	c.mu.RLock()
	defer c.mu.RUnlock()
	return c.now
}

// Since returns the duration since t.
func (c *FakeClock) Since(t time.Time) time.Duration {
	return c.Now().Sub(t)
}

// Until returns the duration until t.
func (c *FakeClock) Until(t time.Time) time.Duration {
	return t.Sub(c.Now())
}

// Sleep does nothing in fake clock.
func (c *FakeClock) Sleep(d time.Duration) {
	c.Advance(d)
}

// After returns a channel that receives after duration (immediately for fake).
func (c *FakeClock) After(d time.Duration) <-chan time.Time {
	ch := make(chan time.Time, 1)
	ch <- c.Now().Add(d)
	return ch
}

// NewTicker creates a fake ticker.
func (c *FakeClock) NewTicker(d time.Duration) api.Ticker {
	return &fakeTicker{ch: make(chan time.Time)}
}

// Set sets the fake time.
func (c *FakeClock) Set(t time.Time) {
	c.mu.Lock()
	c.now = t
	c.mu.Unlock()
}

// Advance advances the fake time.
func (c *FakeClock) Advance(d time.Duration) {
	c.mu.Lock()
	c.now = c.now.Add(d)
	c.mu.Unlock()
}

type fakeTicker struct {
	ch chan time.Time
}

func (t *fakeTicker) C() <-chan time.Time {
	return t.ch
}

func (t *fakeTicker) Stop() {
	close(t.ch)
}

// Assertions provides assertion helpers.
type Assertions struct {
	t api.TestingT
}

// NewAssertions creates a new Assertions.
func NewAssertions(t api.TestingT) *Assertions {
	return &Assertions{t: t}
}

// Equal asserts equality.
func (a *Assertions) Equal(expected, actual any, msgAndArgs ...any) bool {
	a.t.Helper()
	if !reflect.DeepEqual(expected, actual) {
		a.t.Errorf("Expected %v, got %v. %s", expected, actual, formatMsg(msgAndArgs))
		return false
	}
	return true
}

// NotEqual asserts inequality.
func (a *Assertions) NotEqual(expected, actual any, msgAndArgs ...any) bool {
	a.t.Helper()
	if reflect.DeepEqual(expected, actual) {
		a.t.Errorf("Expected values to be different. %s", formatMsg(msgAndArgs))
		return false
	}
	return true
}

// Nil asserts nil.
func (a *Assertions) Nil(object any, msgAndArgs ...any) bool {
	a.t.Helper()
	if !isNil(object) {
		a.t.Errorf("Expected nil, got %v. %s", object, formatMsg(msgAndArgs))
		return false
	}
	return true
}

// NotNil asserts not nil.
func (a *Assertions) NotNil(object any, msgAndArgs ...any) bool {
	a.t.Helper()
	if isNil(object) {
		a.t.Errorf("Expected not nil. %s", formatMsg(msgAndArgs))
		return false
	}
	return true
}

// True asserts true.
func (a *Assertions) True(value bool, msgAndArgs ...any) bool {
	a.t.Helper()
	if !value {
		a.t.Errorf("Expected true. %s", formatMsg(msgAndArgs))
		return false
	}
	return true
}

// False asserts false.
func (a *Assertions) False(value bool, msgAndArgs ...any) bool {
	a.t.Helper()
	if value {
		a.t.Errorf("Expected false. %s", formatMsg(msgAndArgs))
		return false
	}
	return true
}

// Contains asserts contains.
func (a *Assertions) Contains(s, contains any, msgAndArgs ...any) bool {
	a.t.Helper()
	sStr, ok1 := s.(string)
	containsStr, ok2 := contains.(string)
	if ok1 && ok2 {
		if len(sStr) >= len(containsStr) {
			for i := 0; i <= len(sStr)-len(containsStr); i++ {
				if sStr[i:i+len(containsStr)] == containsStr {
					return true
				}
			}
		}
	}
	a.t.Errorf("Expected %v to contain %v. %s", s, contains, formatMsg(msgAndArgs))
	return false
}

// Len asserts length.
func (a *Assertions) Len(object any, length int, msgAndArgs ...any) bool {
	a.t.Helper()
	v := reflect.ValueOf(object)
	if v.Len() != length {
		a.t.Errorf("Expected length %d, got %d. %s", length, v.Len(), formatMsg(msgAndArgs))
		return false
	}
	return true
}

// Empty asserts empty.
func (a *Assertions) Empty(object any, msgAndArgs ...any) bool {
	a.t.Helper()
	v := reflect.ValueOf(object)
	if v.Len() != 0 {
		a.t.Errorf("Expected empty, got length %d. %s", v.Len(), formatMsg(msgAndArgs))
		return false
	}
	return true
}

// NotEmpty asserts not empty.
func (a *Assertions) NotEmpty(object any, msgAndArgs ...any) bool {
	a.t.Helper()
	v := reflect.ValueOf(object)
	if v.Len() == 0 {
		a.t.Errorf("Expected not empty. %s", formatMsg(msgAndArgs))
		return false
	}
	return true
}

// Error asserts error.
func (a *Assertions) Error(err error, msgAndArgs ...any) bool {
	a.t.Helper()
	if err == nil {
		a.t.Errorf("Expected error. %s", formatMsg(msgAndArgs))
		return false
	}
	return true
}

// NoError asserts no error.
func (a *Assertions) NoError(err error, msgAndArgs ...any) bool {
	a.t.Helper()
	if err != nil {
		a.t.Errorf("Expected no error, got %v. %s", err, formatMsg(msgAndArgs))
		return false
	}
	return true
}

// Panics asserts panic.
func (a *Assertions) Panics(fn func(), msgAndArgs ...any) bool {
	a.t.Helper()
	defer func() {
		if r := recover(); r == nil {
			a.t.Errorf("Expected panic. %s", formatMsg(msgAndArgs))
		}
	}()
	fn()
	return true
}

func isNil(object any) bool {
	if object == nil {
		return true
	}
	v := reflect.ValueOf(object)
	switch v.Kind() {
	case reflect.Chan, reflect.Func, reflect.Map, reflect.Ptr, reflect.Interface, reflect.Slice:
		return v.IsNil()
	}
	return false
}

func formatMsg(msgAndArgs []any) string {
	if len(msgAndArgs) == 0 {
		return ""
	}
	if len(msgAndArgs) == 1 {
		return fmt.Sprint(msgAndArgs[0])
	}
	return fmt.Sprintf(msgAndArgs[0].(string), msgAndArgs[1:]...)
}
