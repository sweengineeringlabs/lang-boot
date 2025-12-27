// Package api contains the public interfaces and types for the health module.
package api

import (
	"context"
	"time"
)

// Status represents the health status of a component.
type Status string

const (
	// StatusUp indicates the component is healthy.
	StatusUp Status = "UP"
	// StatusDown indicates the component is unhealthy.
	StatusDown Status = "DOWN"
	// StatusDegraded indicates the component is partially healthy.
	StatusDegraded Status = "DEGRADED"
	// StatusUnknown indicates the health status is unknown.
	StatusUnknown Status = "UNKNOWN"
)

// Check represents a single health check result.
type Check struct {
	Name      string         `json:"name"`
	Status    Status         `json:"status"`
	Duration  time.Duration  `json:"duration"`
	Message   string         `json:"message,omitempty"`
	Details   map[string]any `json:"details,omitempty"`
	Timestamp time.Time      `json:"timestamp"`
}

// NewCheck creates a new Check.
func NewCheck(name string, status Status) *Check {
	return &Check{
		Name:      name,
		Status:    status,
		Timestamp: time.Now(),
		Details:   make(map[string]any),
	}
}

// WithMessage adds a message to the check.
func (c *Check) WithMessage(msg string) *Check {
	c.Message = msg
	return c
}

// WithDetail adds a detail to the check.
func (c *Check) WithDetail(key string, value any) *Check {
	c.Details[key] = value
	return c
}

// IsHealthy returns true if the status is UP.
func (c *Check) IsHealthy() bool {
	return c.Status == StatusUp
}

// Report represents the overall health report.
type Report struct {
	Status    Status           `json:"status"`
	Checks    map[string]Check `json:"checks"`
	Timestamp time.Time        `json:"timestamp"`
}

// NewReport creates a new Report.
func NewReport() *Report {
	return &Report{
		Status:    StatusUp,
		Checks:    make(map[string]Check),
		Timestamp: time.Now(),
	}
}

// AddCheck adds a check to the report.
func (r *Report) AddCheck(check Check) {
	r.Checks[check.Name] = check
	r.updateStatus()
}

func (r *Report) updateStatus() {
	hasDown := false
	hasDegraded := false

	for _, check := range r.Checks {
		switch check.Status {
		case StatusDown:
			hasDown = true
		case StatusDegraded:
			hasDegraded = true
		}
	}

	if hasDown {
		r.Status = StatusDown
	} else if hasDegraded {
		r.Status = StatusDegraded
	} else {
		r.Status = StatusUp
	}
}

// IsHealthy returns true if the overall status is UP.
func (r *Report) IsHealthy() bool {
	return r.Status == StatusUp
}

// Checker is the interface for health checkers.
type Checker interface {
	// Name returns the name of the checker.
	Name() string
	// Check performs the health check.
	Check(ctx context.Context) Check
}

// CheckerFunc is a function that implements Checker.
type CheckerFunc func(ctx context.Context) Check

// Indicator is the interface for health indicators.
type Indicator interface {
	// Health returns the current health status.
	Health(ctx context.Context) *Report
}

// Registry is the interface for registering health checkers.
type Registry interface {
	// Register registers a health checker.
	Register(checker Checker)
	// RegisterFunc registers a check function.
	RegisterFunc(name string, check CheckerFunc)
	// Check performs all health checks.
	Check(ctx context.Context) *Report
	// Checkers returns all registered checkers.
	Checkers() []Checker
}
