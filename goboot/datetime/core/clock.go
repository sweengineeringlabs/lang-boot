// Package core contains the implementation details for the datetime module.
package core

import (
	"sync"
	"time"

	"dev.engineeringlabs/goboot/datetime/api"
)

// SystemClock is a clock that uses the system time.
type SystemClock struct{}

// NewSystemClock creates a new SystemClock.
func NewSystemClock() *SystemClock {
	return &SystemClock{}
}

// Now returns the current time.
func (c *SystemClock) Now() time.Time {
	return time.Now()
}

// Since returns the duration since t.
func (c *SystemClock) Since(t time.Time) time.Duration {
	return time.Since(t)
}

// Until returns the duration until t.
func (c *SystemClock) Until(t time.Time) time.Duration {
	return time.Until(t)
}

// FrozenClock is a clock that returns a fixed time.
type FrozenClock struct {
	frozen time.Time
	mu     sync.RWMutex
}

// NewFrozenClock creates a new FrozenClock.
func NewFrozenClock(t time.Time) *FrozenClock {
	return &FrozenClock{frozen: t}
}

// Now returns the frozen time.
func (c *FrozenClock) Now() time.Time {
	c.mu.RLock()
	defer c.mu.RUnlock()
	return c.frozen
}

// Since returns the duration since t.
func (c *FrozenClock) Since(t time.Time) time.Duration {
	return c.Now().Sub(t)
}

// Until returns the duration until t.
func (c *FrozenClock) Until(t time.Time) time.Duration {
	return t.Sub(c.Now())
}

// Set sets the frozen time.
func (c *FrozenClock) Set(t time.Time) {
	c.mu.Lock()
	c.frozen = t
	c.mu.Unlock()
}

// Advance advances the frozen time by a duration.
func (c *FrozenClock) Advance(d time.Duration) {
	c.mu.Lock()
	c.frozen = c.frozen.Add(d)
	c.mu.Unlock()
}

// OffsetClock is a clock with an offset from system time.
type OffsetClock struct {
	offset time.Duration
}

// NewOffsetClock creates a new OffsetClock.
func NewOffsetClock(offset time.Duration) *OffsetClock {
	return &OffsetClock{offset: offset}
}

// Now returns the current time with offset.
func (c *OffsetClock) Now() time.Time {
	return time.Now().Add(c.offset)
}

// Since returns the duration since t.
func (c *OffsetClock) Since(t time.Time) time.Duration {
	return c.Now().Sub(t)
}

// Until returns the duration until t.
func (c *OffsetClock) Until(t time.Time) time.Duration {
	return t.Sub(c.Now())
}

// Parse parses a string into a time using the given format.
func Parse(format api.DateFormat, value string) (time.Time, error) {
	return time.Parse(string(format), value)
}

// Format formats a time using the given format.
func Format(t time.Time, format api.DateFormat) string {
	return t.Format(string(format))
}

// StartOfDay returns the start of the day for a time.
func StartOfDay(t time.Time) time.Time {
	return time.Date(t.Year(), t.Month(), t.Day(), 0, 0, 0, 0, t.Location())
}

// EndOfDay returns the end of the day for a time.
func EndOfDay(t time.Time) time.Time {
	return time.Date(t.Year(), t.Month(), t.Day(), 23, 59, 59, 999999999, t.Location())
}

// StartOfWeek returns the start of the week (Monday) for a time.
func StartOfWeek(t time.Time) time.Time {
	weekday := int(t.Weekday())
	if weekday == 0 {
		weekday = 7
	}
	return StartOfDay(t.AddDate(0, 0, 1-weekday))
}

// StartOfMonth returns the start of the month for a time.
func StartOfMonth(t time.Time) time.Time {
	return time.Date(t.Year(), t.Month(), 1, 0, 0, 0, 0, t.Location())
}

// EndOfMonth returns the end of the month for a time.
func EndOfMonth(t time.Time) time.Time {
	return StartOfMonth(t).AddDate(0, 1, 0).Add(-time.Nanosecond)
}

// StartOfYear returns the start of the year for a time.
func StartOfYear(t time.Time) time.Time {
	return time.Date(t.Year(), 1, 1, 0, 0, 0, 0, t.Location())
}

// EndOfYear returns the end of the year for a time.
func EndOfYear(t time.Time) time.Time {
	return time.Date(t.Year(), 12, 31, 23, 59, 59, 999999999, t.Location())
}

// IsWeekday returns true if the time is a weekday (Mon-Fri).
func IsWeekday(t time.Time) bool {
	weekday := t.Weekday()
	return weekday >= time.Monday && weekday <= time.Friday
}

// IsWeekend returns true if the time is a weekend (Sat-Sun).
func IsWeekend(t time.Time) bool {
	weekday := t.Weekday()
	return weekday == time.Saturday || weekday == time.Sunday
}

// AddBusinessDays adds business days to a time.
func AddBusinessDays(t time.Time, days int) time.Time {
	for days > 0 {
		t = t.AddDate(0, 0, 1)
		if IsWeekday(t) {
			days--
		}
	}
	return t
}

// DaysBetween returns the number of days between two times.
func DaysBetween(a, b time.Time) int {
	a = StartOfDay(a)
	b = StartOfDay(b)
	return int(b.Sub(a).Hours() / 24)
}

// Age calculates the age in years from a birthdate.
func Age(birthdate time.Time) int {
	now := time.Now()
	years := now.Year() - birthdate.Year()
	if now.YearDay() < birthdate.YearDay() {
		years--
	}
	return years
}

// Default clock
var DefaultClock api.Clock = NewSystemClock()

// Now returns the current time using the default clock.
func Now() time.Time {
	return DefaultClock.Now()
}
