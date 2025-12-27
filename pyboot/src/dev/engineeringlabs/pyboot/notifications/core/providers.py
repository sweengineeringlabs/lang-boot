"""
Notification providers - Built-in provider implementations.
"""

import asyncio
import uuid
from dataclasses import dataclass
from typing import Any

from dev.engineeringlabs.pyboot.notifications.api.types import (
    Notification,
    Email,
    Push,
    SMS,
    Webhook,
    NotificationChannel,
    NotificationStatus,
    NotificationResult,
)
from dev.engineeringlabs.pyboot.notifications.api.exceptions import DeliveryError


@dataclass
class SMTPConfig:
    """SMTP configuration."""
    host: str = "localhost"
    port: int = 587
    username: str | None = None
    password: str | None = None
    use_tls: bool = True
    use_ssl: bool = False
    timeout: float = 30.0


class SMTPEmailProvider:
    """SMTP email provider.
    
    Example:
        provider = SMTPEmailProvider(SMTPConfig(
            host="smtp.gmail.com",
            port=587,
            username="user@gmail.com",
            password="app_password",
        ))
        result = await provider.send(email)
    """
    
    def __init__(self, config: SMTPConfig) -> None:
        self._config = config
    
    async def send(self, notification: Notification) -> NotificationResult:
        if not isinstance(notification, Email):
            return NotificationResult(
                notification=notification,
                status=NotificationStatus.FAILED,
                error="SMTPEmailProvider only supports Email notifications",
            )
        return await self.send_email(notification)
    
    async def send_email(self, email: Email) -> NotificationResult:
        """Send email via SMTP."""
        try:
            import smtplib
            from email.mime.text import MIMEText
            from email.mime.multipart import MIMEMultipart
            
            # Build message
            msg = MIMEMultipart("alternative")
            msg["Subject"] = email.subject
            msg["From"] = f"{email.from_name} <{email.from_email}>" if email.from_name else email.from_email or ""
            msg["To"] = email.to
            
            if email.cc:
                msg["Cc"] = ", ".join(email.cc)
            if email.reply_to:
                msg["Reply-To"] = email.reply_to
            
            # Add body
            msg.attach(MIMEText(email.body, "plain"))
            if email.html:
                msg.attach(MIMEText(email.html, "html"))
            
            # Send in executor to avoid blocking
            def _send() -> str:
                if self._config.use_ssl:
                    server = smtplib.SMTP_SSL(self._config.host, self._config.port, timeout=self._config.timeout)
                else:
                    server = smtplib.SMTP(self._config.host, self._config.port, timeout=self._config.timeout)
                
                try:
                    if self._config.use_tls and not self._config.use_ssl:
                        server.starttls()
                    if self._config.username and self._config.password:
                        server.login(self._config.username, self._config.password)
                    
                    recipients = [email.to] + email.cc + email.bcc
                    server.sendmail(email.from_email or "", recipients, msg.as_string())
                    return str(uuid.uuid4())
                finally:
                    server.quit()
            
            loop = asyncio.get_event_loop()
            message_id = await loop.run_in_executor(None, _send)
            
            return NotificationResult(
                notification=email,
                status=NotificationStatus.SENT,
                message_id=message_id,
                provider="smtp",
            )
            
        except Exception as e:
            return NotificationResult(
                notification=email,
                status=NotificationStatus.FAILED,
                error=str(e),
                provider="smtp",
            )
    
    async def send_bulk(self, emails: list[Email]) -> list[NotificationResult]:
        return [await self.send_email(email) for email in emails]
    
    def supports(self, notification: Notification) -> bool:
        return isinstance(notification, Email)


class ConsoleEmailProvider:
    """Console email provider for development/testing."""
    
    async def send(self, notification: Notification) -> NotificationResult:
        if not isinstance(notification, Email):
            return NotificationResult(
                notification=notification,
                status=NotificationStatus.FAILED,
                error="ConsoleEmailProvider only supports Email",
            )
        return await self.send_email(notification)
    
    async def send_email(self, email: Email) -> NotificationResult:
        message_id = str(uuid.uuid4())
        print(f"\n{'='*50}")
        print(f"📧 EMAIL NOTIFICATION")
        print(f"{'='*50}")
        print(f"To: {email.to}")
        print(f"Subject: {email.subject}")
        print(f"Body: {email.body[:200]}...")
        print(f"Message ID: {message_id}")
        print(f"{'='*50}\n")
        
        return NotificationResult(
            notification=email,
            status=NotificationStatus.SENT,
            message_id=message_id,
            provider="console",
        )
    
    async def send_bulk(self, emails: list[Email]) -> list[NotificationResult]:
        return [await self.send_email(email) for email in emails]
    
    def supports(self, notification: Notification) -> bool:
        return isinstance(notification, Email)


