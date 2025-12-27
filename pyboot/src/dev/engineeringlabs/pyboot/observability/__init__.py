"""
Observability Module - Logging, metrics, and tracing.

This module provides unified observability infrastructure:

- Logging: Structured logging with context propagation
- Metrics: Counter, gauge, histogram, and timer metrics
- Tracing: Distributed tracing spans

Example:
    from dev.engineeringlabs.pyboot.observability import get_logger, metrics

    # Structured logging
    logger = get_logger("my-service")
    logger.info("Processing request", request_id="abc123")

    # Metrics
    counter = metrics.counter("requests_total")
    counter.increment()
"""

from dev.engineeringlabs.pyboot.observability.logging import (
    Logger,
    LogLevel,
    LogRecord,
    get_logger,
    configure_logging,
)

from dev.engineeringlabs.pyboot.observability.metrics import (
    Counter,
    Gauge,
    Histogram,
    Timer,
    get_metrics,
    configure_metrics,
)

from dev.engineeringlabs.pyboot.observability.tracing import (
    Span,
    SpanContext,
    Tracer,
    get_tracer,
    trace,
)

__all__ = [
    # Logging
    "Logger",
    "LogLevel",
    "LogRecord",
    "get_logger",
    "configure_logging",
    # Metrics
    "Counter",
    "Gauge",
    "Histogram",
    "Timer",
    "get_metrics",
    "configure_metrics",
    # Tracing
    "Span",
    "SpanContext",
    "Tracer",
    "get_tracer",
    "trace",
]
