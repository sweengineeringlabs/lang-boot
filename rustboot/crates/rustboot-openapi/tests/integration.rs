//! Integration tests for OpenAPI generation.

use dev_engineeringlabs_rustboot_openapi::*;

#[test]
fn test_openapi_builder() {
    let spec = OpenApiBuilder::new()
        .title("Test API")
        .version("1.0.0")
        .description("A test API specification")
        .server("https://api.example.com", Some("Production server".to_string()))
        .tag("users", Some("User management endpoints".to_string()))
        .build();

    assert_eq!(spec.info.title, "Test API");
    assert_eq!(spec.info.version, "1.0.0");
    assert_eq!(spec.info.description, Some("A test API specification".to_string()));
    assert_eq!(spec.servers.len(), 1);
    assert_eq!(spec.servers[0].url, "https://api.example.com");
    assert_eq!(spec.tags.len(), 1);
    assert_eq!(spec.tags[0].name, "users");
}

#[test]
fn test_json_generation() {
    let spec = OpenApiBuilder::new()
        .title("API")
        .version("1.0.0")
        .build();

    let json = spec.to_json().unwrap();
    assert!(json.contains("\"openapi\""));
    assert!(json.contains("\"info\""));
    assert!(json.contains("\"title\": \"API\""));
    assert!(json.contains("\"version\": \"1.0.0\""));
}

#[cfg(feature = "yaml")]
#[test]
fn test_yaml_generation() {
    let spec = OpenApiBuilder::new()
        .title("API")
        .version("1.0.0")
        .build();

    let yaml = spec.to_yaml().unwrap();
    assert!(yaml.contains("openapi:"));
    assert!(yaml.contains("info:"));
    assert!(yaml.contains("title: API"));
    assert!(yaml.contains("version: 1.0.0"));
}

#[test]
fn test_path_builder() {
    use dev_engineeringlabs_rustboot_openapi::builder::{PathItemBuilder, OperationBuilder};
    use dev_engineeringlabs_rustboot_openapi::spec::Response;
    use std::collections::HashMap;

    let operation = OperationBuilder::new()
        .tag("users")
        .summary("Get user by ID")
        .operation_id("getUserById")
        .response("200", Response {
            description: "User found".to_string(),
            content: HashMap::new(),
            headers: HashMap::new(),
        })
        .build();

    let path_item = PathItemBuilder::new()
        .get(operation)
        .build();

    assert!(path_item.get.is_some());
    let get_op = path_item.get.unwrap();
    assert_eq!(get_op.tags, vec!["users"]);
    assert_eq!(get_op.summary, Some("Get user by ID".to_string()));
    assert_eq!(get_op.operation_id, Some("getUserById".to_string()));
}

#[test]
fn test_schema_primitives() {
    let string_schema = <String as SchemaGenerator>::schema();
    if let Schema::Object(obj) = string_schema {
        assert_eq!(obj.schema_type, Some("string".to_string()));
    } else {
        panic!("Expected object schema");
    }

    let int_schema = <i64 as SchemaGenerator>::schema();
    if let Schema::Object(obj) = int_schema {
        assert_eq!(obj.schema_type, Some("integer".to_string()));
        assert_eq!(obj.format, Some("int64".to_string()));
    } else {
        panic!("Expected object schema");
    }

    let bool_schema = <bool as SchemaGenerator>::schema();
    if let Schema::Object(obj) = bool_schema {
        assert_eq!(obj.schema_type, Some("boolean".to_string()));
    } else {
        panic!("Expected object schema");
    }
}

#[test]
fn test_schema_array() {
    let array_schema = <Vec<String> as SchemaGenerator>::schema();
    if let Schema::Object(obj) = array_schema {
        assert_eq!(obj.schema_type, Some("array".to_string()));
        assert!(obj.items.is_some());
    } else {
        panic!("Expected object schema");
    }
}

#[test]
fn test_schema_option() {
    let optional_schema = <Option<String> as SchemaGenerator>::schema();
    if let Schema::Object(obj) = optional_schema {
        assert_eq!(obj.nullable, Some(true));
    } else {
        panic!("Expected object schema");
    }
}

#[test]
fn test_security_scheme_api_key() {
    use dev_engineeringlabs_rustboot_openapi::spec::{SecurityScheme, ParameterLocation};

    let scheme = SecurityScheme::ApiKey {
        location: ParameterLocation::Header,
        name: "X-API-Key".to_string(),
        description: Some("API key authentication".to_string()),
    };

    let spec = OpenApiBuilder::new()
        .title("API")
        .version("1.0.0")
        .security_scheme("api_key", scheme)
        .build();

    assert!(spec.components.is_some());
    let components = spec.components.unwrap();
    assert!(components.security_schemes.contains_key("api_key"));
}

#[test]
fn test_complete_spec() {
    use dev_engineeringlabs_rustboot_openapi::builder::{PathItemBuilder, OperationBuilder};
    use dev_engineeringlabs_rustboot_openapi::spec::{Response, Parameter, ParameterLocation};
    use std::collections::HashMap;

    let get_user_op = OperationBuilder::new()
        .tag("users")
        .summary("Get user by ID")
        .operation_id("getUserById")
        .parameter(Parameter {
            name: "id".to_string(),
            location: ParameterLocation::Path,
            description: Some("User ID".to_string()),
            required: Some(true),
            deprecated: None,
            schema: Some(<u64 as SchemaGenerator>::schema()),
        })
        .response("200", Response {
            description: "User found".to_string(),
            content: HashMap::new(),
            headers: HashMap::new(),
        })
        .response("404", Response {
            description: "User not found".to_string(),
            content: HashMap::new(),
            headers: HashMap::new(),
        })
        .build();

    let path_item = PathItemBuilder::new()
        .get(get_user_op)
        .build();

    let spec = OpenApiBuilder::new()
        .title("User API")
        .version("1.0.0")
        .description("API for managing users")
        .server("https://api.example.com", Some("Production".to_string()))
        .tag("users", Some("User operations".to_string()))
        .path("/users/{id}", path_item)
        .build();

    assert_eq!(spec.paths.len(), 1);
    assert!(spec.paths.contains_key("/users/{id}"));

    let json = spec.to_json().unwrap();
    assert!(json.contains("User API"));
    assert!(json.contains("/users/{id}"));
}

#[test]
#[cfg(feature = "swagger-ui")]
fn test_swagger_ui_html_generation() {
    use dev_engineeringlabs_rustboot_openapi::swagger_ui::{SwaggerUiConfig, generate_swagger_ui_html};

    let config = SwaggerUiConfig::new("/swagger-ui", "/api-docs/openapi.json")
        .title("My API Documentation");

    let html = generate_swagger_ui_html(&config);

    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("My API Documentation"));
    assert!(html.contains("/api-docs/openapi.json"));
    assert!(html.contains("swagger-ui-bundle.js"));
}

#[test]
#[cfg(feature = "swagger-ui")]
fn test_swagger_ui_server() {
    use dev_engineeringlabs_rustboot_openapi::swagger_ui::{SwaggerUiConfig, SwaggerUiServer};

    let spec = OpenApiBuilder::new()
        .title("Test API")
        .version("1.0.0")
        .build();

    let config = SwaggerUiConfig::default();
    let server = SwaggerUiServer::new(spec, config);

    let json = server.spec_json().unwrap();
    assert!(json.contains("Test API"));

    let html = server.ui_html();
    assert!(html.contains("<!DOCTYPE html>"));

    assert_eq!(server.ui_path(), "/swagger-ui");
    assert_eq!(server.spec_url(), "/api-docs/openapi.json");
}
