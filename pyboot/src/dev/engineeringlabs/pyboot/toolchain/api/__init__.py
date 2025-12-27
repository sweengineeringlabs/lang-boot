"""Toolchain API - Types and enums."""

from enum import Enum, auto


class Environment(Enum):
    """Application environment."""
    DEVELOPMENT = auto()
    STAGING = auto()
    PRODUCTION = auto()
    TEST = auto()


class BuildMode(Enum):
    """Build mode."""
    DEBUG = auto()
    RELEASE = auto()


__all__ = [
    "Environment",
    "BuildMode",
]
