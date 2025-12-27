//! Utoipa Integration Example
//!
//! This example demonstrates using utoipa for automatic OpenAPI generation
//! and converting between utoipa and rustboot-openapi formats.
//!
//! Run with: cargo run --example utoipa_integration --features utoipa

#[cfg(feature = "utoipa")]
fn main() {
    use dev_engineeringlabs_rustboot_openapi::*;
    use serde::{Deserialize, Serialize};
    use utoipa::{OpenApi, ToSchema};

    println!("=== Utoipa Integration Example ===\n");

    // Define types with utoipa's ToSchema derive
    #[derive(Serialize, Deserialize, ToSchema)]
    struct Pet {
        id: i64,
        name: String,
        tag: Option<String>,
    }

    #[derive(Serialize, Deserialize, ToSchema)]
    struct NewPet {
        name: String,
        tag: Option<String>,
    }

    #[derive(Serialize, Deserialize, ToSchema)]
    struct Error {
        code: i32,
        message: String,
    }

    // Define the API using utoipa
    #[derive(OpenApi)]
    #[openapi(
        paths(),
        components(schemas(Pet, NewPet, Error)),
        tags(
            (name = "pets", description = "Pet management endpoints")
        ),
        info(
            title = "Pet Store API",
            version = "1.0.0",
            description = "A sample API demonstrating utoipa integration"
        )
    )]
    struct ApiDoc;

    // Generate utoipa OpenAPI spec
    let utoipa_spec = ApiDoc::openapi();

    println!("1. Utoipa OpenAPI Spec:");
    let utoipa_json = serde_json::to_string_pretty(&utoipa_spec).unwrap();
    println!("{}\n", utoipa_json);

    // Convert to rustboot-openapi format
    println!("2. Converting to rustboot-openapi format:");
    let rustboot_spec = utoipa_support::from_utoipa(&utoipa_spec).unwrap();
    println!("   ✓ Conversion successful");
    println!("   Title: {}", rustboot_spec.info.title);
    println!("   Version: {}", rustboot_spec.info.version);

    if let Some(components) = &rustboot_spec.components {
        println!("   Schemas: {}", components.schemas.len());
        for schema_name in components.schemas.keys() {
            println!("     - {}", schema_name);
        }
    }
    println!();

    // Convert back to utoipa format
    println!("3. Converting back to utoipa format:");
    let converted_back = utoipa_support::to_utoipa(&rustboot_spec).unwrap();
    println!("   ✓ Round-trip conversion successful");
    let converted_json = serde_json::to_string_pretty(&converted_back).unwrap();
    println!("   Generated {} bytes of JSON\n", converted_json.len());

    // Use with Swagger UI
    println!("4. Integrating with Swagger UI:");
    use swagger_ui::{SwaggerUiConfig, SwaggerUiServer};

    let config = SwaggerUiConfig::new("/swagger-ui", "/api-docs/openapi.json")
        .title("Pet Store API - Interactive Docs");

    let server = SwaggerUiServer::new(rustboot_spec, config);
    println!("   ✓ Swagger UI server created");
    println!("   UI path: {}", server.ui_path());
    println!("   Spec URL: {}", server.spec_url());

    let html = server.ui_html();
    println!("   Generated {} bytes of HTML\n", html.len());

    println!("=== Example Complete ===");
    println!("\nKey Features Demonstrated:");
    println!("✓ Utoipa schema generation with ToSchema");
    println!("✓ OpenAPI specification with utoipa macros");
    println!("✓ Conversion from utoipa to rustboot-openapi");
    println!("✓ Round-trip format conversion");
    println!("✓ Swagger UI integration");
}

#[cfg(not(feature = "utoipa"))]
fn main() {
    println!("This example requires the 'utoipa' feature.");
    println!("Run with: cargo run --example utoipa_integration --features utoipa");
}
