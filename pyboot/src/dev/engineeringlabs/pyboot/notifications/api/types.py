"""
Notification types - Message structures and channels.
"""

from dataclasses import dataclass, field
from enum import Enum
from typing import Any
from datetime import datetime


class NotificationChannel(str, Enum):
    """Notification delivery channels."""
    EMAIL = "email"
    PUSH = "push"
    SMS = "sms"
    WEBHOOK = "webhook"
    IN_APP = "in_app"


class NotificationStatus(str, Enum):
    """Notification delivery status."""
    PENDING = "pending"
    SENT = "sent"
    DELIVERED = "delivered"
    FAILED = "failed"
    BOUNCED = "bounced"


@dataclass
class Notification:
    """Base notification class."""
    channel: NotificationChannel
    recipient: str
    metadata: dict[str, Any] = field(default_factory=dict)
    priority: int = 0  # Higher = more urgent
    scheduled_at: datetime | None = None
    created_at: datetime = field(default_factory=datetime.now)


@dataclass
class Email(Notification):
    """Email notification.
    
    Example:
        email = Email(
            to="user@example.com",
            subject="Welcome!",
            body="Thanks for signing up.",
            html="<h1>Welcome!</h1>",
        )
    """
    to: str = ""
    subject: str = ""
    body: str = ""
    html: str | None = None
    from_email: str | None = None
    from_name: str | None = None
    reply_to: str | None = None
    cc: list[str] = field(default_factory=list)
    bcc: list[str] = field(default_factory=list)
    attachments: list[dict[str, Any]] = field(default_factory=list)
    headers: dict[str, str] = field(default_factory=dict)
    
    def __post_init__(self) -> None:
        self.channel = NotificationChannel.EMAIL
        self.recipient = self.to


@dataclass
class Push(Notification):
    """Push notification (FCM, APNs).
    
    Example:
        push = Push(
            token="device_token",
            title="New Message",
            body="You have a new message.",
            data={"message_id": "123"},
        )
    """
    token: str = ""
    title: str = ""
    body: str = ""
    image: str | None = None
    icon: str | None = None
    badge: int | None = None
    sound: str | None = None
    data: dict[str, Any] = field(default_factory=dict)
    topic: str | None = None  # For topic-based push
    condition: str | None = None  # FCM conditions
    
    def __post_init__(self) -> None:
        self.channel = NotificationChannel.PUSH
        self.recipient = self.token


@dataclass  
class SMS(Notification):
    """SMS notification.
    
    Example:
        sms = SMS(
            to="+1234567890",
            message="Your code is 123456",
        )
    """
    to: str = ""
    message: str = ""
    from_number: str | None = None
    
    def __post_init__(self) -> None:
        self.channel = NotificationChannel.SMS
        self.recipient = self.to


@dataclass
class Webhook(Notification):
    """Webhook notification.
    
    Example:
        webhook = Webhook(
            url="https://api.example.com/webhook",
            payload={"event": "user.created", "user_id": "123"},
        )
    """
    url: str = ""
    payload: dict[str, Any] = field(default_factory=dict)
    method: str = "POST"
    headers: dict[str, str] = field(default_factory=dict)
    timeout: float = 30.0
    retry_count: int = 3
    
    def __post_init__(self) -> None:
        self.channel = NotificationChannel.WEBHOOK
        self.recipient = self.url


@dataclass
class InApp(Notification):
    """In-app notification.
    
    Example:
        notification = InApp(
            user_id="user_123",
            title="New Feature",
            message="Check out our new feature!",
            action_url="/features/new",
        )
    """
    user_id: str = ""
    title: str = ""
    message: str = ""
    icon: str | None = None
    action_url: str | None = None
    action_label: str | None = None
    persistent: bool = False
    expires_at: datetime | None = None
    
    def __post_init__(self) -> None:
        self.channel = NotificationChannel.IN_APP
        self.recipient = self.user_id


@dataclass
class NotificationResult:
    """Result of notification delivery."""
    notification: Notification
    status: NotificationStatus
    message_id: str | None = None
    error: str | None = None
    provider: str | None = None
    sent_at: datetime = field(default_factory=datetime.now)
    
    @property
    def success(self) -> bool:
        return self.status in (NotificationStatus.SENT, NotificationStatus.DELIVERED)


@dataclass
class Template:
    """Notification template.
    
    Example:
        template = Template(
            name="welcome_email",
            channel=NotificationChannel.EMAIL,
            subject="Welcome, {{ name }}!",
            body="Hello {{ name }}, thanks for joining.",
            html="<h1>Welcome, {{ name }}!</h1>",
        )
    """
    name: str
    channel: NotificationChannel
    subject: str = ""
    body: str = ""
    html: str | None = None
    metadata: dict[str, Any] = field(default_factory=dict)
