//! Complete API Documentation Example
//!
//! This example demonstrates a complete workflow for documenting a REST API
//! with OpenAPI, including schemas, paths, security, and Swagger UI.
//!
//! Run with: cargo run --example complete_api

use dev_engineeringlabs_rustboot_openapi::*;
use rustboot_macros::OpenApiSchema;
use dev_engineeringlabs_rustboot_openapi::builder::{PathItemBuilder, OperationBuilder};
use dev_engineeringlabs_rustboot_openapi::spec::{Response, Parameter, ParameterLocation, RequestBody, MediaType, SecurityScheme};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Domain models with derived schemas
#[derive(Debug, Serialize, Deserialize, OpenApiSchema)]
struct User {
    id: u64,
    username: String,
    email: String,
    created_at: String,
    role: UserRole,
}

#[derive(Debug, Serialize, Deserialize, OpenApiSchema)]
enum UserRole {
    Admin,
    User,
    Guest,
}

#[derive(Debug, Serialize, Deserialize, OpenApiSchema)]
struct CreateUserRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, OpenApiSchema)]
struct UpdateUserRequest {
    email: Option<String>,
    password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, OpenApiSchema)]
struct ErrorResponse {
    code: u32,
    message: String,
    details: Option<String>,
}

fn main() {
    println!("=== Complete API Documentation Example ===\n");

    // Build the complete API specification
    let spec = build_api_spec();

    // Display the spec
    println!("1. Generated OpenAPI Specification:");
    let json = spec.to_json().unwrap();
    println!("{}\n", json);

    // Show statistics
    show_spec_statistics(&spec);

    // Generate Swagger UI
    #[cfg(feature = "swagger-ui")]
    {
        println!("\n3. Swagger UI Integration:");
        let ui_server = create_swagger_ui(&spec);
        println!("   UI Path: {}", ui_server.ui_path());
        println!("   Spec URL: {}", ui_server.spec_url());
        println!("   HTML Size: {} bytes", ui_server.ui_html().len());
    }

    // Export to file (simulated)
    println!("\n4. Export Options:");
    println!("   JSON: {} bytes", json.len());

    #[cfg(feature = "yaml")]
    {
        let yaml = spec.to_yaml().unwrap();
        println!("   YAML: {} bytes", yaml.len());
    }

    println!("\n=== Example Complete ===");
    println!("\nGenerated documentation for:");
    println!("✓ User management endpoints");
    println!("✓ Authentication and authorization");
    println!("✓ Request/response schemas");
    println!("✓ Error responses");
    println!("✓ Interactive Swagger UI");
}

fn build_api_spec() -> OpenApiSpec {
    let mut spec = OpenApiBuilder::new()
        .title("User Management API")
        .version("2.0.0")
        .description("A comprehensive REST API for user management with authentication")
        .server("https://api.example.com/v2", Some("Production".to_string()))
        .server("https://staging-api.example.com/v2", Some("Staging".to_string()))
        .server("http://localhost:8080/v2", Some("Development".to_string()))
        .tag("users", Some("User account management operations".to_string()))
        .tag("auth", Some("Authentication and authorization".to_string()))
        .license("Apache 2.0", Some("https://www.apache.org/licenses/LICENSE-2.0.html".to_string()))
        .contact(
            "API Team",
            Some("api-team@example.com".to_string()),
            Some("https://example.com/support".to_string())
        )
        .terms_of_service("https://example.com/terms")
        .build();

    // Add security schemes
    spec.components = Some(spec::Components {
        schemas: HashMap::new(),
        responses: HashMap::new(),
        parameters: HashMap::new(),
        security_schemes: {
            let mut schemes = HashMap::new();
            schemes.insert(
                "bearerAuth".to_string(),
                SecurityScheme::Http {
                    scheme: "bearer".to_string(),
                    bearer_format: Some("JWT".to_string()),
                    description: Some("JWT Bearer token authentication".to_string()),
                },
            );
            schemes.insert(
                "apiKey".to_string(),
                SecurityScheme::ApiKey {
                    location: ParameterLocation::Header,
                    name: "X-API-Key".to_string(),
                    description: Some("API key authentication".to_string()),
                },
            );
            schemes
        },
    });

    // Add schemas
    add_schemas(&mut spec);

    // Add paths
    add_user_paths(&mut spec);

    spec
}

fn add_schemas(spec: &mut OpenApiSpec) {
    spec.add_schema(User::schema_name().unwrap(), User::schema());
    spec.add_schema(UserRole::schema_name().unwrap(), UserRole::schema());
    spec.add_schema(CreateUserRequest::schema_name().unwrap(), CreateUserRequest::schema());
    spec.add_schema(UpdateUserRequest::schema_name().unwrap(), UpdateUserRequest::schema());
    spec.add_schema(ErrorResponse::schema_name().unwrap(), ErrorResponse::schema());
}

