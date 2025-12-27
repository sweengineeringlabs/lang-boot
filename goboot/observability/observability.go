// Package observability provides logging, metrics, and tracing for the goboot framework.
//
// This module provides:
//   - API layer: Logger, Metrics, Tracer interfaces
//   - Core layer: JSONLogger, InMemoryMetrics, NoopTracer implementations
//
// Example:
//
//	import "dev.engineeringlabs/goboot/observability"
//
//	// Create a logger
//	logger := observability.NewJSONLogger(os.Stdout, observability.LevelInfo)
//	logger.Info("Application started", observability.String("version", "1.0.0"))
//
//	// Create metrics
//	metrics := observability.NewInMemoryMetrics()
//	metrics.Counter("requests_total", 1, map[string]string{"method": "GET"})
package observability

import (
	"dev.engineeringlabs/goboot/observability/api"
	"dev.engineeringlabs/goboot/observability/core"
)

// Re-export API types
type (
	// LogLevel represents the severity of a log message.
	LogLevel = api.LogLevel
	// Logger is the interface for structured logging.
	Logger = api.Logger
	// Field represents a structured log field.
	Field = api.Field
	// MetricType represents the type of a metric.
	MetricType = api.MetricType
	// Metric represents a single metric.
	Metric = api.Metric
	// Metrics is the interface for metrics collection.
	Metrics = api.Metrics
	// Span represents a trace span.
	Span = api.Span
	// Tracer is the interface for distributed tracing.
	Tracer = api.Tracer
)

// Re-export API constants
const (
	LevelDebug = api.LevelDebug
	LevelInfo  = api.LevelInfo
	LevelWarn  = api.LevelWarn
	LevelError = api.LevelError

	MetricCounter   = api.MetricCounter
	MetricGauge     = api.MetricGauge
	MetricHistogram = api.MetricHistogram
	MetricSummary   = api.MetricSummary
)

// Re-export API field constructors
var (
	String   = api.String
	Int      = api.Int
	Int64    = api.Int64
	Float64  = api.Float64
	Bool     = api.Bool
	Err      = api.Err
	Duration = api.Duration
	Any      = api.Any
)

// Re-export Core types
type (
	// JSONLogger is a structured JSON logger.
	JSONLogger = core.JSONLogger
	// NoopLogger is a logger that does nothing.
	NoopLogger = core.NoopLogger
	// InMemoryMetrics is an in-memory metrics implementation.
	InMemoryMetrics = core.InMemoryMetrics
	// NoopTracer is a tracer that does nothing.
	NoopTracer = core.NoopTracer
)

// Re-export Core functions
var (
	NewJSONLogger      = core.NewJSONLogger
	NewNoopLogger      = core.NewNoopLogger
	NewInMemoryMetrics = core.NewInMemoryMetrics
	NewNoopTracer      = core.NewNoopTracer
)
