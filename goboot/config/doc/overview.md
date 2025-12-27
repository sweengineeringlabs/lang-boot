# Config Module Overview

## WHAT: Configuration Management

Multi-source configuration with type-safe access and environment binding.

Key capabilities:
- **Multi-Source** - Files, environment, flags
- **Type-Safe** - Struct binding
- **Hot-Reload** - Dynamic updates
- **Defaults** - Fallback values

## WHY: Flexible Configuration

**Problems Solved**: Hardcoded values, environment differences

**When to Use**: All applications

## HOW: Usage Guide

```go
type AppConfig struct {
    Server struct {
        Port int    `yaml:"port" default:"8080"`
        Host string `yaml:"host" default:"localhost"`
    } `yaml:"server"`
    Database struct {
        URL string `yaml:"url" env:"DATABASE_URL"`
    } `yaml:"database"`
}

var cfg AppConfig
config.Load("config.yaml", &cfg)
```

---

**Status**: Stable
