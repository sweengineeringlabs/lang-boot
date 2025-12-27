"""
Fake data generators - Test data factories.
"""

import random
import string
import uuid
from datetime import datetime, date, timedelta
from typing import Any


class FakeDataGenerator:
    """Fake data generator for tests.
    
    Example:
        from dev.engineeringlabs.pyboot.testing import fake
        
        name = fake.name()
        email = fake.email()
        user_id = fake.uuid()
        created = fake.datetime_past()
    """
    
    _first_names = ["Alice", "Bob", "Charlie", "Diana", "Eve", "Frank", "Grace", "Henry"]
    _last_names = ["Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller"]
    _domains = ["example.com", "test.org", "demo.net", "sample.io"]
    
    def uuid(self) -> str:
        """Generate a random UUID."""
        return str(uuid.uuid4())
    
    def name(self) -> str:
        """Generate a random full name."""
        return f"{random.choice(self._first_names)} {random.choice(self._last_names)}"
    
    def first_name(self) -> str:
        """Generate a random first name."""
        return random.choice(self._first_names)
    
    def last_name(self) -> str:
        """Generate a random last name."""
        return random.choice(self._last_names)
    
    def email(self, domain: str | None = None) -> str:
        """Generate a random email address."""
        name = self.first_name().lower()
        suffix = self.string(4, string.digits)
        domain = domain or random.choice(self._domains)
        return f"{name}{suffix}@{domain}"
    
    def string(self, length: int = 10, alphabet: str | None = None) -> str:
        """Generate a random string."""
        chars = alphabet or string.ascii_letters
        return "".join(random.choice(chars) for _ in range(length))
    
    def integer(self, min_val: int = 0, max_val: int = 1000) -> int:
        """Generate a random integer."""
        return random.randint(min_val, max_val)
    
    def boolean(self) -> bool:
        """Generate a random boolean."""
        return random.choice([True, False])
    
    def choice(self, items: list[Any]) -> Any:
        """Choose a random item from list."""
        return random.choice(items)
    
    def datetime_past(self, days_ago: int = 365) -> datetime:
        """Generate a random datetime in the past."""
        delta = timedelta(days=random.randint(1, days_ago))
        return datetime.now() - delta
    
    def datetime_future(self, days_ahead: int = 365) -> datetime:
        """Generate a random datetime in the future."""
        delta = timedelta(days=random.randint(1, days_ahead))
        return datetime.now() + delta
    
    def date_past(self, days_ago: int = 365) -> date:
        """Generate a random date in the past."""
        return self.datetime_past(days_ago).date()
    
    def phone(self) -> str:
        """Generate a random phone number."""
        return f"+1-{self.string(3, string.digits)}-{self.string(3, string.digits)}-{self.string(4, string.digits)}"
    
    def url(self) -> str:
        """Generate a random URL."""
        domain = random.choice(self._domains)
        path = self.string(8, string.ascii_lowercase)
        return f"https://{domain}/{path}"
    
    def ipv4(self) -> str:
        """Generate a random IPv4 address."""
        return ".".join(str(random.randint(1, 255)) for _ in range(4))
    
    def sentence(self, word_count: int = 8) -> str:
        """Generate a random sentence."""
        words = [self.string(random.randint(3, 10), string.ascii_lowercase) for _ in range(word_count)]
        words[0] = words[0].capitalize()
        return " ".join(words) + "."


# Global instance
fake = FakeDataGenerator()
