"""Debug API - Debug types and configuration."""

from enum import Enum, auto
from dataclasses import dataclass


class DebugLevel(Enum):
    """Debug logging level."""
    TRACE = auto()
    DEBUG = auto()
    INFO = auto()
    WARN = auto()
    ERROR = auto()


@dataclass
class DebugConfig:
    """Debug configuration."""
    level: DebugLevel = DebugLevel.DEBUG
    enabled: bool = True
    show_timestamps: bool = True
    show_source: bool = True


__all__ = [
    "DebugLevel",
    "DebugConfig",
]
