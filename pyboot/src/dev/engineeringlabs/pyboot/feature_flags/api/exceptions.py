"""Feature flags exceptions - Standalone error types."""


class FeatureFlagError(Exception):
    """Base exception for feature flag errors."""
    
    def __init__(self, message: str, *, cause: Exception | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.cause = cause


class FlagNotFoundError(FeatureFlagError):
    """Exception when flag is not found."""
    
    def __init__(self, flag_name: str) -> None:
        super().__init__(f"Feature flag not found: {flag_name}")
        self.flag_name = flag_name


class FlagConfigurationError(FeatureFlagError):
    """Exception when flag configuration is invalid."""
    
    def __init__(self, flag_name: str, reason: str) -> None:
        super().__init__(f"Invalid configuration for flag '{flag_name}': {reason}")
        self.flag_name = flag_name
        self.reason = reason


__all__ = ["FeatureFlagError", "FlagNotFoundError", "FlagConfigurationError"]
