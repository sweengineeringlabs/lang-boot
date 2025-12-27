"""CLI API - Command types and errors."""

from typing import Any, Callable
from dataclasses import dataclass, field
from enum import Enum


class CLIError(Exception):
    """Base error for CLI operations."""
    
    def __init__(self, message: str, *, cause: Exception | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.cause = cause


@dataclass
class CLIResult:
    """Result of a CLI command."""
    success: bool
    message: str = ""
    exit_code: int = 0
    data: Any = None


@dataclass
class Argument:
    """CLI argument definition."""
    name: str
    description: str = ""
    required: bool = True
    default: Any = None


@dataclass
class Option:
    """CLI option definition."""
    name: str
    short: str | None = None
    description: str = ""
    required: bool = False
    default: Any = None


@dataclass
class Command:
    """CLI command definition."""
    name: str
    description: str = ""
    arguments: list[Argument] = field(default_factory=list)
    options: list[Option] = field(default_factory=list)
    handler: Callable[..., CLIResult] | None = None


__all__ = [
    "CLIError",
    "CLIResult",
    "Argument",
    "Option",
    "Command",
]
