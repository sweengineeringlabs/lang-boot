// Package scheduler provides task scheduling utilities for the goboot framework.
//
// This module provides:
//   - API layer: Job, Scheduler interface, cron expressions
//   - Core layer: SimpleScheduler, DelayedTask, PeriodicTask
//
// Example:
//
//	import "dev.engineeringlabs/goboot/scheduler"
//
//	s := scheduler.NewScheduler()
//
//	// Schedule with interval
//	s.ScheduleFunc("cleanup", "@every 1h", func(ctx context.Context, job *scheduler.Job) error {
//	    log.Println("Running cleanup")
//	    return nil
//	})
//
//	// Start scheduler
//	s.Start(ctx)
//	defer s.Stop()
//
//	// Delayed task
//	task := scheduler.NewDelayedTask(5*time.Second, func() {
//	    log.Println("Delayed task executed")
//	})
package scheduler

import (
	"dev.engineeringlabs/goboot/scheduler/api"
	"dev.engineeringlabs/goboot/scheduler/core"
)

// Re-export API types
type (
	// Job represents a scheduled job.
	Job = api.Job
	// JobFunc is the function signature for job handlers.
	JobFunc = api.JobFunc
	// JobResult represents the result of a job execution.
	JobResult = api.JobResult
	// Scheduler is the interface for task schedulers.
	Scheduler = api.Scheduler
	// CronSchedule represents a cron schedule.
	CronSchedule = api.CronSchedule
	// Ticker is the interface for time-based execution.
	Ticker = api.Ticker
)

// Re-export API constants
const (
	EveryMinute = api.EveryMinute
	EveryHour   = api.EveryHour
	EveryDay    = api.EveryDay
	EveryWeek   = api.EveryWeek
	EveryMonth  = api.EveryMonth
)

// Re-export API functions
var IntervalSchedule = api.IntervalSchedule

// Re-export Core types
type (
	// SimpleScheduler is a basic scheduler implementation.
	SimpleScheduler = core.SimpleScheduler
	// DelayedTask runs a task after a delay.
	DelayedTask = core.DelayedTask
	// PeriodicTask runs a task periodically.
	PeriodicTask = core.PeriodicTask
)

// Re-export Core functions
var (
	NewScheduler    = core.NewScheduler
	NewDelayedTask  = core.NewDelayedTask
	NewPeriodicTask = core.NewPeriodicTask
)
