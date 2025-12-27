"""DateTime types."""

from dataclasses import dataclass
from datetime import datetime, timedelta
from typing import Any


@dataclass
class Duration:
    """Time duration with units.
    
    Example:
        duration = Duration(hours=2, minutes=30)
        future = datetime.now() + duration.to_timedelta()
    """
    days: int = 0
    hours: int = 0
    minutes: int = 0
    seconds: int = 0
    milliseconds: int = 0
    
    def to_timedelta(self) -> timedelta:
        """Convert to timedelta."""
        return timedelta(
            days=self.days,
            hours=self.hours,
            minutes=self.minutes,
            seconds=self.seconds,
            milliseconds=self.milliseconds,
        )
    
    def to_seconds(self) -> float:
        """Convert to total seconds."""
        return self.to_timedelta().total_seconds()
    
    def __add__(self, other: "Duration") -> "Duration":
        return Duration(
            days=self.days + other.days,
            hours=self.hours + other.hours,
            minutes=self.minutes + other.minutes,
            seconds=self.seconds + other.seconds,
            milliseconds=self.milliseconds + other.milliseconds,
        )
    
    @classmethod
    def from_seconds(cls, seconds: float) -> "Duration":
        """Create from total seconds."""
        td = timedelta(seconds=seconds)
        days = td.days
        remaining = td.seconds
        hours, remaining = divmod(remaining, 3600)
        minutes, secs = divmod(remaining, 60)
        return cls(days=days, hours=hours, minutes=minutes, seconds=secs)


@dataclass
class Interval:
    """Time interval between two datetimes.
    
    Example:
        interval = Interval(start=datetime(2024,1,1), end=datetime(2024,1,15))
        print(interval.duration)
    """
    start: datetime
    end: datetime
    
    @property
    def duration(self) -> Duration:
        """Get duration between start and end."""
        return Duration.from_seconds((self.end - self.start).total_seconds())
    
    def contains(self, dt: datetime) -> bool:
        """Check if datetime is within interval."""
        return self.start <= dt <= self.end
    
    def overlaps(self, other: "Interval") -> bool:
        """Check if intervals overlap."""
        return self.start <= other.end and other.start <= self.end


@dataclass
class TimeZone:
    """Timezone representation.
    
    Example:
        tz = TimeZone("America/New_York")
        local_time = tz.localize(utc_time)
    """
    name: str
    
    def get_tz(self) -> Any:
        """Get timezone object."""
        try:
            from zoneinfo import ZoneInfo
            return ZoneInfo(self.name)
        except ImportError:
            try:
                import pytz
                return pytz.timezone(self.name)
            except ImportError:
                raise ImportError("zoneinfo or pytz required")
    
    def localize(self, dt: datetime) -> datetime:
        """Convert datetime to this timezone."""
        tz = self.get_tz()
        if dt.tzinfo is None:
            return dt.replace(tzinfo=tz)
        return dt.astimezone(tz)
