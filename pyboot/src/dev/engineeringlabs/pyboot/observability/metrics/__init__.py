"""Metrics module for application instrumentation."""

import time
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from typing import Any


class Metric(ABC):
    """Base class for all metrics."""

    def __init__(self, name: str, description: str = "", labels: dict[str, str] | None = None):
        self._name = name
        self._description = description
        self._labels = labels or {}

    @property
    def name(self) -> str:
        """Get the metric name."""
        return self._name

    @property
    def description(self) -> str:
        """Get the metric description."""
        return self._description

    @property
    def labels(self) -> dict[str, str]:
        """Get the metric labels."""
        return self._labels

    @abstractmethod
    def reset(self) -> None:
        """Reset the metric."""
        ...


class Counter(Metric):
    """
    Counter metric - monotonically increasing value.

    Example:
        counter = Counter("http_requests_total", description="Total HTTP requests")
        counter.increment()
        counter.add(5)
    """

    def __init__(
        self,
        name: str,
        description: str = "",
        labels: dict[str, str] | None = None,
    ):
        super().__init__(name, description, labels)
        self._value: float = 0

    @property
    def value(self) -> float:
        """Get the current counter value."""
        return self._value

    def increment(self, value: float = 1) -> None:
        """Increment the counter."""
        if value < 0:
            raise ValueError("Counter can only be incremented by positive values")
        self._value += value

    def add(self, value: float) -> None:
        """Add to the counter (alias for increment)."""
        self.increment(value)

    def reset(self) -> None:
        """Reset the counter to zero."""
        self._value = 0


class Gauge(Metric):
    """
    Gauge metric - value that can go up and down.

    Example:
        gauge = Gauge("active_connections", description="Active connections")
        gauge.set(10)
        gauge.increment()
        gauge.decrement()
    """

    def __init__(
        self,
        name: str,
        description: str = "",
        labels: dict[str, str] | None = None,
    ):
        super().__init__(name, description, labels)
        self._value: float = 0

    @property
    def value(self) -> float:
        """Get the current gauge value."""
        return self._value

    def set(self, value: float) -> None:
        """Set the gauge value."""
        self._value = value

    def increment(self, value: float = 1) -> None:
        """Increment the gauge."""
        self._value += value

    def decrement(self, value: float = 1) -> None:
        """Decrement the gauge."""
        self._value -= value

    def reset(self) -> None:
        """Reset the gauge to zero."""
        self._value = 0


class Histogram(Metric):
    """
    Histogram metric - distribution of values.

    Example:
        histogram = Histogram("request_duration_seconds")
        histogram.observe(0.5)
        histogram.observe(1.2)
    """

    def __init__(
        self,
        name: str,
        description: str = "",
        labels: dict[str, str] | None = None,
        buckets: tuple[float, ...] = (0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0),
    ):
        super().__init__(name, description, labels)
        self._buckets = buckets
        self._bucket_counts: list[int] = [0] * len(buckets)
        self._sum: float = 0
        self._count: int = 0

    @property
    def count(self) -> int:
        """Get the number of observations."""
        return self._count

    @property
    def sum(self) -> float:
        """Get the sum of all observations."""
        return self._sum

    @property
    def mean(self) -> float:
        """Get the mean of observations."""
        if self._count == 0:
            return 0
        return self._sum / self._count

    def observe(self, value: float) -> None:
        """Record an observation."""
        self._sum += value
        self._count += 1
        for i, bucket in enumerate(self._buckets):
            if value <= bucket:
                self._bucket_counts[i] += 1

    def reset(self) -> None:
        """Reset the histogram."""
        self._bucket_counts = [0] * len(self._buckets)
        self._sum = 0
        self._count = 0


class Timer(Metric):
    """
    Timer metric - for measuring durations.

    Example:
        timer = Timer("request_duration_seconds")

        with timer.time():
            do_work()

        # Or manually
        start = timer.start()
        do_work()
        timer.stop(start)
    """

    def __init__(
        self,
        name: str,
        description: str = "",
        labels: dict[str, str] | None = None,
    ):
        super().__init__(name, description, labels)
        self._histogram = Histogram(name, description, labels)

    @property
    def count(self) -> int:
        """Get the number of observations."""
        return self._histogram.count

    @property
    def sum(self) -> float:
        """Get the sum of all durations."""
        return self._histogram.sum

    @property
    def mean(self) -> float:
        """Get the mean duration."""
        return self._histogram.mean

    def start(self) -> float:
        """Start the timer and return the start time."""
        return time.perf_counter()

    def stop(self, start_time: float) -> float:
        """Stop the timer and record the duration."""
        duration = time.perf_counter() - start_time
        self._histogram.observe(duration)
        return duration

    def observe(self, duration: float) -> None:
        """Record a duration directly."""
        self._histogram.observe(duration)

    def time(self) -> "TimerContext":
        """Context manager for timing."""
        return TimerContext(self)

    def reset(self) -> None:
        """Reset the timer."""
        self._histogram.reset()


class TimerContext:
    """Context manager for Timer."""

    def __init__(self, timer: Timer):
        self._timer = timer
        self._start: float = 0

    def __enter__(self) -> "TimerContext":
        self._start = self._timer.start()
        return self

    def __exit__(self, *args: Any) -> None:
        self._timer.stop(self._start)


@dataclass
class MetricsRegistry:
    """Registry for all metrics."""
    _counters: dict[str, Counter] = field(default_factory=dict)
    _gauges: dict[str, Gauge] = field(default_factory=dict)
    _histograms: dict[str, Histogram] = field(default_factory=dict)
    _timers: dict[str, Timer] = field(default_factory=dict)

    def counter(self, name: str, description: str = "", labels: dict[str, str] | None = None) -> Counter:
        """Get or create a counter."""
        if name not in self._counters:
            self._counters[name] = Counter(name, description, labels)
        return self._counters[name]

    def gauge(self, name: str, description: str = "", labels: dict[str, str] | None = None) -> Gauge:
        """Get or create a gauge."""
        if name not in self._gauges:
            self._gauges[name] = Gauge(name, description, labels)
        return self._gauges[name]

    def histogram(
        self,
        name: str,
        description: str = "",
        labels: dict[str, str] | None = None,
        buckets: tuple[float, ...] | None = None,
    ) -> Histogram:
        """Get or create a histogram."""
        if name not in self._histograms:
            if buckets:
                self._histograms[name] = Histogram(name, description, labels, buckets)
            else:
                self._histograms[name] = Histogram(name, description, labels)
        return self._histograms[name]

    def timer(self, name: str, description: str = "", labels: dict[str, str] | None = None) -> Timer:
        """Get or create a timer."""
        if name not in self._timers:
            self._timers[name] = Timer(name, description, labels)
        return self._timers[name]

    def reset_all(self) -> None:
        """Reset all metrics."""
        for counter in self._counters.values():
            counter.reset()
        for gauge in self._gauges.values():
            gauge.reset()
        for histogram in self._histograms.values():
            histogram.reset()
        for timer in self._timers.values():
            timer.reset()


# Global metrics registry
_registry: MetricsRegistry | None = None


def configure_metrics() -> MetricsRegistry:
    """Configure and get the global metrics registry."""
    global _registry
    if _registry is None:
        _registry = MetricsRegistry()
    return _registry


def get_metrics() -> MetricsRegistry:
    """Get the global metrics registry."""
    return configure_metrics()


__all__ = [
    "Counter",
    "Gauge",
    "Histogram",
    "Timer",
    "MetricsRegistry",
    "get_metrics",
    "configure_metrics",
]