fn add_user_paths(spec: &mut OpenApiSpec) {
    // GET /users - List users
    let list_users = OperationBuilder::new()
        .tag("users")
        .summary("List all users")
        .operation_id("listUsers")
        .description("Returns a paginated list of users")
        .parameter(Parameter {
            name: "page".to_string(),
            location: ParameterLocation::Query,
            description: Some("Page number (default: 1)".to_string()),
            required: Some(false),
            deprecated: None,
            schema: Some(builder::schemas::integer()),
        })
        .parameter(Parameter {
            name: "limit".to_string(),
            location: ParameterLocation::Query,
            description: Some("Items per page (default: 20, max: 100)".to_string()),
            required: Some(false),
            deprecated: None,
            schema: Some(builder::schemas::integer()),
        })
        .response("200", create_success_response(
            "List of users",
            Some(builder::schemas::array(builder::schemas::reference("User")))
        ))
        .response("401", create_error_response("Unauthorized"))
        .build();

    // POST /users - Create user
    let create_user = OperationBuilder::new()
        .tag("users")
        .summary("Create a new user")
        .operation_id("createUser")
        .request_body(create_json_request_body(
            "User to create",
            builder::schemas::reference("CreateUserRequest"),
            true
        ))
        .response("201", create_success_response(
            "User created successfully",
            Some(builder::schemas::reference("User"))
        ))
        .response("400", create_error_response("Invalid request"))
        .response("409", create_error_response("User already exists"))
        .build();

    let users_path = PathItemBuilder::new()
        .get(list_users)
        .post(create_user)
        .build();

    // GET /users/{id} - Get user by ID
    let get_user = OperationBuilder::new()
        .tag("users")
        .summary("Get user by ID")
        .operation_id("getUserById")
        .parameter(create_path_parameter("id", "User ID"))
        .response("200", create_success_response(
            "User found",
            Some(builder::schemas::reference("User"))
        ))
        .response("404", create_error_response("User not found"))
        .build();

    // PUT /users/{id} - Update user
    let update_user = OperationBuilder::new()
        .tag("users")
        .summary("Update user")
        .operation_id("updateUser")
        .parameter(create_path_parameter("id", "User ID"))
        .request_body(create_json_request_body(
            "User updates",
            builder::schemas::reference("UpdateUserRequest"),
            true
        ))
        .response("200", create_success_response(
            "User updated",
            Some(builder::schemas::reference("User"))
        ))
        .response("400", create_error_response("Invalid request"))
        .response("404", create_error_response("User not found"))
        .build();

    // DELETE /users/{id} - Delete user
    let delete_user = OperationBuilder::new()
        .tag("users")
        .summary("Delete user")
        .operation_id("deleteUser")
        .parameter(create_path_parameter("id", "User ID"))
        .response("204", Response {
            description: "User deleted successfully".to_string(),
            content: HashMap::new(),
            headers: HashMap::new(),
        })
        .response("404", create_error_response("User not found"))
        .build();

    let user_by_id_path = PathItemBuilder::new()
        .get(get_user)
        .put(update_user)
        .delete(delete_user)
        .build();

    spec.add_path("/users".to_string(), users_path);
    spec.add_path("/users/{id}".to_string(), user_by_id_path);
}

fn create_path_parameter(name: &str, description: &str) -> Parameter {
    Parameter {
        name: name.to_string(),
        location: ParameterLocation::Path,
        description: Some(description.to_string()),
        required: Some(true),
        deprecated: None,
        schema: Some(builder::schemas::integer()),
    }
}

fn create_json_request_body(description: &str, schema: Schema, required: bool) -> RequestBody {
    let mut content = HashMap::new();
    content.insert("application/json".to_string(), MediaType {
        schema: Some(schema),
        example: None,
        examples: HashMap::new(),
    });

    RequestBody {
        description: Some(description.to_string()),
        content,
        required: Some(required),
    }
}

fn create_success_response(description: &str, schema: Option<Schema>) -> Response {
    let mut content = HashMap::new();
    if let Some(schema) = schema {
        content.insert("application/json".to_string(), MediaType {
            schema: Some(schema),
            example: None,
            examples: HashMap::new(),
        });
    }

    Response {
        description: description.to_string(),
        content,
        headers: HashMap::new(),
    }
}

fn create_error_response(description: &str) -> Response {
    create_success_response(
        description,
        Some(builder::schemas::reference("ErrorResponse"))
    )
}

fn show_spec_statistics(spec: &OpenApiSpec) {
    println!("2. API Specification Statistics:");
    println!("   OpenAPI Version: {}", spec.openapi);
    println!("   API Title: {}", spec.info.title);
    println!("   API Version: {}", spec.info.version);
    println!("   Servers: {}", spec.servers.len());
    println!("   Tags: {}", spec.tags.len());
    println!("   Paths: {}", spec.paths.len());

    if let Some(components) = &spec.components {
        println!("   Schemas: {}", components.schemas.len());
        println!("   Security Schemes: {}", components.security_schemes.len());
    }

    // Count operations
    let mut operation_count = 0;
    for path_item in spec.paths.values() {
        if path_item.get.is_some() { operation_count += 1; }
        if path_item.post.is_some() { operation_count += 1; }
        if path_item.put.is_some() { operation_count += 1; }
        if path_item.delete.is_some() { operation_count += 1; }
        if path_item.patch.is_some() { operation_count += 1; }
    }
    println!("   Operations: {}", operation_count);
}

#[cfg(feature = "swagger-ui")]
fn create_swagger_ui(spec: &OpenApiSpec) -> dev_engineeringlabs_rustboot_openapi::swagger_ui::SwaggerUiServer {
    use dev_engineeringlabs_rustboot_openapi::swagger_ui::{SwaggerUiConfig, SwaggerUiServer};

    let config = SwaggerUiConfig::new("/docs", "/api/openapi.json")
        .title("User Management API - Interactive Documentation");

    SwaggerUiServer::new(spec.clone(), config)
}
