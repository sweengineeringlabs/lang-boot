// Package api contains the public interfaces and types for the scheduler module.
package api

import (
	"context"
	"time"
)

// Job represents a scheduled job.
type Job struct {
	ID          string
	Name        string
	Schedule    string // Cron expression or interval
	NextRun     time.Time
	LastRun     time.Time
	RunCount    int64
	Enabled     bool
	Data        map[string]any
}

// JobFunc is the function signature for job handlers.
type JobFunc func(ctx context.Context, job *Job) error

// JobResult represents the result of a job execution.
type JobResult struct {
	JobID     string
	StartTime time.Time
	EndTime   time.Time
	Duration  time.Duration
	Error     error
	Success   bool
}

// Scheduler is the interface for task schedulers.
type Scheduler interface {
	// Schedule schedules a job.
	Schedule(job *Job, handler JobFunc) error

	// ScheduleFunc schedules a function with a name and schedule.
	ScheduleFunc(name, schedule string, handler JobFunc) error

	// Unschedule removes a job.
	Unschedule(jobID string) error

	// Start starts the scheduler.
	Start(ctx context.Context) error

	// Stop stops the scheduler.
	Stop() error

	// Jobs returns all scheduled jobs.
	Jobs() []*Job

	// RunNow runs a job immediately.
	RunNow(jobID string) error
}

// CronSchedule represents a cron schedule.
type CronSchedule struct {
	Minute     string // 0-59 or *
	Hour       string // 0-23 or *
	DayOfMonth string // 1-31 or *
	Month      string // 1-12 or *
	DayOfWeek  string // 0-6 (Sunday=0) or *
}

// String returns the cron expression.
func (c CronSchedule) String() string {
	return c.Minute + " " + c.Hour + " " + c.DayOfMonth + " " + c.Month + " " + c.DayOfWeek
}

// Common schedules
const (
	// EveryMinute runs every minute.
	EveryMinute = "* * * * *"
	// EveryHour runs every hour.
	EveryHour = "0 * * * *"
	// EveryDay runs every day at midnight.
	EveryDay = "0 0 * * *"
	// EveryWeek runs every Sunday at midnight.
	EveryWeek = "0 0 * * 0"
	// EveryMonth runs on the first of every month.
	EveryMonth = "0 0 1 * *"
)

// IntervalSchedule creates an interval-based schedule string.
func IntervalSchedule(d time.Duration) string {
	return "@every " + d.String()
}

// Ticker is the interface for time-based execution.
type Ticker interface {
	// C returns the ticker channel.
	C() <-chan time.Time

	// Stop stops the ticker.
	Stop()
}
