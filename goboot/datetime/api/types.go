// Package api contains the public interfaces and types for the datetime module.
package api

import (
	"time"
)

// TimeZone represents a timezone.
type TimeZone string

const (
	// UTC timezone.
	UTC TimeZone = "UTC"
	// Local timezone.
	Local TimeZone = "Local"
)

// DateFormat represents a date format.
type DateFormat string

const (
	// ISO8601 date format.
	FormatISO8601 DateFormat = "2006-01-02T15:04:05Z07:00"
	// RFC3339 date format.
	FormatRFC3339 DateFormat = time.RFC3339
	// RFC3339Nano date format.
	FormatRFC3339Nano DateFormat = time.RFC3339Nano
	// DateOnly format.
	FormatDateOnly DateFormat = "2006-01-02"
	// TimeOnly format.
	FormatTimeOnly DateFormat = "15:04:05"
	// DateTime format.
	FormatDateTime DateFormat = "2006-01-02 15:04:05"
)

// Clock is the interface for time providers.
type Clock interface {
	// Now returns the current time.
	Now() time.Time
	// Since returns the duration since t.
	Since(t time.Time) time.Duration
	// Until returns the duration until t.
	Until(t time.Time) time.Duration
}

// Duration helpers
const (
	Nanosecond  = time.Nanosecond
	Microsecond = time.Microsecond
	Millisecond = time.Millisecond
	Second      = time.Second
	Minute      = time.Minute
	Hour        = time.Hour
	Day         = 24 * time.Hour
	Week        = 7 * Day
)

// Period represents a time period.
type Period struct {
	Start time.Time
	End   time.Time
}

// NewPeriod creates a new Period.
func NewPeriod(start, end time.Time) Period {
	return Period{Start: start, End: end}
}

// Duration returns the duration of the period.
func (p Period) Duration() time.Duration {
	return p.End.Sub(p.Start)
}

// Contains checks if a time is within the period.
func (p Period) Contains(t time.Time) bool {
	return !t.Before(p.Start) && !t.After(p.End)
}

// Overlaps checks if two periods overlap.
func (p Period) Overlaps(other Period) bool {
	return p.Start.Before(other.End) && p.End.After(other.Start)
}

// IsValid returns true if the period is valid (start before end).
func (p Period) IsValid() bool {
	return p.Start.Before(p.End)
}

// Scheduler is the interface for scheduling operations.
type Scheduler interface {
	// Schedule schedules a task to run at a specific time.
	Schedule(at time.Time, task func()) (cancel func())

	// ScheduleAfter schedules a task to run after a duration.
	ScheduleAfter(d time.Duration, task func()) (cancel func())

	// ScheduleRepeat schedules a task to run repeatedly.
	ScheduleRepeat(interval time.Duration, task func()) (cancel func())
}
