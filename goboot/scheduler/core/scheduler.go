// Package core contains the implementation details for the scheduler module.
package core

import (
	"context"
	"fmt"
	"strconv"
	"strings"
	"sync"
	"time"

	"dev.engineeringlabs/goboot/scheduler/api"
)

// SimpleScheduler is a basic scheduler implementation.
type SimpleScheduler struct {
	jobs     map[string]*scheduledJob
	mu       sync.RWMutex
	running  bool
	stopChan chan struct{}
	wg       sync.WaitGroup
}

type scheduledJob struct {
	job      *api.Job
	handler  api.JobFunc
	interval time.Duration
	ticker   *time.Ticker
}

// NewScheduler creates a new SimpleScheduler.
func NewScheduler() *SimpleScheduler {
	return &SimpleScheduler{
		jobs:     make(map[string]*scheduledJob),
		stopChan: make(chan struct{}),
	}
}

// Schedule schedules a job.
func (s *SimpleScheduler) Schedule(job *api.Job, handler api.JobFunc) error {
	interval, err := parseSchedule(job.Schedule)
	if err != nil {
		return err
	}

	s.mu.Lock()
	s.jobs[job.ID] = &scheduledJob{
		job:      job,
		handler:  handler,
		interval: interval,
	}
	s.mu.Unlock()

	// If already running, start this job
	if s.running {
		s.startJob(job.ID)
	}

	return nil
}

// ScheduleFunc schedules a function.
func (s *SimpleScheduler) ScheduleFunc(name, schedule string, handler api.JobFunc) error {
	job := &api.Job{
		ID:       name,
		Name:     name,
		Schedule: schedule,
		Enabled:  true,
	}
	return s.Schedule(job, handler)
}

// Unschedule removes a job.
func (s *SimpleScheduler) Unschedule(jobID string) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	if sj, ok := s.jobs[jobID]; ok {
		if sj.ticker != nil {
			sj.ticker.Stop()
		}
		delete(s.jobs, jobID)
	}
	return nil
}

// Start starts the scheduler.
func (s *SimpleScheduler) Start(ctx context.Context) error {
	s.mu.Lock()
	if s.running {
		s.mu.Unlock()
		return nil
	}
	s.running = true
	s.stopChan = make(chan struct{})
	s.mu.Unlock()

	// Start all jobs
	s.mu.RLock()
	for jobID := range s.jobs {
		s.startJob(jobID)
	}
	s.mu.RUnlock()

	// Wait for context cancellation
	go func() {
		select {
		case <-ctx.Done():
			s.Stop()
		case <-s.stopChan:
		}
	}()

	return nil
}

func (s *SimpleScheduler) startJob(jobID string) {
	s.mu.RLock()
	sj, ok := s.jobs[jobID]
	s.mu.RUnlock()

	if !ok || !sj.job.Enabled {
		return
	}

	sj.ticker = time.NewTicker(sj.interval)
	sj.job.NextRun = time.Now().Add(sj.interval)

	s.wg.Add(1)
	go func() {
		defer s.wg.Done()
		for {
			select {
			case <-sj.ticker.C:
				s.runJob(sj)
			case <-s.stopChan:
				sj.ticker.Stop()
				return
			}
		}
	}()
}

func (s *SimpleScheduler) runJob(sj *scheduledJob) {
	ctx := context.Background()
	sj.job.LastRun = time.Now()
	sj.job.RunCount++
	sj.job.NextRun = time.Now().Add(sj.interval)

	if err := sj.handler(ctx, sj.job); err != nil {
		// Log error (in production, use proper logging)
	}
}

// Stop stops the scheduler.
func (s *SimpleScheduler) Stop() error {
	s.mu.Lock()
	if !s.running {
		s.mu.Unlock()
		return nil
	}
	s.running = false
	close(s.stopChan)
	s.mu.Unlock()

	s.wg.Wait()
	return nil
}

// Jobs returns all scheduled jobs.
func (s *SimpleScheduler) Jobs() []*api.Job {
	s.mu.RLock()
	defer s.mu.RUnlock()

	jobs := make([]*api.Job, 0, len(s.jobs))
	for _, sj := range s.jobs {
		jobs = append(jobs, sj.job)
	}
	return jobs
}

// RunNow runs a job immediately.
func (s *SimpleScheduler) RunNow(jobID string) error {
	s.mu.RLock()
	sj, ok := s.jobs[jobID]
	s.mu.RUnlock()

	if !ok {
		return fmt.Errorf("job not found: %s", jobID)
	}

	s.runJob(sj)
	return nil
}

// parseSchedule parses a schedule string to a duration.
func parseSchedule(schedule string) (time.Duration, error) {
	// Handle @every format
	if strings.HasPrefix(schedule, "@every ") {
		durationStr := strings.TrimPrefix(schedule, "@every ")
		return time.ParseDuration(durationStr)
	}

	// Handle simple cron-like patterns
	parts := strings.Fields(schedule)
	if len(parts) == 5 {
		// Parse minute field for simple interval
		if parts[0] == "*" && parts[1] == "*" {
			return time.Minute, nil
		}
		if parts[0] == "0" && parts[1] == "*" {
			return time.Hour, nil
		}
		if parts[0] == "0" && parts[1] == "0" {
			return 24 * time.Hour, nil
		}
		// Try to parse minute as interval
		if min, err := strconv.Atoi(parts[0]); err == nil && min > 0 {
			return time.Duration(min) * time.Minute, nil
		}
	}

	return 0, fmt.Errorf("unsupported schedule format: %s", schedule)
}

// DelayedTask runs a task after a delay.
type DelayedTask struct {
	timer  *time.Timer
	cancel chan struct{}
}

// NewDelayedTask creates a delayed task.
func NewDelayedTask(delay time.Duration, fn func()) *DelayedTask {
	task := &DelayedTask{
		cancel: make(chan struct{}),
	}

	task.timer = time.AfterFunc(delay, func() {
		select {
		case <-task.cancel:
			return
		default:
			fn()
		}
	})

	return task
}

// Cancel cancels the delayed task.
func (t *DelayedTask) Cancel() bool {
	close(t.cancel)
	return t.timer.Stop()
}

// PeriodicTask runs a task periodically.
type PeriodicTask struct {
	ticker   *time.Ticker
	stopChan chan struct{}
	stopped  bool
	mu       sync.Mutex
}

// NewPeriodicTask creates a periodic task.
func NewPeriodicTask(interval time.Duration, fn func()) *PeriodicTask {
	task := &PeriodicTask{
		ticker:   time.NewTicker(interval),
		stopChan: make(chan struct{}),
	}

	go func() {
		for {
			select {
			case <-task.ticker.C:
				fn()
			case <-task.stopChan:
				return
			}
		}
	}()

	return task
}

// Stop stops the periodic task.
func (t *PeriodicTask) Stop() {
	t.mu.Lock()
	defer t.mu.Unlock()

	if !t.stopped {
		t.stopped = true
		t.ticker.Stop()
		close(t.stopChan)
	}
}
