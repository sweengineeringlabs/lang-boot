# Config Module Overview

## WHAT: Configuration Management

Multi-source configuration with type-safe access, environment binding, and hot-reload support.

Key capabilities:
- **Multi-Source** - Files, environment, system properties
- **Type-Safe** - Strongly typed configuration classes
- **Profiles** - Environment-specific configuration
- **Hot-Reload** - Dynamic configuration updates

## WHY: Flexible Configuration

**Problems Solved**:
1. **Hardcoded Values** - Externalize configuration
2. **Environment Differences** - Profile-based config
3. **Type Errors** - Compile-time config validation

**When to Use**: All applications needing external configuration

## HOW: Usage Guide

```java
var config = Config.load("application.yml");

String dbUrl = config.getString("database.url");
int port = config.getInt("server.port", 8080);

// Bind to class
var dbConfig = config.bind("database", DatabaseConfig.class);
```

## Relationship to Other Modules

| Module | Relationship |
|--------|--------------|
| All modules | Configuration source |

---

**Status**: Stable
