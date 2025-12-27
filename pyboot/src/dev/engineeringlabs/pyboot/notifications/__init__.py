"""
Notifications Module - Multi-channel notification delivery.

This module provides:
- Email notifications (SMTP, SendGrid, SES)
- Push notifications (FCM, APNs)
- SMS notifications (Twilio, SNS)
- Webhook notifications
- In-app notifications
- Template-based messaging

Example:
    from dev.engineeringlabs.pyboot.notifications import notify, Email, Push, SMS
    from dev.engineeringlabs.pyboot.notifications import NotificationService, Template
    
    # Simple email
    await notify(Email(
        to="user@example.com",
        subject="Welcome!",
        body="Thanks for signing up."
    ))
    
    # Push notification
    await notify(Push(
        token="device_token",
        title="New Message",
        body="You have a new message.",
    ))
    
    # Using templates
    service = NotificationService()
    await service.send_template(
        "welcome_email",
        to="user@example.com",
        context={"name": "Alice"},
    )
"""

from dev.engineeringlabs.pyboot.notifications.api import (
    # Types
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
    # Protocols
    NotificationProvider,
    EmailProvider,
    PushProvider,
    SMSProvider,
    # Exceptions
    NotificationError,
    DeliveryError,
    TemplateError,
)

from dev.engineeringlabs.pyboot.notifications.core import (
    # Main interface
    NotificationService,
    notify,
    # Providers
    SMTPEmailProvider,
    ConsoleEmailProvider,
    ConsolePushProvider,
    ConsoleSMSProvider,
    WebhookProvider,
    # Templates
    TemplateEngine,
    # Utilities
    render_template,
)

__all__ = [
    # API - Types
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
    # API - Protocols
    "NotificationProvider",
    "EmailProvider",
    "PushProvider",
    "SMSProvider",
    # API - Exceptions
    "NotificationError",
    "DeliveryError",
    "TemplateError",
    # Core
    "NotificationService",
    "notify",
    # Core - Providers
    "SMTPEmailProvider",
    "ConsoleEmailProvider",
    "ConsolePushProvider",
    "ConsoleSMSProvider",
    "WebhookProvider",
    # Core - Templates
    "TemplateEngine",
    "render_template",
]