class ConsolePushProvider:
    """Console push provider for development/testing."""
    
    async def send(self, notification: Notification) -> NotificationResult:
        if not isinstance(notification, Push):
            return NotificationResult(
                notification=notification,
                status=NotificationStatus.FAILED,
                error="ConsolePushProvider only supports Push",
            )
        return await self.send_push(notification)
    
    async def send_push(self, push: Push) -> NotificationResult:
        message_id = str(uuid.uuid4())
        print(f"\n{'='*50}")
        print(f"🔔 PUSH NOTIFICATION")
        print(f"{'='*50}")
        print(f"Token: {push.token[:20]}...")
        print(f"Title: {push.title}")
        print(f"Body: {push.body}")
        print(f"Data: {push.data}")
        print(f"Message ID: {message_id}")
        print(f"{'='*50}\n")
        
        return NotificationResult(
            notification=push,
            status=NotificationStatus.SENT,
            message_id=message_id,
            provider="console",
        )
    
    async def send_to_topic(self, topic: str, push: Push) -> NotificationResult:
        push.topic = topic
        return await self.send_push(push)
    
    async def send_multicast(self, tokens: list[str], push: Push) -> list[NotificationResult]:
        results = []
        for token in tokens:
            push.token = token
            results.append(await self.send_push(push))
        return results
    
    def supports(self, notification: Notification) -> bool:
        return isinstance(notification, Push)


class ConsoleSMSProvider:
    """Console SMS provider for development/testing."""
    
    async def send(self, notification: Notification) -> NotificationResult:
        if not isinstance(notification, SMS):
            return NotificationResult(
                notification=notification,
                status=NotificationStatus.FAILED,
                error="ConsoleSMSProvider only supports SMS",
            )
        return await self.send_sms(notification)
    
    async def send_sms(self, sms: SMS) -> NotificationResult:
        message_id = str(uuid.uuid4())
        print(f"\n{'='*50}")
        print(f"📱 SMS NOTIFICATION")
        print(f"{'='*50}")
        print(f"To: {sms.to}")
        print(f"Message: {sms.message}")
        print(f"Message ID: {message_id}")
        print(f"{'='*50}\n")
        
        return NotificationResult(
            notification=sms,
            status=NotificationStatus.SENT,
            message_id=message_id,
            provider="console",
        )
    
    def supports(self, notification: Notification) -> bool:
        return isinstance(notification, SMS)


class WebhookProvider:
    """HTTP webhook provider."""
    
    async def send(self, notification: Notification) -> NotificationResult:
        if not isinstance(notification, Webhook):
            return NotificationResult(
                notification=notification,
                status=NotificationStatus.FAILED,
                error="WebhookProvider only supports Webhook",
            )
        
        try:
            import httpx
            
            async with httpx.AsyncClient(timeout=notification.timeout) as client:
                response = await client.request(
                    method=notification.method,
                    url=notification.url,
                    json=notification.payload,
                    headers=notification.headers,
                )
                response.raise_for_status()
                
                return NotificationResult(
                    notification=notification,
                    status=NotificationStatus.SENT,
                    message_id=str(uuid.uuid4()),
                    provider="webhook",
                )
                
        except ImportError:
            # Fallback without httpx
            return NotificationResult(
                notification=notification,
                status=NotificationStatus.FAILED,
                error="httpx not installed. Install with: pip install httpx",
                provider="webhook",
            )
        except Exception as e:
            return NotificationResult(
                notification=notification,
                status=NotificationStatus.FAILED,
                error=str(e),
                provider="webhook",
            )
    
    def supports(self, notification: Notification) -> bool:
        return isinstance(notification, Webhook)
