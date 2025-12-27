"""
DateTime Module - Enhanced datetime utilities.

This module provides:
- Timezone-aware datetime handling
- Date/time parsing and formatting
- Duration and interval operations
- Business day calculations
- Relative time (humanized)

Example:
    from dev.engineeringlabs.pyboot.datetime import now, parse, format_dt, Duration
    from dev.engineeringlabs.pyboot.datetime import add_days, start_of_day, is_business_day
    
    # Current time
    current = now()  # UTC by default
    local = now("America/New_York")
    
    # Parsing
    dt = parse("2024-01-15T10:30:00Z")
    dt = parse("Jan 15, 2024", format="%b %d, %Y")
    
    # Formatting
    formatted = format_dt(dt, "YYYY-MM-DD")
    relative = format_dt(dt, relative=True)  # "2 hours ago"
    
    # Duration
    duration = Duration(hours=2, minutes=30)
    future = current + duration
    
    # Business days
    next_business = add_business_days(current, 5)
"""

from dev.engineeringlabs.pyboot.datetime.api import (
    # Types
    Duration,
    Interval,
    TimeZone,
    # Exceptions
    DateTimeError,
    ParseError,
    TimezoneError,
)

from dev.engineeringlabs.pyboot.datetime.core import (
    # Current time
    now,
    today,
    utc_now,
    # Parsing
    parse,
    parse_date,
    parse_time,
    # Formatting
    format_dt,
    to_iso,
    to_timestamp,
    from_timestamp,
    # Manipulation
    add_days,
    add_hours,
    add_minutes,
    subtract_days,
    # Range
    start_of_day,
    end_of_day,
    start_of_week,
    start_of_month,
    # Business
    is_business_day,
    add_business_days,
    # Relative
    time_ago,
    time_until,
)

__all__ = [
    # API
    "Duration",
    "Interval",
    "TimeZone",
    "DateTimeError",
    "ParseError",
    "TimezoneError",
    # Core - Current
    "now",
    "today",
    "utc_now",
    # Core - Parsing
    "parse",
    "parse_date",
    "parse_time",
    # Core - Formatting
    "format_dt",
    "to_iso",
    "to_timestamp",
    "from_timestamp",
    # Core - Manipulation
    "add_days",
    "add_hours",
    "add_minutes",
    "subtract_days",
    # Core - Range
    "start_of_day",
    "end_of_day",
    "start_of_week",
    "start_of_month",
    # Core - Business
    "is_business_day",
    "add_business_days",
    # Core - Relative
    "time_ago",
    "time_until",
]
