// Package core contains the implementation details for the health module.
package core

import (
	"context"
	"sync"
	"time"

	"dev.engineeringlabs/goboot/health/api"
)

// DefaultRegistry is the default health check registry.
type DefaultRegistry struct {
	checkers []api.Checker
	mu       sync.RWMutex
}

// NewRegistry creates a new DefaultRegistry.
func NewRegistry() *DefaultRegistry {
	return &DefaultRegistry{
		checkers: make([]api.Checker, 0),
	}
}

// Register registers a health checker.
func (r *DefaultRegistry) Register(checker api.Checker) {
	r.mu.Lock()
	r.checkers = append(r.checkers, checker)
	r.mu.Unlock()
}

// RegisterFunc registers a check function.
func (r *DefaultRegistry) RegisterFunc(name string, check api.CheckerFunc) {
	r.Register(&funcChecker{name: name, check: check})
}

// Check performs all health checks.
func (r *DefaultRegistry) Check(ctx context.Context) *api.Report {
	r.mu.RLock()
	checkers := make([]api.Checker, len(r.checkers))
	copy(checkers, r.checkers)
	r.mu.RUnlock()

	report := api.NewReport()

	// Run checks concurrently
	var wg sync.WaitGroup
	var mu sync.Mutex

	for _, checker := range checkers {
		wg.Add(1)
		go func(c api.Checker) {
			defer wg.Done()
			check := c.Check(ctx)
			mu.Lock()
			report.AddCheck(check)
			mu.Unlock()
		}(checker)
	}

	wg.Wait()
	return report
}

// Checkers returns all registered checkers.
func (r *DefaultRegistry) Checkers() []api.Checker {
	r.mu.RLock()
	defer r.mu.RUnlock()
	result := make([]api.Checker, len(r.checkers))
	copy(result, r.checkers)
	return result
}

type funcChecker struct {
	name  string
	check api.CheckerFunc
}

func (c *funcChecker) Name() string {
	return c.name
}

func (c *funcChecker) Check(ctx context.Context) api.Check {
	return c.check(ctx)
}

// PingChecker checks if a ping function succeeds.
type PingChecker struct {
	name string
	ping func(ctx context.Context) error
}

// NewPingChecker creates a new PingChecker.
func NewPingChecker(name string, ping func(ctx context.Context) error) *PingChecker {
	return &PingChecker{name: name, ping: ping}
}

// Name returns the name of the checker.
func (c *PingChecker) Name() string {
	return c.name
}

// Check performs the health check.
func (c *PingChecker) Check(ctx context.Context) api.Check {
	start := time.Now()
	err := c.ping(ctx)
	duration := time.Since(start)

	check := api.NewCheck(c.name, api.StatusUp)
	check.Duration = duration

	if err != nil {
		check.Status = api.StatusDown
		check.Message = err.Error()
	}

	return *check
}

// TimeoutChecker wraps a checker with a timeout.
type TimeoutChecker struct {
	checker api.Checker
	timeout time.Duration
}

// NewTimeoutChecker creates a new TimeoutChecker.
func NewTimeoutChecker(checker api.Checker, timeout time.Duration) *TimeoutChecker {
	return &TimeoutChecker{checker: checker, timeout: timeout}
}

// Name returns the name of the checker.
func (c *TimeoutChecker) Name() string {
	return c.checker.Name()
}

// Check performs the health check with a timeout.
func (c *TimeoutChecker) Check(ctx context.Context) api.Check {
	ctx, cancel := context.WithTimeout(ctx, c.timeout)
	defer cancel()

	done := make(chan api.Check, 1)
	go func() {
		done <- c.checker.Check(ctx)
	}()

	select {
	case check := <-done:
		return check
	case <-ctx.Done():
		return *api.NewCheck(c.checker.Name(), api.StatusDown).
			WithMessage("health check timed out")
	}
}

// CompositeChecker combines multiple checkers.
type CompositeChecker struct {
	name     string
	checkers []api.Checker
}

// NewCompositeChecker creates a new CompositeChecker.
func NewCompositeChecker(name string, checkers ...api.Checker) *CompositeChecker {
	return &CompositeChecker{name: name, checkers: checkers}
}

// Name returns the name of the checker.
func (c *CompositeChecker) Name() string {
	return c.name
}

// Check performs all sub-checks and returns a combined result.
func (c *CompositeChecker) Check(ctx context.Context) api.Check {
	start := time.Now()
	status := api.StatusUp
	details := make(map[string]any)

	for _, checker := range c.checkers {
		subCheck := checker.Check(ctx)
		details[checker.Name()] = subCheck.Status

		if subCheck.Status == api.StatusDown {
			status = api.StatusDown
		} else if subCheck.Status == api.StatusDegraded && status != api.StatusDown {
			status = api.StatusDegraded
		}
	}

	check := api.NewCheck(c.name, status)
	check.Duration = time.Since(start)
	check.Details = details
	return *check
}

// LivenessCheck returns a simple liveness check.
func LivenessCheck() api.CheckerFunc {
	return func(ctx context.Context) api.Check {
		return *api.NewCheck("liveness", api.StatusUp)
	}
}

// ReadinessCheck returns a readiness check based on a condition.
func ReadinessCheck(ready func() bool) api.CheckerFunc {
	return func(ctx context.Context) api.Check {
		if ready() {
			return *api.NewCheck("readiness", api.StatusUp)
		}
		return *api.NewCheck("readiness", api.StatusDown).
			WithMessage("application not ready")
	}
}
