// Package api contains the public interfaces and types for the observability module.
package api

import (
	"context"
	"time"
)

// LogLevel represents the severity of a log message.
type LogLevel int

const (
	LevelDebug LogLevel = iota
	LevelInfo
	LevelWarn
	LevelError
)

// String returns the string representation of a LogLevel.
func (l LogLevel) String() string {
	switch l {
	case LevelDebug:
		return "DEBUG"
	case LevelInfo:
		return "INFO"
	case LevelWarn:
		return "WARN"
	case LevelError:
		return "ERROR"
	default:
		return "UNKNOWN"
	}
}

// Logger is the interface for structured logging.
type Logger interface {
	// Debug logs a debug message.
	Debug(msg string, fields ...Field)
	// Info logs an info message.
	Info(msg string, fields ...Field)
	// Warn logs a warning message.
	Warn(msg string, fields ...Field)
	// Error logs an error message.
	Error(msg string, fields ...Field)
	// With returns a logger with additional fields.
	With(fields ...Field) Logger
}

// Field represents a structured log field.
type Field struct {
	Key   string
	Value any
}

// String creates a string field.
func String(key, value string) Field {
	return Field{Key: key, Value: value}
}

// Int creates an integer field.
func Int(key string, value int) Field {
	return Field{Key: key, Value: value}
}

// Int64 creates an int64 field.
func Int64(key string, value int64) Field {
	return Field{Key: key, Value: value}
}

// Float64 creates a float64 field.
func Float64(key string, value float64) Field {
	return Field{Key: key, Value: value}
}

// Bool creates a boolean field.
func Bool(key string, value bool) Field {
	return Field{Key: key, Value: value}
}

// Err creates an error field.
func Err(err error) Field {
	if err == nil {
		return Field{Key: "error", Value: nil}
	}
	return Field{Key: "error", Value: err.Error()}
}

// Duration creates a duration field.
func Duration(key string, value time.Duration) Field {
	return Field{Key: key, Value: value}
}

// Any creates a field with any value.
func Any(key string, value any) Field {
	return Field{Key: key, Value: value}
}

// MetricType represents the type of a metric.
type MetricType int

const (
	MetricCounter MetricType = iota
	MetricGauge
	MetricHistogram
	MetricSummary
)

// Metric represents a single metric.
type Metric struct {
	Name   string
	Type   MetricType
	Value  float64
	Labels map[string]string
}

// Metrics is the interface for metrics collection.
type Metrics interface {
	// Counter increments a counter metric.
	Counter(name string, value float64, labels map[string]string)
	// Gauge sets a gauge metric.
	Gauge(name string, value float64, labels map[string]string)
	// Histogram records a histogram value.
	Histogram(name string, value float64, labels map[string]string)
}

// Span represents a trace span.
type Span interface {
	// End ends the span.
	End()
	// SetAttribute sets a span attribute.
	SetAttribute(key string, value any)
	// SetError marks the span as error.
	SetError(err error)
}

// Tracer is the interface for distributed tracing.
type Tracer interface {
	// Start starts a new span.
	Start(ctx context.Context, name string) (context.Context, Span)
	// Inject injects trace context into a carrier.
	Inject(ctx context.Context, carrier map[string]string)
	// Extract extracts trace context from a carrier.
	Extract(ctx context.Context, carrier map[string]string) context.Context
}
