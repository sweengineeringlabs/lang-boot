# Rustboot OpenAPI

OpenAPI/Swagger documentation generation for the Rustboot framework.

## Features

- **OpenAPI 3.0 Specification**: Complete support for OpenAPI 3.0.3 specification
- **Automatic Schema Generation**: Derive macros for automatic schema generation from Rust types
- **Builder API**: Fluent API for constructing OpenAPI specifications programmatically
- **Swagger UI Integration**: Built-in Swagger UI support for interactive documentation
- **Utoipa Integration**: Optional integration with the utoipa crate
- **Multiple Output Formats**: Generate JSON and YAML specifications

## Feature Flags

- `utoipa` - Enables utoipa integration for automatic OpenAPI generation
- `swagger-ui` - Enables Swagger UI HTML generation
- `yaml` - Enables YAML output format

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dev-engineeringlabs-rustboot-openapi = "0.1.0"
rustboot-macros = "0.1.0"

# Optional: Enable utoipa support
dev-engineeringlabs-rustboot-openapi = { version = "0.1.0", features = ["utoipa", "swagger-ui"] }
```

## Quick Start

### 1. Define Your Models

Use the `#[derive(OpenApiSchema)]` macro to automatically generate OpenAPI schemas:

```rust
use rustboot_macros::OpenApiSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, OpenApiSchema)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize, OpenApiSchema)]
struct CreateUserRequest {
    name: String,
    email: String,
}
```

### 2. Build Your API Specification

```rust
use dev_engineeringlabs_rustboot_openapi::*;

let spec = OpenApiBuilder::new()
    .title("User API")
    .version("1.0.0")
    .description("API for managing users")
    .server("https://api.example.com", Some("Production".to_string()))
    .tag("users", Some("User management".to_string()))
    .build();

// Add generated schemas
spec.add_schema(User::schema_name().unwrap(), User::schema());
spec.add_schema(CreateUserRequest::schema_name().unwrap(), CreateUserRequest::schema());
```

### 3. Define API Endpoints

```rust
use builder::{PathItemBuilder, OperationBuilder};
use spec::{Response, Parameter, ParameterLocation};

let get_user = OperationBuilder::new()
    .tag("users")
    .summary("Get user by ID")
    .operation_id("getUserById")
    .parameter(Parameter {
        name: "id".to_string(),
        location: ParameterLocation::Path,
        description: Some("User ID".to_string()),
        required: Some(true),
        deprecated: None,
        schema: Some(builder::schemas::integer()),
    })
    .response("200", Response {
        description: "User found".to_string(),
        content: create_json_content(User::schema()),
        headers: HashMap::new(),
    })
    .build();

let path = PathItemBuilder::new()
    .get(get_user)
    .build();

spec.add_path("/users/{id}".to_string(), path);
```

### 4. Generate Documentation

```rust
// Generate JSON
let json = spec.to_json().unwrap();
println!("{}", json);

// Generate YAML (with yaml feature)
#[cfg(feature = "yaml")]
let yaml = spec.to_yaml().unwrap();

// Generate Swagger UI
use swagger_ui::{SwaggerUiConfig, SwaggerUiServer};

let config = SwaggerUiConfig::new("/swagger-ui", "/api-docs/openapi.json")
    .title("My API Documentation");

let server = SwaggerUiServer::new(spec, config);
let html = server.ui_html();
```

## Examples

The crate includes several examples:

- `basic_openapi` - Basic OpenAPI specification building
- `derive_schema` - Using derive macros for schema generation
- `swagger_ui` - Swagger UI integration
- `utoipa_integration` - Integration with utoipa (requires `utoipa` feature)
- `complete_api` - Complete API documentation workflow

Run examples with:

```bash
cargo run --example basic_openapi
cargo run --example derive_schema
cargo run --example swagger_ui
cargo run --example utoipa_integration --features utoipa
cargo run --example complete_api
```

## Schema Generation

### Automatic Schema Derivation

```rust
#[derive(OpenApiSchema)]
struct User {
    id: u64,              // Required field
    name: String,         // Required field
    email: Option<String>, // Optional field (nullable)
    roles: Vec<String>,   // Array field
}
```

### Enum Support

```rust
#[derive(OpenApiSchema)]
enum Status {
    Active,
    Inactive,
    Pending,
}
```

### Manual Schema Building

```rust
use builder::schemas;

let user_schema = Schema::Object(SchemaObject {
    schema_type: Some("object".to_string()),
    properties: {
        let mut props = HashMap::new();
        props.insert("id".to_string(), schemas::integer());
        props.insert("name".to_string(), schemas::string());
        props
    },
    required: vec!["id".to_string(), "name".to_string()],
    ..Default::default()
});
```

## Security Schemes

```rust
use spec::{SecurityScheme, ParameterLocation};

// Bearer token authentication
let bearer = SecurityScheme::Http {
    scheme: "bearer".to_string(),
    bearer_format: Some("JWT".to_string()),
    description: Some("JWT token".to_string()),
};

// API key authentication
let api_key = SecurityScheme::ApiKey {
    location: ParameterLocation::Header,
    name: "X-API-Key".to_string(),
    description: Some("API key".to_string()),
};

spec.security_scheme("bearerAuth", bearer);
spec.security_scheme("apiKey", api_key);
```

## Utoipa Integration

When the `utoipa` feature is enabled, you can convert between utoipa and rustboot-openapi formats:

```rust
use utoipa::{OpenApi, ToSchema};
use dev_engineeringlabs_rustboot_openapi::utoipa_support;

#[derive(ToSchema)]
struct User {
    id: i64,
    name: String,
}

#[derive(OpenApi)]
#[openapi(components(schemas(User)))]
struct ApiDoc;

let utoipa_spec = ApiDoc::openapi();
let rustboot_spec = utoipa_support::from_utoipa(&utoipa_spec).unwrap();
```

## Swagger UI Integration

Serve interactive API documentation:

```rust
use swagger_ui::{SwaggerUiConfig, SwaggerUiServer};

let config = SwaggerUiConfig::new("/swagger-ui", "/api-docs/openapi.json")
    .title("My API");

let server = SwaggerUiServer::new(spec, config);

// In your web framework:
// GET /swagger-ui -> server.ui_html()
// GET /api-docs/openapi.json -> server.spec_json()
```

## Best Practices

1. **Use Derive Macros**: Leverage `#[derive(OpenApiSchema)]` for automatic schema generation
2. **Organize by Tags**: Group related endpoints using tags
3. **Document Thoroughly**: Add descriptions to all operations and parameters
4. **Version Your API**: Use semantic versioning in the info section
5. **Define Security**: Clearly specify authentication requirements
6. **Provide Examples**: Include example values in schemas and responses
7. **Use References**: Reference shared schemas to avoid duplication

## Integration with Web Frameworks

The generated OpenAPI specifications can be served through any Rust web framework:

```rust
// Example with Axum (pseudo-code)
async fn serve_openapi() -> String {
    spec.to_json().unwrap()
}

async fn serve_swagger_ui() -> Html<String> {
    Html(server.ui_html())
}

let app = Router::new()
    .route("/api-docs/openapi.json", get(serve_openapi))
    .route("/swagger-ui", get(serve_swagger_ui));
```

## Testing

Run the test suite:

```bash
cargo test
cargo test --features utoipa
cargo test --features yaml
cargo test --all-features
```

## License

This crate is part of the Rustboot framework and is licensed under the MIT License.

## Contributing

Contributions are welcome! Please see the main Rustboot repository for contribution guidelines.
