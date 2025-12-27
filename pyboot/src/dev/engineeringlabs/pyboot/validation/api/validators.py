"""Common validators."""

import re
from typing import Any, Sequence

from dev.engineeringlabs.pyboot.validation.api.validator import Validator


class Required(Validator):
    """Validate that a value is present."""

    def __init__(self, message: str = "This field is required"):
        self._message = message

    def validate(self, value: Any) -> str | None:
        if value is None:
            return self._message
        if isinstance(value, str) and not value.strip():
            return self._message
        return None


class NotEmpty(Validator):
    """Validate that a value is not empty."""

    def __init__(self, message: str = "This field cannot be empty"):
        self._message = message

    def validate(self, value: Any) -> str | None:
        if value is None:
            return self._message
        if isinstance(value, (str, list, dict, set)) and len(value) == 0:
            return self._message
        return None


class MinLength(Validator):
    """Validate minimum length."""

    def __init__(self, min_len: int, message: str | None = None):
        self._min_len = min_len
        self._message = message or f"Must be at least {min_len} characters"

    def validate(self, value: Any) -> str | None:
        if value is None:
            return None  # Use Required for null check
        if hasattr(value, "__len__") and len(value) < self._min_len:
            return self._message
        return None


class MaxLength(Validator):
    """Validate maximum length."""

    def __init__(self, max_len: int, message: str | None = None):
        self._max_len = max_len
        self._message = message or f"Must be at most {max_len} characters"

    def validate(self, value: Any) -> str | None:
        if value is None:
            return None
        if hasattr(value, "__len__") and len(value) > self._max_len:
            return self._message
        return None


class Pattern(Validator):
    """Validate against a regex pattern."""

    def __init__(self, pattern: str, message: str = "Invalid format"):
        self._pattern = re.compile(pattern)
        self._message = message

    def validate(self, value: Any) -> str | None:
        if value is None:
            return None
        if not isinstance(value, str):
            return self._message
        if not self._pattern.match(value):
            return self._message
        return None


class Email(Validator):
    """Validate email format."""

    _EMAIL_PATTERN = re.compile(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    )

    def __init__(self, message: str = "Invalid email address"):
        self._message = message

    def validate(self, value: Any) -> str | None:
        if value is None:
            return None
        if not isinstance(value, str):
            return self._message
        if not self._EMAIL_PATTERN.match(value):
            return self._message
        return None


class Url(Validator):
    """Validate URL format."""

    _URL_PATTERN = re.compile(
        r"^https?://[^\s/$.?#].[^\s]*$", re.IGNORECASE
    )

    def __init__(self, message: str = "Invalid URL"):
        self._message = message

    def validate(self, value: Any) -> str | None:
        if value is None:
            return None
        if not isinstance(value, str):
            return self._message
        if not self._URL_PATTERN.match(value):
            return self._message
        return None


class MinValue(Validator):
    """Validate minimum value."""

    def __init__(self, min_val: float, message: str | None = None):
        self._min_val = min_val
        self._message = message or f"Must be at least {min_val}"

    def validate(self, value: Any) -> str | None:
        if value is None:
            return None
        try:
            if float(value) < self._min_val:
                return self._message
        except (ValueError, TypeError):
            return self._message
        return None


class MaxValue(Validator):
    """Validate maximum value."""

    def __init__(self, max_val: float, message: str | None = None):
        self._max_val = max_val
        self._message = message or f"Must be at most {max_val}"

    def validate(self, value: Any) -> str | None:
        if value is None:
            return None
        try:
            if float(value) > self._max_val:
                return self._message
        except (ValueError, TypeError):
            return self._message
        return None


class InRange(Validator):
    """Validate value is in range."""

    def __init__(self, min_val: float, max_val: float, message: str | None = None):
        self._min_val = min_val
        self._max_val = max_val
        self._message = message or f"Must be between {min_val} and {max_val}"

    def validate(self, value: Any) -> str | None:
        if value is None:
            return None
        try:
            val = float(value)
            if val < self._min_val or val > self._max_val:
                return self._message
        except (ValueError, TypeError):
            return self._message
        return None


class OneOf(Validator):
    """Validate value is one of allowed values."""

    def __init__(self, choices: Sequence[Any], message: str | None = None):
        self._choices = choices
        self._message = message or f"Must be one of: {', '.join(str(c) for c in choices)}"

    def validate(self, value: Any) -> str | None:
        if value is None:
            return None
        if value not in self._choices:
            return self._message
        return None


