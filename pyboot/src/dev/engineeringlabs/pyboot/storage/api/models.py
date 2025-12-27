"""Storage models."""

from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True, slots=True)
class FileInfo:
    """Information about a file."""

    path: str
    name: str
    size: int
    modified_at: float | None = None
    created_at: float | None = None
    content_type: str | None = None
    metadata: dict[str, Any] | None = None

    @property
    def extension(self) -> str:
        """Get file extension."""
        if "." in self.name:
            return self.name.rsplit(".", 1)[-1].lower()
        return ""

    @property
    def is_text(self) -> bool:
        """Check if file is likely text."""
        text_extensions = {"txt", "md", "json", "yaml", "yml", "xml", "html", "css", "js", "py"}
        return self.extension in text_extensions

    def to_dict(self) -> dict[str, Any]:
        """Convert to dictionary."""
        result: dict[str, Any] = {
            "path": self.path,
            "name": self.name,
            "size": self.size,
        }
        if self.modified_at is not None:
            result["modified_at"] = self.modified_at
        if self.created_at is not None:
            result["created_at"] = self.created_at
        if self.content_type:
            result["content_type"] = self.content_type
        if self.metadata:
            result["metadata"] = self.metadata
        return result


__all__ = ["FileInfo"]
