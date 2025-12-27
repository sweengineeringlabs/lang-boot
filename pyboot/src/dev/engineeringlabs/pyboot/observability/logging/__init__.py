"""Structured logging module."""

import json
import sys
import time
from dataclasses import dataclass, field
from enum import IntEnum
from typing import Any, TextIO


class LogLevel(IntEnum):
    """Log level enumeration."""
    TRACE = 5
    DEBUG = 10
    INFO = 20
    WARN = 30
    ERROR = 40
    FATAL = 50


@dataclass(slots=True)
class LogRecord:
    """A log record with structured data."""
    timestamp: float
    level: LogLevel
    message: str
    logger_name: str
    context: dict[str, Any] = field(default_factory=dict)
    exception: BaseException | None = None

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        result = {
            "timestamp": self.timestamp,
            "level": self.level.name,
            "message": self.message,
            "logger": self.logger_name,
        }
        if self.context:
            result["context"] = self.context
        if self.exception:
            result["exception"] = {
                "type": type(self.exception).__name__,
                "message": str(self.exception),
            }
        return result


class Logger:
    """
    Structured logger with context propagation.

    Example:
        logger = Logger("my-service")
        logger.info("Processing request", request_id="abc123")

        # Create child logger with context
        request_logger = logger.with_context(user_id="user42")
        request_logger.info("User action")  # Includes user_id
    """

    def __init__(
        self,
        name: str,
        level: LogLevel = LogLevel.INFO,
        context: dict[str, Any] | None = None,
        output: TextIO | None = None,
        json_output: bool = False,
    ) -> None:
        self._name = name
        self._level = level
        self._context = context or {}
        self._output = output or sys.stderr
        self._json_output = json_output

    @property
    def name(self) -> str:
        """Get the logger name."""
        return self._name

    @property
    def level(self) -> LogLevel:
        """Get the current log level."""
        return self._level

    def set_level(self, level: LogLevel) -> None:
        """Set the log level."""
        self._level = level

    def log(self, level: LogLevel, message: str, **context: Any) -> None:
        """Log a message at the specified level."""
        if level < self._level:
            return

        record = LogRecord(
            timestamp=time.time(),
            level=level,
            message=message,
            logger_name=self._name,
            context={**self._context, **context},
        )
        self._write(record)

    def trace(self, message: str, **context: Any) -> None:
        """Log a trace message."""
        self.log(LogLevel.TRACE, message, **context)

    def debug(self, message: str, **context: Any) -> None:
        """Log a debug message."""
        self.log(LogLevel.DEBUG, message, **context)

    def info(self, message: str, **context: Any) -> None:
        """Log an info message."""
        self.log(LogLevel.INFO, message, **context)

    def warn(self, message: str, **context: Any) -> None:
        """Log a warning message."""
        self.log(LogLevel.WARN, message, **context)

    def warning(self, message: str, **context: Any) -> None:
        """Log a warning message (alias for warn)."""
        self.warn(message, **context)

    def error(
        self,
        message: str,
        exception: BaseException | None = None,
        **context: Any,
    ) -> None:
        """Log an error message."""
        if exception:
            context["exception_type"] = type(exception).__name__
            context["exception_message"] = str(exception)
        self.log(LogLevel.ERROR, message, **context)

    def fatal(
        self,
        message: str,
        exception: BaseException | None = None,
        **context: Any,
    ) -> None:
        """Log a fatal message."""
        if exception:
            context["exception_type"] = type(exception).__name__
            context["exception_message"] = str(exception)
        self.log(LogLevel.FATAL, message, **context)

    def with_context(self, **context: Any) -> "Logger":
        """Create a child logger with additional context."""
        return Logger(
            name=self._name,
            level=self._level,
            context={**self._context, **context},
            output=self._output,
            json_output=self._json_output,
        )

    def _write(self, record: LogRecord) -> None:
        """Write a log record to output."""
        if self._json_output:
            output = json.dumps(record.to_dict())
        else:
            # Human-readable format
            timestamp = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime(record.timestamp))
            level = record.level.name.ljust(5)
            context_str = ""
            if record.context:
                context_str = " " + " ".join(f"{k}={v}" for k, v in record.context.items())
            output = f"[{timestamp}] {level} {record.logger_name}: {record.message}{context_str}"

        print(output, file=self._output)


# Global loggers registry
_loggers: dict[str, Logger] = {}
_default_level: LogLevel = LogLevel.INFO
_json_output: bool = False


def configure_logging(
    level: LogLevel = LogLevel.INFO,
    json_output: bool = False,
) -> None:
    """Configure global logging settings."""
    global _default_level, _json_output
    _default_level = level
    _json_output = json_output
    # Update existing loggers
    for logger in _loggers.values():
        logger.set_level(level)


def get_logger(name: str) -> Logger:
    """Get or create a logger by name."""
    if name not in _loggers:
        _loggers[name] = Logger(
            name=name,
            level=_default_level,
            json_output=_json_output,
        )
    return _loggers[name]


__all__ = [
    "Logger",
    "LogLevel",
    "LogRecord",
    "get_logger",
    "configure_logging",
]
