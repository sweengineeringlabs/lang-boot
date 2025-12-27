"""DateTime functions."""

from datetime import datetime, date, time, timedelta, timezone
from typing import Any

from dev.engineeringlabs.pyboot.datetime.api.exceptions import ParseError, TimezoneError


# ============ Current Time ============

def now(tz: str | None = None) -> datetime:
    """Get current datetime.
    
    Args:
        tz: Timezone name (e.g., "America/New_York"). None for local.
        
    Returns:
        Current datetime.
    """
    if tz:
        try:
            from zoneinfo import ZoneInfo
            return datetime.now(ZoneInfo(tz))
        except ImportError:
            try:
                import pytz
                return datetime.now(pytz.timezone(tz))
            except ImportError:
                pass
    return datetime.now()


def today() -> date:
    """Get current date."""
    return date.today()


def utc_now() -> datetime:
    """Get current UTC datetime."""
    return datetime.now(timezone.utc)


# ============ Parsing ============

def parse(value: str, format: str | None = None) -> datetime:
    """Parse datetime string.
    
    Args:
        value: String to parse.
        format: Format string (strptime). Auto-detects if None.
        
    Returns:
        Parsed datetime.
    """
    if format:
        try:
            return datetime.strptime(value, format)
        except ValueError as e:
            raise ParseError(value, format) from e
    
    # Auto-detect common formats
    formats = [
        "%Y-%m-%dT%H:%M:%S.%fZ",
        "%Y-%m-%dT%H:%M:%SZ",
        "%Y-%m-%dT%H:%M:%S.%f",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d",
        "%d/%m/%Y %H:%M:%S",
        "%d/%m/%Y",
        "%m/%d/%Y",
        "%b %d, %Y",
    ]
    
    for fmt in formats:
        try:
            return datetime.strptime(value, fmt)
        except ValueError:
            continue
    
    raise ParseError(value)


def parse_date(value: str, format: str | None = None) -> date:
    """Parse date string."""
    return parse(value, format).date()


def parse_time(value: str, format: str = "%H:%M:%S") -> time:
    """Parse time string."""
    try:
        return datetime.strptime(value, format).time()
    except ValueError as e:
        raise ParseError(value, format) from e


# ============ Formatting ============

def format_dt(dt: datetime, format: str = "%Y-%m-%d %H:%M:%S", relative: bool = False) -> str:
    """Format datetime to string.
    
    Args:
        dt: Datetime to format.
        format: Format string or "iso".
        relative: Use relative format ("2 hours ago").
        
    Returns:
        Formatted string.
    """
    if relative:
        return time_ago(dt)
    
    if format.lower() == "iso":
        return dt.isoformat()
    
    return dt.strftime(format)


def to_iso(dt: datetime) -> str:
    """Format to ISO 8601."""
    return dt.isoformat()


def to_timestamp(dt: datetime) -> float:
    """Convert to Unix timestamp."""
    return dt.timestamp()


def from_timestamp(ts: float, tz: str | None = None) -> datetime:
    """Create datetime from Unix timestamp."""
    if tz:
        try:
            from zoneinfo import ZoneInfo
            return datetime.fromtimestamp(ts, ZoneInfo(tz))
        except ImportError:
            pass
    return datetime.fromtimestamp(ts)


# ============ Manipulation ============

def add_days(dt: datetime, days: int) -> datetime:
    """Add days to datetime."""
    return dt + timedelta(days=days)


def add_hours(dt: datetime, hours: int) -> datetime:
    """Add hours to datetime."""
    return dt + timedelta(hours=hours)


def add_minutes(dt: datetime, minutes: int) -> datetime:
    """Add minutes to datetime."""
    return dt + timedelta(minutes=minutes)


def subtract_days(dt: datetime, days: int) -> datetime:
    """Subtract days from datetime."""
    return dt - timedelta(days=days)


# ============ Range ============

def start_of_day(dt: datetime) -> datetime:
    """Get start of day (midnight)."""
    return dt.replace(hour=0, minute=0, second=0, microsecond=0)


def end_of_day(dt: datetime) -> datetime:
    """Get end of day (23:59:59)."""
    return dt.replace(hour=23, minute=59, second=59, microsecond=999999)


def start_of_week(dt: datetime, week_starts_monday: bool = True) -> datetime:
    """Get start of week."""
    weekday = dt.weekday() if week_starts_monday else (dt.weekday() + 1) % 7
    start = dt - timedelta(days=weekday)
    return start_of_day(start)


def start_of_month(dt: datetime) -> datetime:
    """Get start of month."""
    return dt.replace(day=1, hour=0, minute=0, second=0, microsecond=0)


# ============ Business Days ============

def is_business_day(dt: datetime, holidays: list[date] | None = None) -> bool:
    """Check if date is a business day (Mon-Fri, not holiday)."""
    if dt.weekday() >= 5:  # Saturday=5, Sunday=6
        return False
    if holidays and dt.date() in holidays:
        return False
    return True


def add_business_days(dt: datetime, days: int, holidays: list[date] | None = None) -> datetime:
    """Add business days to datetime."""
    result = dt
    remaining = abs(days)
    direction = 1 if days >= 0 else -1
    
    while remaining > 0:
        result = result + timedelta(days=direction)
        if is_business_day(result, holidays):
            remaining -= 1
    
    return result


# ============ Relative Time ============

def time_ago(dt: datetime) -> str:
    """Format datetime as relative time ("2 hours ago").
    
    Example:
        >>> time_ago(datetime.now() - timedelta(hours=2))
        "2 hours ago"
    """
    now_dt = datetime.now(dt.tzinfo)
    diff = now_dt - dt
    
    seconds = diff.total_seconds()
    
    if seconds < 0:
        return time_until(dt)
    
    if seconds < 60:
        return "just now"
    
    minutes = int(seconds / 60)
    if minutes < 60:
        return f"{minutes} minute{'s' if minutes != 1 else ''} ago"
    
    hours = int(minutes / 60)
    if hours < 24:
        return f"{hours} hour{'s' if hours != 1 else ''} ago"
    
    days = int(hours / 24)
    if days < 30:
        return f"{days} day{'s' if days != 1 else ''} ago"
    
    months = int(days / 30)
    if months < 12:
        return f"{months} month{'s' if months != 1 else ''} ago"
    
    years = int(days / 365)
    return f"{years} year{'s' if years != 1 else ''} ago"


def time_until(dt: datetime) -> str:
    """Format datetime as time until ("in 2 hours")."""
    now_dt = datetime.now(dt.tzinfo)
    diff = dt - now_dt
    
    seconds = diff.total_seconds()
    
    if seconds < 0:
        return time_ago(dt)
    
    if seconds < 60:
        return "in a moment"
    
    minutes = int(seconds / 60)
    if minutes < 60:
        return f"in {minutes} minute{'s' if minutes != 1 else ''}"
    
    hours = int(minutes / 60)
    if hours < 24:
        return f"in {hours} hour{'s' if hours != 1 else ''}"
    
    days = int(hours / 24)
    if days < 30:
        return f"in {days} day{'s' if days != 1 else ''}"
    
    months = int(days / 30)
    if months < 12:
        return f"in {months} month{'s' if months != 1 else ''}"
    
    years = int(days / 365)
    return f"in {years} year{'s' if years != 1 else ''}"
