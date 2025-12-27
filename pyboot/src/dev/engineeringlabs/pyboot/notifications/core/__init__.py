"""
Notifications Core - Implementations.
"""

from dev.engineeringlabs.pyboot.notifications.core.service import (
    NotificationService,
    notify,
)

from dev.engineeringlabs.pyboot.notifications.core.providers import (
    SMTPEmailProvider,
    ConsoleEmailProvider,
    ConsolePushProvider,
    ConsoleSMSProvider,
    WebhookProvider,
)

from dev.engineeringlabs.pyboot.notifications.core.templates import (
    TemplateEngine,
    render_template,
)

__all__ = [
    "NotificationService",
    "notify",
    "SMTPEmailProvider",
    "ConsoleEmailProvider",
    "ConsolePushProvider",
    "ConsoleSMSProvider",
    "WebhookProvider",
    "TemplateEngine",
    "render_template",
]
