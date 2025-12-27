"""Notification exceptions - Standalone error types."""


class NotificationError(Exception):
    """Base exception for notification errors."""
    
    def __init__(self, message: str, *, cause: Exception | None = None) -> None:
        super().__init__(message)
        self.message = message
        self.cause = cause


class DeliveryError(NotificationError):
    """Exception when notification delivery fails."""
    
    def __init__(self, channel: str, recipient: str, *, cause: Exception | None = None) -> None:
        super().__init__(f"Failed to deliver to '{recipient}' via {channel}", cause=cause)
        self.channel = channel
        self.recipient = recipient


class TemplateError(NotificationError):
    """Exception when template rendering fails."""
    
    def __init__(self, template_name: str, *, cause: Exception | None = None) -> None:
        super().__init__(f"Failed to render template: {template_name}", cause=cause)
        self.template_name = template_name


__all__ = ["NotificationError", "DeliveryError", "TemplateError"]
