"""FileIO API - File error types."""


class FileError(Exception):
    """Base error for file operations."""
    
    def __init__(self, message: str, path: str | None = None, *, cause: Exception | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.path = path
        self.cause = cause


class FileNotFoundError(FileError):
    """File not found error."""
    
    def __init__(self, path: str) -> None:
        super().__init__(f"File not found: {path}", path=path)


class PermissionError(FileError):
    """Permission denied error."""
    
    def __init__(self, path: str, operation: str = "access") -> None:
        super().__init__(f"Permission denied to {operation}: {path}", path=path)
        self.operation = operation


__all__ = [
    "FileError",
    "FileNotFoundError",
    "PermissionError",
]