class Uuid(Validator):
    """Validate UUID format.
    
    Supports UUID versions 1-5 and validates the format.
    Accepts both with and without hyphens.
    
    Example:
        validator = Uuid()
        validator.validate("550e8400-e29b-41d4-a716-446655440000")  # Valid
        validator.validate("550e8400e29b41d4a716446655440000")      # Valid
        validator.validate("invalid-uuid")                           # Invalid
    """
    
    # UUID pattern: 8-4-4-4-12 hex digits with optional hyphens
    _UUID_PATTERN = re.compile(
        r"^[0-9a-fA-F]{8}-?[0-9a-fA-F]{4}-?[1-5][0-9a-fA-F]{3}-?[89abAB][0-9a-fA-F]{3}-?[0-9a-fA-F]{12}$"
    )
    
    def __init__(self, message: str = "Invalid UUID format"):
        self._message = message
    
    def validate(self, value: Any) -> str | None:
        if value is None:
            return None
        
        # Accept UUID objects
        import uuid as uuid_module
        if isinstance(value, uuid_module.UUID):
            return None
        
        if not isinstance(value, str):
            return self._message
        
        # Try parsing with uuid module for strict validation
        try:
            uuid_module.UUID(value)
            return None
        except (ValueError, AttributeError):
            pass
        
        # Fallback to pattern matching
        if not self._UUID_PATTERN.match(value):
            return self._message
        return None


class Phone(Validator):
    """Validate phone number format.
    
    Supports multiple formats:
    - E.164: +14155551234
    - International: +1 415 555 1234
    - US format: (415) 555-1234, 415-555-1234
    - With extensions: +1 415 555 1234 ext 123
    
    Example:
        validator = Phone()
        validator.validate("+14155551234")        # Valid
        validator.validate("(415) 555-1234")      # Valid
        validator.validate("415-555-1234")        # Valid
        validator.validate("123")                  # Invalid
    """
    
    # Comprehensive phone pattern
    _PHONE_PATTERNS = [
        # E.164 format: +[country][number] (7-15 digits)
        re.compile(r"^\+[1-9]\d{6,14}$"),
        # International with spaces/dashes
        re.compile(r"^\+[1-9][\d\s\-]{6,20}$"),
        # US format: (xxx) xxx-xxxx
        re.compile(r"^\(\d{3}\)\s?\d{3}[-.]?\d{4}$"),
        # US format without parentheses: xxx-xxx-xxxx
        re.compile(r"^\d{3}[-.\s]?\d{3}[-.\s]?\d{4}$"),
        # 10+ digit number
        re.compile(r"^\d{10,15}$"),
    ]
    
    def __init__(
        self,
        message: str = "Invalid phone number",
        min_digits: int = 7,
        max_digits: int = 15,
    ):
        self._message = message
        self._min_digits = min_digits
        self._max_digits = max_digits
    
    def validate(self, value: Any) -> str | None:
        if value is None:
            return None
        if not isinstance(value, str):
            return self._message
        
        # Normalize: remove spaces, dashes, parentheses for digit count
        cleaned = re.sub(r"[\s\-\(\)\.]", "", value)
        
        # Remove leading + for digit count
        digits_only = cleaned.lstrip("+")
        
        # Check digit count
        if not digits_only.isdigit():
            return self._message
        if len(digits_only) < self._min_digits or len(digits_only) > self._max_digits:
            return self._message
        
        # Check against known patterns
        for pattern in self._PHONE_PATTERNS:
            if pattern.match(value):
                return None
        
        # Accept if it has valid digit count even without matching pattern
        if self._min_digits <= len(digits_only) <= self._max_digits:
            return None
        
        return self._message


