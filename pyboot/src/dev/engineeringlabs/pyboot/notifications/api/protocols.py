"""
Notification protocols - Provider interfaces.
"""

from typing import Protocol, runtime_checkable
from dev.engineeringlabs.pyboot.notifications.api.types import (
    Notification,
    NotificationResult,
    Email,
    Push,
    SMS,
)


@runtime_checkable
class NotificationProvider(Protocol):
    """Base protocol for notification providers."""
    
    async def send(self, notification: Notification) -> NotificationResult:
        """Send a notification."""
        ...
    
    def supports(self, notification: Notification) -> bool:
        """Check if provider supports this notification type."""
        ...


@runtime_checkable
class EmailProvider(Protocol):
    """Protocol for email providers (SMTP, SendGrid, SES)."""
    
    async def send_email(self, email: Email) -> NotificationResult:
        """Send an email."""
        ...
    
    async def send_bulk(self, emails: list[Email]) -> list[NotificationResult]:
        """Send multiple emails."""
        ...


@runtime_checkable
class PushProvider(Protocol):
    """Protocol for push notification providers (FCM, APNs)."""
    
    async def send_push(self, push: Push) -> NotificationResult:
        """Send a push notification."""
        ...
    
    async def send_to_topic(self, topic: str, push: Push) -> NotificationResult:
        """Send to a topic."""
        ...
    
    async def send_multicast(self, tokens: list[str], push: Push) -> list[NotificationResult]:
        """Send to multiple tokens."""
        ...


@runtime_checkable  
class SMSProvider(Protocol):
    """Protocol for SMS providers (Twilio, SNS)."""
    
    async def send_sms(self, sms: SMS) -> NotificationResult:
        """Send an SMS."""
        ...
