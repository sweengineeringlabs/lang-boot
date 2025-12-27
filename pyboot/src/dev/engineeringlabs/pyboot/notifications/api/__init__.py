"""
Notifications API - Public interfaces and types.
"""

from dev.engineeringlabs.pyboot.notifications.api.types import (
    Notification,
    Email,
    Push,
    SMS,
    Webhook,
    InApp,
    NotificationChannel,
    NotificationStatus,
    NotificationResult,
    Template,
)

from dev.engineeringlabs.pyboot.notifications.api.protocols import (
    NotificationProvider,
    EmailProvider,
    PushProvider,
    SMSProvider,
)

from dev.engineeringlabs.pyboot.notifications.api.exceptions import (
    NotificationError,
    DeliveryError,
    TemplateError,
)

__all__ = [
    # Types
    "Notification",
    "Email",
    "Push",
    "SMS",
    "Webhook",
    "InApp",
    "NotificationChannel",
    "NotificationStatus",
    "NotificationResult",
    "Template",
    # Protocols
    "NotificationProvider",
    "EmailProvider",
    "PushProvider",
    "SMSProvider",
    # Exceptions
    "NotificationError",
    "DeliveryError",
    "TemplateError",
]
