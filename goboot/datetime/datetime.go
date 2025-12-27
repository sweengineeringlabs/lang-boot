// Package datetime provides datetime utilities for the goboot framework.
//
// This module provides:
//   - API layer: Clock, Period, DateFormat types
//   - Core layer: SystemClock, FrozenClock, date utilities
//
// Example:
//
//	import "dev.engineeringlabs/goboot/datetime"
//
//	// Using clocks
//	clock := datetime.NewSystemClock()
//	now := clock.Now()
//
//	// Frozen clock for testing
//	frozen := datetime.NewFrozenClock(time.Date(2024, 1, 1, 0, 0, 0, 0, time.UTC))
//	frozen.Advance(time.Hour)
//
//	// Date utilities
//	start := datetime.StartOfDay(time.Now())
//	end := datetime.EndOfMonth(time.Now())
//
//	// Formatting
//	formatted := datetime.Format(time.Now(), datetime.FormatISO8601)
package datetime

import (
	"dev.engineeringlabs/goboot/datetime/api"
	"dev.engineeringlabs/goboot/datetime/core"
)

// Re-export API types
type (
	// TimeZone represents a timezone.
	TimeZone = api.TimeZone
	// DateFormat represents a date format.
	DateFormat = api.DateFormat
	// Clock is the interface for time providers.
	Clock = api.Clock
	// Period represents a time period.
	Period = api.Period
	// Scheduler is the interface for scheduling operations.
	Scheduler = api.Scheduler
)

// Re-export API constants
const (
	UTC   = api.UTC
	Local = api.Local

	FormatISO8601     = api.FormatISO8601
	FormatRFC3339     = api.FormatRFC3339
	FormatRFC3339Nano = api.FormatRFC3339Nano
	FormatDateOnly    = api.FormatDateOnly
	FormatTimeOnly    = api.FormatTimeOnly
	FormatDateTime    = api.FormatDateTime

	Nanosecond  = api.Nanosecond
	Microsecond = api.Microsecond
	Millisecond = api.Millisecond
	Second      = api.Second
	Minute      = api.Minute
	Hour        = api.Hour
	Day         = api.Day
	Week        = api.Week
)

// Re-export API functions
var NewPeriod = api.NewPeriod

// Re-export Core types
type (
	// SystemClock uses the system time.
	SystemClock = core.SystemClock
	// FrozenClock returns a fixed time.
	FrozenClock = core.FrozenClock
	// OffsetClock adds an offset to system time.
	OffsetClock = core.OffsetClock
)

// Re-export Core functions
var (
	NewSystemClock  = core.NewSystemClock
	NewFrozenClock  = core.NewFrozenClock
	NewOffsetClock  = core.NewOffsetClock
	Parse           = core.Parse
	Format          = core.Format
	StartOfDay      = core.StartOfDay
	EndOfDay        = core.EndOfDay
	StartOfWeek     = core.StartOfWeek
	StartOfMonth    = core.StartOfMonth
	EndOfMonth      = core.EndOfMonth
	StartOfYear     = core.StartOfYear
	EndOfYear       = core.EndOfYear
	IsWeekday       = core.IsWeekday
	IsWeekend       = core.IsWeekend
	AddBusinessDays = core.AddBusinessDays
	DaysBetween     = core.DaysBetween
	Age             = core.Age
	Now             = core.Now
)

// DefaultClock is the default clock.
var DefaultClock = core.DefaultClock
