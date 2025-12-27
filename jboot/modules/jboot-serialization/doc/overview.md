# Serialization Module Overview

## WHAT: Serialization Formats

JSON, XML, and binary serialization with type-safe encoding/decoding.

Key capabilities:
- **JSON** - Jackson-based JSON handling
- **XML** - JAXB XML serialization
- **Binary** - Protocol buffers, MessagePack
- **Custom** - Pluggable serializers

## WHY: Data Exchange

**Problems Solved**:
1. **Format Handling** - Unified serialization API
2. **Type Safety** - Generic type handling
3. **Performance** - Optimized encoders

**When to Use**: API responses, message encoding

## HOW: Usage Guide

```java
var json = Json.create();

// Serialize
String jsonStr = json.encode(user);

// Deserialize
User user = json.decode(jsonStr, User.class);

// Pretty print
String pretty = json.encodePretty(user);
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| jboot-http | Response encoding |
| jboot-messaging | Message encoding |

---

**Status**: Stable
