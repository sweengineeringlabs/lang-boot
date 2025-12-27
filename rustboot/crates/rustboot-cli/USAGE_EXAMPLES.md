# Rustboot CLI Usage Examples

## Quick Start

### 1. Create a Simple Web Service

```bash
# Create a new project
rustboot new hello-api

# Navigate to the project
cd hello-api

# The generated project includes:
# - Basic HTTP client setup
# - Logging with tracing
# - Configuration file
# - Docker and Kubernetes manifests

# Build and run
cargo run
```

### 2. Create a Database-Backed API

```bash
# Create project
rustboot new user-service
cd user-service

# Add database support
rustboot add database

# Add API models
rustboot add api

# Configure database
cp .env.example .env
# Edit .env and set DATABASE_URL

# Update src/main.rs to include:
# mod database;
# mod models;

# Build and run
cargo run
```

### 3. Create a Secure API with Authentication

```bash
# Create project
rustboot new secure-api
cd secure-api

# Add all features
rustboot add database
rustboot add auth
rustboot add api

# The project now has:
# - Database connection pooling (src/database.rs)
# - JWT authentication middleware (src/auth.rs)
# - API models with OpenAPI support (src/models.rs)
```

## Detailed Examples

### Example 1: Hello World Service

```bash
rustboot new hello-world
cd hello-world
```

The generated `src/main.rs`:

```rust
//! hello-world - A Rustboot Application
//!
//! This is a basic web server built with the Rustboot framework.

use dev_engineeringlabs_rustboot_http::{HttpClient, ReqwestClient};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting hello-world...");

    // Create HTTP client
    let client = ReqwestClient::new();

    // Example: Make a simple HTTP request
    let response = client.get("https://httpbin.org/get").await?;
    info!("Response status: {}", response.status);
    info!("Response successful: {}", response.is_success());

    info!("hello-world is running!");

    Ok(())
}
```

### Example 2: Database Service

```bash
rustboot new db-service
cd db-service
rustboot add database
```

Update `src/main.rs`:

```rust
mod database;

use dev_engineeringlabs_rustboot_http::{HttpClient, ReqwestClient};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting db-service...");

    // Initialize database pool
    let pool = database::init_database().await?;
    info!("Database connection pool initialized");

    // Your application logic here

    Ok(())
}
```

### Example 3: Complete API Service

```bash
rustboot new complete-api
cd complete-api
rustboot add database
rustboot add auth
rustboot add api
```

Update `src/main.rs`:

```rust
mod database;
mod auth;
mod models;

use dev_engineeringlabs_rustboot_http::{HttpClient, ReqwestClient};
use dev_engineeringlabs_rustboot_middleware::{MiddlewareChain, Context};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Starting complete-api...");

    // Initialize database
    let pool = database::init_database().await?;
    info!("Database initialized");

    // Setup authentication
    let auth_middleware = auth::AuthMiddleware::new(
        std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string())
    );

    // Create middleware chain
    let mut chain = MiddlewareChain::new();
    chain.add(Box::new(auth_middleware));

    info!("API server ready!");

    Ok(())
}
```

## Docker Deployment

Every generated project includes a Dockerfile:

```bash
# Build Docker image
docker build -t my-api .

# Run container
docker run -p 8080:8080 my-api
```

## Kubernetes Deployment

Every generated project includes Kubernetes manifests:

```bash
# Apply deployment
kubectl apply -f deployment.yaml

# Check status
kubectl get pods
kubectl get services

# Access the service
kubectl port-forward service/my-api-service 8080:80
```

## Project Customization

### Adding Custom Dependencies

Edit `Cargo.toml` to add more dependencies:

```toml
[dependencies]
# ... existing dependencies ...
axum = "0.7"
tower = "0.4"
```

### Modifying Configuration

Edit `config.toml`:

```toml
[server]
host = "0.0.0.0"
port = 3000

[logging]
level = "debug"
format = "pretty"

[database]
max_connections = 20
min_connections = 5
```

### Adding More Features

You can run `rustboot add` multiple times:

```bash
rustboot add database
rustboot add auth
rustboot add api
```

## Tips and Best Practices

1. **Start Simple**: Begin with `rustboot new` and add features as needed
2. **Use Environment Variables**: Store secrets in `.env` files (not committed to git)
3. **Follow the Generated Structure**: The templates provide a good starting point
4. **Leverage Middleware**: Use the middleware pattern for cross-cutting concerns
5. **Read the READMEs**: Each generated project includes documentation

## Troubleshooting

### Project Already Exists

```bash
# Error: Directory 'my-api' already exists
# Solution: Use a different name or remove the existing directory
rm -rf my-api
rustboot new my-api
```

### Not in a Rustboot Project

```bash
# Error: Not in a Rustboot project
# Solution: Run 'rustboot add' from the project root where Cargo.toml exists
cd /path/to/my-project
rustboot add database
```

### Missing Dependencies

```bash
# After adding features, run:
cargo build

# This will download and compile all new dependencies
```

## Advanced Usage

### Custom Template Path

Currently, the CLI uses embedded templates. To customize:

1. Build the CLI from source
2. Modify templates in `crates/rustboot-cli/templates/`
3. Rebuild: `cargo build -p rustboot-cli --release`

### Scripting

Use the CLI in scripts:

```bash
#!/bin/bash
# create-services.sh

for service in auth users products orders; do
    rustboot new "$service-service"
    cd "$service-service"
    rustboot add database
    rustboot add api
    cd ..
done
```

## Next Steps

After creating your project:

1. Review the generated `README.md`
2. Customize `config.toml` for your needs
3. Implement your business logic
4. Add tests
5. Deploy using Docker or Kubernetes

For more information, see:
- [Rustboot Documentation](https://github.com/phdsystems/rustboot)
- [Rustboot CLI README](README.md)
