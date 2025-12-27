"""
Notification service - Central notification manager.
"""

from typing import Any
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
from dev.engineeringlabs.pyboot.notifications.api.protocols import NotificationProvider
from dev.engineeringlabs.pyboot.notifications.api.exceptions import NotificationError, DeliveryError


class NotificationService:
    """Central notification service.
    
    Manages providers for different channels and routes
    notifications to the appropriate provider.
    
    Example:
        service = NotificationService()
        service.register_provider(NotificationChannel.EMAIL, smtp_provider)
        service.register_provider(NotificationChannel.PUSH, fcm_provider)
        
        # Send notification
        result = await service.send(Email(
            to="user@example.com",
            subject="Hello",
            body="World",
        ))
        
        # Send using template
        result = await service.send_template(
            "welcome_email",
            to="user@example.com",
            context={"name": "Alice"},
        )
    """
    
    def __init__(self) -> None:
        self._providers: dict[NotificationChannel, NotificationProvider] = {}
        self._templates: dict[str, Template] = {}
        self._default_from_email: str | None = None
        self._default_from_name: str | None = None
    
    def register_provider(
        self,
        channel: NotificationChannel,
        provider: NotificationProvider,
    ) -> None:
        """Register a provider for a channel."""
        self._providers[channel] = provider
    
    def register_template(self, template: Template) -> None:
        """Register a notification template."""
        self._templates[template.name] = template
    
    def set_defaults(
        self,
        from_email: str | None = None,
        from_name: str | None = None,
    ) -> None:
        """Set default sender information."""
        self._default_from_email = from_email
        self._default_from_name = from_name
    
    async def send(self, notification: Notification) -> NotificationResult:
        """Send a notification through the appropriate provider."""
        provider = self._providers.get(notification.channel)
        
        if provider is None:
            return NotificationResult(
                notification=notification,
                status=NotificationStatus.FAILED,
                error=f"No provider registered for channel: {notification.channel}",
            )
        
        # Apply defaults for email
        if isinstance(notification, Email):
            if not notification.from_email and self._default_from_email:
                notification.from_email = self._default_from_email
            if not notification.from_name and self._default_from_name:
                notification.from_name = self._default_from_name
        
        try:
            return await provider.send(notification)
        except Exception as e:
            return NotificationResult(
                notification=notification,
                status=NotificationStatus.FAILED,
                error=str(e),
            )
    
    async def send_email(
        self,
        to: str,
        subject: str,
        body: str,
        *,
        html: str | None = None,
        **kwargs: Any,
    ) -> NotificationResult:
        """Convenience method for sending emails."""
        email = Email(to=to, subject=subject, body=body, html=html, **kwargs)
        return await self.send(email)
    
    async def send_push(
        self,
        token: str,
        title: str,
        body: str,
        *,
        data: dict[str, Any] | None = None,
        **kwargs: Any,
    ) -> NotificationResult:
        """Convenience method for sending push notifications."""
        push = Push(token=token, title=title, body=body, data=data or {}, **kwargs)
        return await self.send(push)
    
    async def send_sms(
        self,
        to: str,
        message: str,
        **kwargs: Any,
    ) -> NotificationResult:
        """Convenience method for sending SMS."""
        sms = SMS(to=to, message=message, **kwargs)
        return await self.send(sms)
    
    async def send_template(
        self,
        template_name: str,
        *,
        to: str,
        context: dict[str, Any] | None = None,
        **kwargs: Any,
    ) -> NotificationResult:
        """Send notification using a template."""
        from dev.engineeringlabs.pyboot.notifications.core.templates import render_template
        
        template = self._templates.get(template_name)
        if not template:
            return NotificationResult(
                notification=Email(to=to, subject="", body=""),
                status=NotificationStatus.FAILED,
                error=f"Template not found: {template_name}",
            )
        
        ctx = context or {}
        
        if template.channel == NotificationChannel.EMAIL:
            email = Email(
                to=to,
                subject=render_template(template.subject, ctx),
                body=render_template(template.body, ctx),
                html=render_template(template.html, ctx) if template.html else None,
                **kwargs,
            )
            return await self.send(email)
        
        # Add support for other channels as needed
        return NotificationResult(
            notification=Email(to=to, subject="", body=""),
            status=NotificationStatus.FAILED,
            error=f"Template channel not supported: {template.channel}",
        )
    
    def get_template(self, name: str) -> Template | None:
        """Get a registered template."""
        return self._templates.get(name)


# Global service instance
_global_service = NotificationService()


async def notify(notification: Notification) -> NotificationResult:
    """Send a notification using the global service.
    
    Example:
        result = await notify(Email(
            to="user@example.com",
            subject="Hello",
            body="World",
        ))
    """
    return await _global_service.send(notification)


def get_notification_service() -> NotificationService:
    """Get the global notification service."""
    return _global_service


def configure_notifications(
    *,
    email_provider: NotificationProvider | None = None,
    push_provider: NotificationProvider | None = None,
    sms_provider: NotificationProvider | None = None,
    from_email: str | None = None,
    from_name: str | None = None,
) -> None:
    """Configure the global notification service."""
    if email_provider:
        _global_service.register_provider(NotificationChannel.EMAIL, email_provider)
    if push_provider:
        _global_service.register_provider(NotificationChannel.PUSH, push_provider)
    if sms_provider:
        _global_service.register_provider(NotificationChannel.SMS, sms_provider)
    _global_service.set_defaults(from_email=from_email, from_name=from_name)
