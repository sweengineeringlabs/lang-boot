//! Swagger UI Example
//!
//! This example demonstrates generating Swagger UI HTML for serving API documentation.
//!
//! Run with: cargo run --example swagger_ui

use dev_engineeringlabs_rustboot_openapi::*;
use dev_engineeringlabs_rustboot_openapi::builder::{PathItemBuilder, OperationBuilder};
use dev_engineeringlabs_rustboot_openapi::spec::{Response, Parameter, ParameterLocation};
use dev_engineeringlabs_rustboot_openapi::swagger_ui::{SwaggerUiConfig, SwaggerUiServer};
use std::collections::HashMap;

fn main() {
    println!("=== Swagger UI Example ===\n");

    // Build a sample API specification
    let spec = create_sample_spec();

    // Create Swagger UI configuration
    let config = SwaggerUiConfig::new("/swagger-ui", "/api-docs/openapi.json")
        .title("Pet Store API Documentation");

    // Create a Swagger UI server
    let server = SwaggerUiServer::new(spec, config);

    // Get the OpenAPI spec as JSON
    println!("1. OpenAPI Specification (JSON):");
    let spec_json = server.spec_json().unwrap();
    println!("{}\n", spec_json);

    // Get the Swagger UI HTML
    println!("2. Swagger UI HTML (first 500 chars):");
    let ui_html = server.ui_html();
    println!("{}...\n", &ui_html[..500.min(ui_html.len())]);

    // Show server configuration
    println!("3. Server Configuration:");
    println!("   UI Path: {}", server.ui_path());
    println!("   Spec URL: {}", server.spec_url());

    // In a real application, you would serve these as HTTP endpoints:
    println!("\n4. How to integrate with a web server:");
    println!("   GET {} -> serve ui_html()", server.ui_path());
    println!("   GET {} -> serve spec_json()", server.spec_url());

    #[cfg(feature = "yaml")]
    {
        println!("\n5. OpenAPI Specification (YAML - first 500 chars):");
        let spec_yaml = server.spec_yaml().unwrap();
        println!("{}...\n", &spec_yaml[..500.min(spec_yaml.len())]);
    }

    println!("=== Example Complete ===");
    println!("\nIn a real web server, you would:");
    println!("1. Serve the Swagger UI HTML at {}", server.ui_path());
    println!("2. Serve the OpenAPI spec at {}", server.spec_url());
    println!("3. Visit {} in your browser to see interactive docs", server.ui_path());
}

fn create_sample_spec() -> OpenApiSpec {
    // Create some sample operations
    let list_pets_op = OperationBuilder::new()
        .tag("pets")
        .summary("List all pets")
        .operation_id("listPets")
        .parameter(Parameter {
            name: "limit".to_string(),
            location: ParameterLocation::Query,
            description: Some("Maximum number of items to return".to_string()),
            required: Some(false),
            deprecated: None,
            schema: Some(builder::schemas::integer()),
        })
        .response("200", Response {
            description: "A list of pets".to_string(),
            content: HashMap::new(),
            headers: HashMap::new(),
        })
        .build();

    let get_pet_op = OperationBuilder::new()
        .tag("pets")
        .summary("Get a pet by ID")
        .operation_id("getPetById")
        .parameter(Parameter {
            name: "petId".to_string(),
            location: ParameterLocation::Path,
            description: Some("The ID of the pet to retrieve".to_string()),
            required: Some(true),
            deprecated: None,
            schema: Some(builder::schemas::integer()),
        })
        .response("200", Response {
            description: "Expected response to a valid request".to_string(),
            content: HashMap::new(),
            headers: HashMap::new(),
        })
        .response("404", Response {
            description: "Pet not found".to_string(),
            content: HashMap::new(),
            headers: HashMap::new(),
        })
        .build();

    let pets_path = PathItemBuilder::new()
        .get(list_pets_op)
        .build();

    let pet_by_id_path = PathItemBuilder::new()
        .get(get_pet_op)
        .build();

    // Build the complete spec
    OpenApiBuilder::new()
        .title("Pet Store API")
        .version("1.0.0")
        .description("A sample API that uses a pet store as an example to demonstrate features in the OpenAPI specification")
        .server("https://petstore.example.com/v1", Some("Production server".to_string()))
        .server("http://localhost:8080/v1", Some("Development server".to_string()))
        .tag("pets", Some("Everything about your Pets".to_string()))
        .license("MIT", Some("https://opensource.org/licenses/MIT".to_string()))
        .contact(
            "API Support",
            Some("support@petstore.example.com".to_string()),
            Some("https://www.petstore.example.com/support".to_string())
        )
        .path("/pets", pets_path)
        .path("/pets/{petId}", pet_by_id_path)
        .build()
}
