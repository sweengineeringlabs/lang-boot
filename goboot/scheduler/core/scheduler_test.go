package core

import (
	"context"
	"sync/atomic"
	"testing"
	"time"

	"dev.engineeringlabs/goboot/scheduler/api"
)

func TestSimpleScheduler_ScheduleAndRun(t *testing.T) {
	scheduler := NewScheduler()
	ctx := context.Background()

	var count int32
	scheduler.ScheduleFunc("test", "@every 50ms", func(ctx context.Context, job *api.Job) error {
		atomic.AddInt32(&count, 1)
		return nil
	})

	scheduler.Start(ctx)
	time.Sleep(150 * time.Millisecond)
	scheduler.Stop()

	if atomic.LoadInt32(&count) < 2 {
		t.Errorf("Expected at least 2 runs, got %d", count)
	}
}

func TestSimpleScheduler_Unschedule(t *testing.T) {
	scheduler := NewScheduler()

	scheduler.ScheduleFunc("test", "@every 1s", func(ctx context.Context, job *api.Job) error {
		return nil
	})

	scheduler.Unschedule("test")

	jobs := scheduler.Jobs()
	if len(jobs) != 0 {
		t.Error("Job should be unscheduled")
	}
}

func TestDelayedTask(t *testing.T) {
	executed := false
	task := NewDelayedTask(50*time.Millisecond, func() {
		executed = true
	})
	_ = task

	time.Sleep(100 * time.Millisecond)

	if !executed {
		t.Error("Task should have executed")
	}
}

func TestDelayedTask_Cancel(t *testing.T) {
	executed := false
	task := NewDelayedTask(100*time.Millisecond, func() {
		executed = true
	})

	task.Cancel()
	time.Sleep(150 * time.Millisecond)

	if executed {
		t.Error("Cancelled task should not execute")
	}
}

func TestPeriodicTask(t *testing.T) {
	var count int32
	task := NewPeriodicTask(50*time.Millisecond, func() {
		atomic.AddInt32(&count, 1)
	})

	time.Sleep(130 * time.Millisecond)
	task.Stop()

	if atomic.LoadInt32(&count) < 2 {
		t.Errorf("Expected at least 2 runs, got %d", count)
	}
}

func TestPeriodicTask_Stop(t *testing.T) {
	var count int32
	task := NewPeriodicTask(50*time.Millisecond, func() {
		atomic.AddInt32(&count, 1)
	})

	task.Stop()
	time.Sleep(100 * time.Millisecond)

	if atomic.LoadInt32(&count) > 1 {
		t.Error("Stopped task should not run more")
	}
}
