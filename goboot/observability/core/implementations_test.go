package core

import (
	"bytes"
	"testing"

	"dev.engineeringlabs/goboot/observability/api"
)

func TestJSONLogger_Levels(t *testing.T) {
	// If logger is configured at a level, it should log messages at that level and above
	// Level order: DEBUG (0) < INFO (1) < WARN (2) < ERROR (3)
	tests := []struct {
		configLevel api.LogLevel // Logger's configured level
		msgLevel    api.LogLevel // Level of the message being logged
		expected    bool         // Should the message be logged?
	}{
		// Logger at DEBUG - logs everything
		{api.LevelDebug, api.LevelDebug, true},
		{api.LevelDebug, api.LevelInfo, true},
		{api.LevelDebug, api.LevelWarn, true},
		{api.LevelDebug, api.LevelError, true},
		// Logger at INFO - logs INFO, WARN, ERROR
		{api.LevelInfo, api.LevelDebug, false},
		{api.LevelInfo, api.LevelInfo, true},
		{api.LevelInfo, api.LevelWarn, true},
		{api.LevelInfo, api.LevelError, true},
		// Logger at WARN - logs WARN, ERROR
		{api.LevelWarn, api.LevelDebug, false},
		{api.LevelWarn, api.LevelInfo, false},
		{api.LevelWarn, api.LevelWarn, true},
		{api.LevelWarn, api.LevelError, true},
		// Logger at ERROR - logs only ERROR
		{api.LevelError, api.LevelDebug, false},
		{api.LevelError, api.LevelInfo, false},
		{api.LevelError, api.LevelWarn, false},
		{api.LevelError, api.LevelError, true},
	}

	for _, tt := range tests {
		var buf bytes.Buffer
		logger := NewJSONLogger(&buf, tt.configLevel)

		switch tt.msgLevel {
		case api.LevelDebug:
			logger.Debug("test")
		case api.LevelInfo:
			logger.Info("test")
		case api.LevelWarn:
			logger.Warn("test")
		case api.LevelError:
			logger.Error("test")
		}

		hasOutput := buf.Len() > 0
		if hasOutput != tt.expected {
			t.Errorf("Logger level %v, log level %v: expected output=%v, got %v",
				tt.configLevel, tt.msgLevel, tt.expected, hasOutput)
		}
	}
}

func TestJSONLogger_Fields(t *testing.T) {
	var buf bytes.Buffer
	logger := NewJSONLogger(&buf, api.LevelInfo)

	logger.Info("test message",
		api.String("key", "value"),
		api.Int("count", 42),
		api.Bool("active", true),
	)

	output := buf.String()
	if output == "" {
		t.Error("Expected output")
	}

	// Check that output contains expected fields
	if !bytes.Contains(buf.Bytes(), []byte(`"key":"value"`)) {
		t.Error("Output should contain key field")
	}
	if !bytes.Contains(buf.Bytes(), []byte(`"count":42`)) {
		t.Error("Output should contain count field")
	}
}

func TestJSONLogger_With(t *testing.T) {
	var buf bytes.Buffer
	logger := NewJSONLogger(&buf, api.LevelInfo)

	contextLogger := logger.With(api.String("request_id", "123"))
	contextLogger.Info("test")

	if !bytes.Contains(buf.Bytes(), []byte(`"request_id":"123"`)) {
		t.Error("Contextual logger should include preset fields")
	}
}

func TestNoopLogger(t *testing.T) {
	// NoopLogger should not panic and produce no output
	logger := NewNoopLogger()

	logger.Debug("test", api.String("key", "value"))
	logger.Info("test")
	logger.Warn("test")
	logger.Error("test")

	contextLogger := logger.With(api.String("key", "value"))
	contextLogger.Info("test")
}

func TestInMemoryMetrics_Counter(t *testing.T) {
	metrics := NewInMemoryMetrics()

	labels := map[string]string{"method": "GET", "path": "/users"}

	metrics.Counter("requests_total", 1, labels)
	metrics.Counter("requests_total", 1, labels)
	metrics.Counter("requests_total", 1, labels)

	count := metrics.GetCounter("requests_total", labels)
	if count != 3 {
		t.Errorf("Expected 3, got %f", count)
	}
}

func TestInMemoryMetrics_Gauge(t *testing.T) {
	metrics := NewInMemoryMetrics()

	metrics.Gauge("active_connections", 10, nil)
	metrics.Gauge("active_connections", 15, nil)

	value := metrics.GetGauge("active_connections", nil)
	if value != 15 {
		t.Errorf("Expected 15, got %f", value)
	}
}

func TestInMemoryMetrics_Histogram(t *testing.T) {
	metrics := NewInMemoryMetrics()

	metrics.Histogram("request_duration", 0.1, nil)
	metrics.Histogram("request_duration", 0.2, nil)
	metrics.Histogram("request_duration", 0.3, nil)

	// Histograms are recorded - just verify no panic
	// The implementation may vary so we don't check specific values
}

func TestInMemoryMetrics_DifferentLabels(t *testing.T) {
	metrics := NewInMemoryMetrics()

	metrics.Counter("http_requests", 1, map[string]string{"method": "GET"})
	metrics.Counter("http_requests", 1, map[string]string{"method": "GET"})
	metrics.Counter("http_requests", 1, map[string]string{"method": "POST"})

	getCount := metrics.GetCounter("http_requests", map[string]string{"method": "GET"})
	postCount := metrics.GetCounter("http_requests", map[string]string{"method": "POST"})

	if getCount != 2 {
		t.Errorf("GET count should be 2, got %f", getCount)
	}
	if postCount != 1 {
		t.Errorf("POST count should be 1, got %f", postCount)
	}
}

func TestNoopTracer(t *testing.T) {
	tracer := NewNoopTracer()

	// Should not panic
	ctx, span := tracer.Start(nil, "test-span")
	span.SetAttribute("key", "value")
	span.End()

	if ctx != nil {
		t.Error("NoopTracer should return nil context")
	}
}

func TestFieldConstructors(t *testing.T) {
	t.Run("String", func(t *testing.T) {
		f := api.String("key", "value")
		if f.Key != "key" || f.Value != "value" {
			t.Error("String field not correct")
		}
	})

	t.Run("Int", func(t *testing.T) {
		f := api.Int("count", 42)
		if f.Key != "count" || f.Value != 42 {
			t.Error("Int field not correct")
		}
	})

	t.Run("Bool", func(t *testing.T) {
		f := api.Bool("active", true)
		if f.Key != "active" || f.Value != true {
			t.Error("Bool field not correct")
		}
	})

	t.Run("Float64", func(t *testing.T) {
		f := api.Float64("score", 3.14)
		if f.Key != "score" || f.Value != 3.14 {
			t.Error("Float64 field not correct")
		}
	})

	t.Run("Err", func(t *testing.T) {
		err := &testError{"test error"}
		f := api.Err(err)
		if f.Key != "error" {
			t.Error("Err field key not correct")
		}
	})
}

type testError struct {
	msg string
}

func (e *testError) Error() string {
	return e.msg
}
