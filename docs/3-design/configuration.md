# Configuration Conventions

Idiomatic configuration patterns for each language.

---

## 🦀 Rust

### Using `config` crate

```rust
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name("config/local").required(false))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?
            .try_deserialize()
    }
}
```

### File Formats

- TOML (preferred): `config.toml`
- YAML: `config.yaml`
- JSON: `config.json`

### Conventions

- Use `config` crate for layered config
- Environment variables override files
- Use `serde` for deserialization
- Prefix env vars: `APP_DATABASE__URL`

---

## 🦫 Go

### Using `viper`

```go
import "github.com/spf13/viper"

type Config struct {
    Database DatabaseConfig `mapstructure:"database"`
    Server   ServerConfig   `mapstructure:"server"`
}

func LoadConfig() (*Config, error) {
    viper.SetConfigName("config")
    viper.SetConfigType("yaml")
    viper.AddConfigPath(".")
    viper.AddConfigPath("./config")
    
    viper.AutomaticEnv()
    viper.SetEnvKeyReplacer(strings.NewReplacer(".", "_"))
    
    if err := viper.ReadInConfig(); err != nil {
        return nil, err
    }
    
    var config Config
    if err := viper.Unmarshal(&config); err != nil {
        return nil, err
    }
    return &config, nil
}
```

### Using Environment Only

```go
import "github.com/kelseyhightower/envconfig"

type Config struct {
    DatabaseURL string `envconfig:"DATABASE_URL" required:"true"`
    Port        int    `envconfig:"PORT" default:"8080"`
}

func LoadConfig() (*Config, error) {
    var cfg Config
    err := envconfig.Process("", &cfg)
    return &cfg, err
}
```

### Conventions

- Use `viper` for complex config
- Use `envconfig` for 12-factor apps
- Struct tags define mapping
- Environment overrides files

---

## ☕ Java

### Spring Boot (application.yml)

```yaml
# application.yml
server:
  port: 8080

database:
  url: jdbc:postgresql://localhost:5432/mydb
  max-connections: 10
```

```java
@ConfigurationProperties(prefix = "database")
public record DatabaseConfig(
    String url,
    int maxConnections
) {}

@Component
public class MyService {
    private final DatabaseConfig config;
    
    public MyService(DatabaseConfig config) {
        this.config = config;
    }
}
```

### Environment Variables

```bash
DATABASE_URL=jdbc:postgresql://... java -jar app.jar
# or
java -jar app.jar --database.url=jdbc:postgresql://...
```

### Conventions

- Use YAML for config files
- Profile-specific: `application-{profile}.yml`
- `@ConfigurationProperties` for type-safe config
- Environment variables use `_` as separator

---

## 🐍 Python

### Using `pydantic-settings`

```python
from pydantic_settings import BaseSettings

class DatabaseSettings(BaseSettings):
    url: str
    max_connections: int = 10
    
    class Config:
        env_prefix = "DATABASE_"

class Settings(BaseSettings):
    database: DatabaseSettings = DatabaseSettings()
    server_port: int = 8080
    
    class Config:
        env_file = ".env"
        env_nested_delimiter = "__"

settings = Settings()
```

### Using `python-dotenv`

```python
from dotenv import load_dotenv
import os

load_dotenv()

DATABASE_URL = os.getenv("DATABASE_URL")
PORT = int(os.getenv("PORT", "8080"))
```

### Conventions

- Use `pydantic-settings` for type-safe config
- Use `.env` files for local development
- Never commit `.env` to git
- Use `python-dotenv` or pydantic's env_file

---

## File Examples

### TOML (Rust)

```toml
[server]
port = 8080
host = "0.0.0.0"

[database]
url = "postgres://localhost/mydb"
max_connections = 10
```

### YAML (Go/Java)

```yaml
server:
  port: 8080
  host: "0.0.0.0"

database:
  url: "postgres://localhost/mydb"
  max_connections: 10
```

### .env (Python/All)

```bash
SERVER_PORT=8080
SERVER_HOST=0.0.0.0
DATABASE_URL=postgres://localhost/mydb
DATABASE_MAX_CONNECTIONS=10
```

---

## Comparison

| Aspect | Rust | Go | Java | Python |
|--------|------|-----|------|--------|
| Library | config | viper/envconfig | Spring | pydantic-settings |
| Format | TOML | YAML | YAML | .env/YAML |
| Env Override | ✅ | ✅ | ✅ | ✅ |
| Type-safe | ✅ serde | ✅ struct tags | ✅ @ConfigProperties | ✅ pydantic |

---

## Best Practices

1. **Environment for secrets** - Never commit secrets
2. **Layered config** - defaults → file → env
3. **Type-safe** - Deserialize to structs/classes
4. **Validation** - Fail fast on invalid config
5. **Document** - Provide example config files
