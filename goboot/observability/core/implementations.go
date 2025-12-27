// Package core contains the implementation details for the observability module.
package core

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os"
	"sync"
	"time"

	"dev.engineeringlabs/goboot/observability/api"
)

// JSONLogger is a structured JSON logger.
type JSONLogger struct {
	writer io.Writer
	level  api.LogLevel
	fields []api.Field
	mu     sync.Mutex
}

// NewJSONLogger creates a new JSONLogger.
func NewJSONLogger(writer io.Writer, level api.LogLevel) *JSONLogger {
	if writer == nil {
		writer = os.Stdout
	}
	return &JSONLogger{
		writer: writer,
		level:  level,
		fields: make([]api.Field, 0),
	}
}

// Debug logs a debug message.
func (l *JSONLogger) Debug(msg string, fields ...api.Field) {
	l.log(api.LevelDebug, msg, fields...)
}

// Info logs an info message.
func (l *JSONLogger) Info(msg string, fields ...api.Field) {
	l.log(api.LevelInfo, msg, fields...)
}

// Warn logs a warning message.
func (l *JSONLogger) Warn(msg string, fields ...api.Field) {
	l.log(api.LevelWarn, msg, fields...)
}

// Error logs an error message.
func (l *JSONLogger) Error(msg string, fields ...api.Field) {
	l.log(api.LevelError, msg, fields...)
}

// With returns a logger with additional fields.
func (l *JSONLogger) With(fields ...api.Field) api.Logger {
	newFields := make([]api.Field, len(l.fields)+len(fields))
	copy(newFields, l.fields)
	copy(newFields[len(l.fields):], fields)
	return &JSONLogger{
		writer: l.writer,
		level:  l.level,
		fields: newFields,
	}
}

func (l *JSONLogger) log(level api.LogLevel, msg string, fields ...api.Field) {
	if level < l.level {
		return
	}

	entry := map[string]any{
		"timestamp": time.Now().UTC().Format(time.RFC3339Nano),
		"level":     level.String(),
		"message":   msg,
	}

	// Add default fields
	for _, f := range l.fields {
		entry[f.Key] = f.Value
	}

	// Add message-specific fields
	for _, f := range fields {
		entry[f.Key] = f.Value
	}

	data, _ := json.Marshal(entry)

	l.mu.Lock()
	fmt.Fprintln(l.writer, string(data))
	l.mu.Unlock()
}

// NoopLogger is a logger that does nothing.
type NoopLogger struct{}

// NewNoopLogger creates a new NoopLogger.
func NewNoopLogger() *NoopLogger {
	return &NoopLogger{}
}

func (l *NoopLogger) Debug(msg string, fields ...api.Field) {}
func (l *NoopLogger) Info(msg string, fields ...api.Field)  {}
func (l *NoopLogger) Warn(msg string, fields ...api.Field)  {}
func (l *NoopLogger) Error(msg string, fields ...api.Field) {}
func (l *NoopLogger) With(fields ...api.Field) api.Logger   { return l }

// InMemoryMetrics is an in-memory metrics implementation.
type InMemoryMetrics struct {
	counters   map[string]float64
	gauges     map[string]float64
	histograms map[string][]float64
	mu         sync.RWMutex
}

// NewInMemoryMetrics creates a new InMemoryMetrics.
func NewInMemoryMetrics() *InMemoryMetrics {
	return &InMemoryMetrics{
		counters:   make(map[string]float64),
		gauges:     make(map[string]float64),
		histograms: make(map[string][]float64),
	}
}

func (m *InMemoryMetrics) key(name string, labels map[string]string) string {
	// Sort keys for deterministic ordering
	key := name
	if len(labels) > 0 {
		keys := make([]string, 0, len(labels))
		for k := range labels {
			keys = append(keys, k)
		}
		// Simple sort
		for i := 0; i < len(keys)-1; i++ {
			for j := i + 1; j < len(keys); j++ {
				if keys[i] > keys[j] {
					keys[i], keys[j] = keys[j], keys[i]
				}
			}
		}
		for _, k := range keys {
			key += fmt.Sprintf(";%s=%s", k, labels[k])
		}
	}
	return key
}

// Counter increments a counter metric.
func (m *InMemoryMetrics) Counter(name string, value float64, labels map[string]string) {
	m.mu.Lock()
	defer m.mu.Unlock()
	key := m.key(name, labels)
	m.counters[key] += value
}

// Gauge sets a gauge metric.
func (m *InMemoryMetrics) Gauge(name string, value float64, labels map[string]string) {
	m.mu.Lock()
	defer m.mu.Unlock()
	key := m.key(name, labels)
	m.gauges[key] = value
}

// Histogram records a histogram value.
func (m *InMemoryMetrics) Histogram(name string, value float64, labels map[string]string) {
	m.mu.Lock()
	defer m.mu.Unlock()
	key := m.key(name, labels)
	m.histograms[key] = append(m.histograms[key], value)
}

// GetCounter returns the current counter value.
func (m *InMemoryMetrics) GetCounter(name string, labels map[string]string) float64 {
	m.mu.RLock()
	defer m.mu.RUnlock()
	return m.counters[m.key(name, labels)]
}

// GetGauge returns the current gauge value.
func (m *InMemoryMetrics) GetGauge(name string, labels map[string]string) float64 {
	m.mu.RLock()
	defer m.mu.RUnlock()
	return m.gauges[m.key(name, labels)]
}

// NoopSpan is a span that does nothing.
type NoopSpan struct{}

func (s *NoopSpan) End()                        {}
func (s *NoopSpan) SetAttribute(key string, value any) {}
func (s *NoopSpan) SetError(err error)          {}

// NoopTracer is a tracer that does nothing.
type NoopTracer struct{}

// NewNoopTracer creates a new NoopTracer.
func NewNoopTracer() *NoopTracer {
	return &NoopTracer{}
}

func (t *NoopTracer) Start(ctx context.Context, name string) (context.Context, api.Span) {
	return ctx, &NoopSpan{}
}

func (t *NoopTracer) Inject(ctx context.Context, carrier map[string]string) {}

func (t *NoopTracer) Extract(ctx context.Context, carrier map[string]string) context.Context {
	return ctx
}