class IpAddress(Validator):
    """Validate IP address format.
    
    Supports:
    - IPv4: 192.168.1.1
    - IPv6: 2001:0db8:85a3:0000:0000:8a2e:0370:7334
    - IPv6 shortened: 2001:db8:85a3::8a2e:370:7334
    - CIDR notation (optional): 192.168.1.0/24
    
    Example:
        validator = IpAddress()
        validator.validate("192.168.1.1")    # Valid
        validator.validate("::1")             # Valid (localhost IPv6)
        validator.validate("999.999.999.999") # Invalid
        
        # IPv4 only
        validator = IpAddress(version=4)
        
        # With CIDR
        validator = IpAddress(allow_cidr=True)
        validator.validate("192.168.1.0/24") # Valid
    """
    
    def __init__(
        self,
        version: int | None = None,
        allow_cidr: bool = False,
        message: str = "Invalid IP address",
    ):
        """
        Args:
            version: IP version (4, 6, or None for both).
            allow_cidr: Allow CIDR notation (e.g., 192.168.1.0/24).
            message: Error message.
        """
        self._version = version
        self._allow_cidr = allow_cidr
        self._message = message
    
    def validate(self, value: Any) -> str | None:
        if value is None:
            return None
        if not isinstance(value, str):
            return self._message
        
        # Handle CIDR notation
        ip_part = value
        if "/" in value:
            if not self._allow_cidr:
                return self._message
            ip_part, cidr = value.rsplit("/", 1)
            if not cidr.isdigit():
                return self._message
            cidr_int = int(cidr)
            # Validate CIDR range later based on IP version
        
        # Try using ipaddress module for validation
        try:
            import ipaddress
            
            if self._version == 4:
                addr = ipaddress.IPv4Address(ip_part)
                if "/" in value and int(value.rsplit("/", 1)[1]) > 32:
                    return self._message
            elif self._version == 6:
                addr = ipaddress.IPv6Address(ip_part)
                if "/" in value and int(value.rsplit("/", 1)[1]) > 128:
                    return self._message
            else:
                # Try IPv4 first, then IPv6
                try:
                    addr = ipaddress.IPv4Address(ip_part)
                    if "/" in value and int(value.rsplit("/", 1)[1]) > 32:
                        return self._message
                except ipaddress.AddressValueError:
                    addr = ipaddress.IPv6Address(ip_part)
                    if "/" in value and int(value.rsplit("/", 1)[1]) > 128:
                        return self._message
            
            return None
            
        except (ValueError, ipaddress.AddressValueError):
            return self._message


# Factory functions for cleaner usage
def required(message: str = "This field is required") -> Required:
    """Create a required validator."""
    return Required(message)


def not_empty(message: str = "This field cannot be empty") -> NotEmpty:
    """Create a not-empty validator."""
    return NotEmpty(message)


def min_length(length: int, message: str | None = None) -> MinLength:
    """Create a minimum length validator."""
    return MinLength(length, message)


def max_length(length: int, message: str | None = None) -> MaxLength:
    """Create a maximum length validator."""
    return MaxLength(length, message)


def pattern(regex: str, message: str = "Invalid format") -> Pattern:
    """Create a pattern validator."""
    return Pattern(regex, message)


def email(message: str = "Invalid email address") -> Email:
    """Create an email validator."""
    return Email(message)


def url(message: str = "Invalid URL") -> Url:
    """Create a URL validator."""
    return Url(message)


def min_value(value: float, message: str | None = None) -> MinValue:
    """Create a minimum value validator."""
    return MinValue(value, message)


def max_value(value: float, message: str | None = None) -> MaxValue:
    """Create a maximum value validator."""
    return MaxValue(value, message)


def in_range(min_val: float, max_val: float, message: str | None = None) -> InRange:
    """Create an in-range validator."""
    return InRange(min_val, max_val, message)


def one_of(choices: Sequence[Any], message: str | None = None) -> OneOf:
    """Create a one-of validator."""
    return OneOf(choices, message)


def uuid(message: str = "Invalid UUID format") -> Uuid:
    """Create a UUID validator.
    
    Example:
        errors = validate(
            ("id", user_id, [required(), uuid()]),
        )
    """
    return Uuid(message)


def phone(
    message: str = "Invalid phone number",
    min_digits: int = 7,
    max_digits: int = 15,
) -> Phone:
    """Create a phone number validator.
    
    Args:
        message: Error message.
        min_digits: Minimum digits required.
        max_digits: Maximum digits allowed.
    
    Example:
        errors = validate(
            ("phone", user_phone, [required(), phone()]),
        )
    """
    return Phone(message, min_digits, max_digits)


def ip_address(
    version: int | None = None,
    allow_cidr: bool = False,
    message: str = "Invalid IP address",
) -> IpAddress:
    """Create an IP address validator.
    
    Args:
        version: IP version (4, 6, or None for both).
        allow_cidr: Allow CIDR notation.
        message: Error message.
    
    Example:
        # Any IP
        errors = validate(("ip", server_ip, [ip_address()]))
        
        # IPv4 only
        errors = validate(("ip", server_ip, [ip_address(version=4)]))
        
        # With CIDR
        errors = validate(("subnet", net, [ip_address(allow_cidr=True)]))
    """
    return IpAddress(version, allow_cidr, message)


__all__ = [
    # Classes
    "Required",
    "NotEmpty",
    "MinLength",
    "MaxLength",
    "Pattern",
    "Email",
    "Url",
    "MinValue",
    "MaxValue",
    "InRange",
    "OneOf",
    "Uuid",
    "Phone",
    "IpAddress",
    # Factory functions
    "required",
    "not_empty",
    "min_length",
    "max_length",
    "pattern",
    "email",
    "url",
    "min_value",
    "max_value",
    "in_range",
    "one_of",
    "uuid",
    "phone",
    "ip_address",
]

