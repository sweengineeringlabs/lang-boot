//go:build ignore

// Package main demonstrates the observability module usage.
package main

import (
	"bytes"
	"context"
	"fmt"
	"os"

	"dev.engineeringlabs/goboot/observability"
)

func main() {
	fmt.Println("=== Goboot Observability Module Example ===\n")

	// Example 1: JSON Logger
	fmt.Println("1. JSON Logger:")
	logger := observability.NewJSONLogger(os.Stdout, observability.LevelInfo)

	logger.Info("Application started",
		observability.String("version", "1.0.0"),
		observability.String("environment", "development"),
	)

	logger.Debug("This won't show (below INFO level)")

	logger.Warn("Deprecation warning",
		observability.String("feature", "old_api"),
		observability.String("replacement", "new_api"),
	)

	logger.Error("Connection failed",
		observability.Err(fmt.Errorf("connection refused")),
		observability.Int("attempt", 3),
	)

	// Example 2: Logger with context
	fmt.Println("\n2. Logger with context:")
	reqLogger := logger.With(
		observability.String("request_id", "abc-123"),
		observability.String("user_id", "user-456"),
	)

	reqLogger.Info("Processing request")
	reqLogger.Info("Request completed",
		observability.Duration("duration", 150*1000*1000), // 150ms in nanoseconds
	)

	// Example 3: Noop Logger (for testing)
	fmt.Println("\n3. Noop Logger (silent):")
	noopLogger := observability.NewNoopLogger()
	noopLogger.Info("This won't output anything")
	fmt.Println("   (no output from noop logger)")

	// Example 4: In-Memory Metrics
	fmt.Println("\n4. In-Memory Metrics:")
	metrics := observability.NewInMemoryMetrics()

	// Counter
	metrics.Counter("http_requests_total", 1, map[string]string{
		"method": "GET",
		"path":   "/api/users",
	})
	metrics.Counter("http_requests_total", 1, map[string]string{
		"method": "GET",
		"path":   "/api/users",
	})

	// Gauge
	metrics.Gauge("active_connections", 42, nil)

	// Histogram
	metrics.Histogram("request_duration_seconds", 0.125, map[string]string{
		"path": "/api/users",
	})
	metrics.Histogram("request_duration_seconds", 0.250, map[string]string{
		"path": "/api/users",
	})

	counter := metrics.GetCounter("http_requests_total", map[string]string{
		"method": "GET",
		"path":   "/api/users",
	})
	gauge := metrics.GetGauge("active_connections", nil)

	fmt.Printf("   http_requests_total (GET /api/users): %.0f\n", counter)
	fmt.Printf("   active_connections: %.0f\n", gauge)

	// Example 5: Tracer
	fmt.Println("\n5. Noop Tracer:")
	tracer := observability.NewNoopTracer()
	ctx := context.Background()

	ctx, span := tracer.Start(ctx, "process_request")
	span.SetAttribute("user_id", "123")
	span.End()
	fmt.Println("   Span created and ended (no output from noop tracer)")

	// Example 6: Logging to buffer (for testing)
	fmt.Println("\n6. Logging to buffer:")
	var buf bytes.Buffer
	bufLogger := observability.NewJSONLogger(&buf, observability.LevelDebug)
	bufLogger.Debug("Debug message")
	bufLogger.Info("Info message")
	fmt.Printf("   Captured output:\n%s", buf.String())
}
