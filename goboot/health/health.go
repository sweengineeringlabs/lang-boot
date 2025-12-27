// Package health provides health check utilities for the goboot framework.
//
// This module provides:
//   - API layer: Status, Check, Report, Checker interface
//   - Core layer: DefaultRegistry, PingChecker, TimeoutChecker
//
// Example:
//
//	import "dev.engineeringlabs/goboot/health"
//
//	registry := health.NewRegistry()
//
//	// Register a simple ping check
//	registry.Register(health.NewPingChecker("database", db.Ping))
//
//	// Register a custom check
//	registry.RegisterFunc("cache", func(ctx context.Context) health.Check {
//	    if cache.IsConnected() {
//	        return *health.NewCheck("cache", health.StatusUp)
//	    }
//	    return *health.NewCheck("cache", health.StatusDown)
//	})
//
//	// Get health report
//	report := registry.Check(ctx)
//	fmt.Println("Health:", report.Status)
package health

import (
	"dev.engineeringlabs/goboot/health/api"
	"dev.engineeringlabs/goboot/health/core"
)

// Re-export API types
type (
	// Status represents the health status.
	Status = api.Status
	// Check represents a health check result.
	Check = api.Check
	// Report represents the overall health report.
	Report = api.Report
	// Checker is the interface for health checkers.
	Checker = api.Checker
	// CheckerFunc is a function that implements Checker.
	CheckerFunc = api.CheckerFunc
	// Indicator is the interface for health indicators.
	Indicator = api.Indicator
	// Registry is the interface for registering health checkers.
	Registry = api.Registry
)

// Re-export API constants
const (
	StatusUp       = api.StatusUp
	StatusDown     = api.StatusDown
	StatusDegraded = api.StatusDegraded
	StatusUnknown  = api.StatusUnknown
)

// Re-export API functions
var (
	NewCheck  = api.NewCheck
	NewReport = api.NewReport
)

// Re-export Core types
type (
	// DefaultRegistry is the default health check registry.
	DefaultRegistry = core.DefaultRegistry
	// PingChecker checks if a ping function succeeds.
	PingChecker = core.PingChecker
	// TimeoutChecker wraps a checker with a timeout.
	TimeoutChecker = core.TimeoutChecker
	// CompositeChecker combines multiple checkers.
	CompositeChecker = core.CompositeChecker
)

// Re-export Core functions
var (
	NewRegistry         = core.NewRegistry
	NewPingChecker      = core.NewPingChecker
	NewTimeoutChecker   = core.NewTimeoutChecker
	NewCompositeChecker = core.NewCompositeChecker
	LivenessCheck       = core.LivenessCheck
	ReadinessCheck      = core.ReadinessCheck
)
