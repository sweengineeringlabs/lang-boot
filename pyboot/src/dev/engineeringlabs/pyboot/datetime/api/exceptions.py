"""Datetime exceptions - Standalone error types."""


class DateTimeError(Exception):
    """Base exception for datetime errors."""
    
    def __init__(self, message: str, *, cause: Exception | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.cause = cause


class ParseError(DateTimeError):
    """Exception when datetime parsing fails."""
    
    def __init__(self, value: str, format: str | None = None, *, cause: Exception | None = None) -> None:
        msg = f"Failed to parse datetime: '{value}'"
        if format:
            msg += f" with format '{format}'"
        super().__init__(msg, cause=cause)
        self.value = value
        self.format = format


class TimezoneError(DateTimeError):
    """Exception for timezone-related errors."""
    
    def __init__(self, timezone: str, *, cause: Exception | None = None) -> None:
        super().__init__(f"Invalid or unknown timezone: {timezone}", cause=cause)
        self.timezone = timezone


__all__ = ["DateTimeError", "ParseError", "TimezoneError"]
