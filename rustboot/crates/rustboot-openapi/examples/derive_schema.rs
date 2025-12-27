//! OpenAPI Schema Derivation Example
//!
//! This example demonstrates using the #[derive(OpenApiSchema)] macro
//! to automatically generate OpenAPI schemas from Rust types.
//!
//! Run with: cargo run --example derive_schema

use dev_engineeringlabs_rustboot_openapi::*;
use rustboot_macros::OpenApiSchema;
use serde::{Deserialize, Serialize};

// Derive OpenAPI schema for a User struct
#[derive(Debug, Serialize, Deserialize, OpenApiSchema)]
struct User {
    id: u64,
    username: String,
    email: String,
    age: Option<u32>,
    roles: Vec<String>,
}

// Derive OpenAPI schema for a CreateUserRequest
#[derive(Debug, Serialize, Deserialize, OpenApiSchema)]
struct CreateUserRequest {
    username: String,
    email: String,
    password: String,
    age: Option<u32>,
}

// Derive OpenAPI schema for an enum
#[derive(Debug, Serialize, Deserialize, OpenApiSchema)]
enum UserStatus {
    Active,
    Inactive,
    Suspended,
}

// Another complex example
#[derive(Debug, Serialize, Deserialize, OpenApiSchema)]
struct Post {
    id: u64,
    title: String,
    content: String,
    author_id: u64,
    tags: Vec<String>,
    published: bool,
}

fn main() {
    println!("=== OpenAPI Schema Derivation Example ===\n");

    // Build API spec with derived schemas
    let mut spec = OpenApiBuilder::new()
        .title("User & Post API")
        .version("1.0.0")
        .description("API demonstrating automatic schema generation")
        .server("https://api.example.com", None)
        .tag("users", Some("User management".to_string()))
        .tag("posts", Some("Post management".to_string()))
        .build();

    // Add schemas using the derived implementations
    println!("1. Adding schemas from derived types:");

    let user_schema = User::schema();
    let user_name = User::schema_name().unwrap();
    println!("   - {} schema generated", user_name);
    spec.add_schema(user_name, user_schema);

    let create_user_schema = CreateUserRequest::schema();
    let create_user_name = CreateUserRequest::schema_name().unwrap();
    println!("   - {} schema generated", create_user_name);
    spec.add_schema(create_user_name, create_user_schema);

    let status_schema = UserStatus::schema();
    let status_name = UserStatus::schema_name().unwrap();
    println!("   - {} schema generated", status_name);
    spec.add_schema(status_name, status_schema);

    let post_schema = Post::schema();
    let post_name = Post::schema_name().unwrap();
    println!("   - {} schema generated", post_name);
    spec.add_schema(post_name, post_schema);

    println!();

    // Display the generated spec
    println!("2. Generated OpenAPI Specification:");
    let json = spec.to_json().unwrap();
    println!("{}\n", json);

    // Show individual schema details
    println!("3. Schema Details:");
    if let Some(components) = &spec.components {
        for (name, schema) in &components.schemas {
            println!("   Schema: {}", name);
            if let Schema::Object(obj) = schema {
                if let Some(schema_type) = &obj.schema_type {
                    println!("     Type: {}", schema_type);
                }
                if !obj.properties.is_empty() {
                    println!("     Properties: {}", obj.properties.len());
                    for (prop_name, _) in &obj.properties {
                        println!("       - {}", prop_name);
                    }
                }
                if !obj.required.is_empty() {
                    println!("     Required: {:?}", obj.required);
                }
                if !obj.enum_values.is_empty() {
                    println!("     Enum values: {:?}", obj.enum_values);
                }
            }
            println!();
        }
    }

    println!("=== Example Complete ===");
}
