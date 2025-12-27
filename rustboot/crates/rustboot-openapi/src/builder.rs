//! OpenAPI specification builder.
//!
//! Provides a fluent API for constructing OpenAPI specifications.

use crate::spec::*;
use std::collections::HashMap;

/// Builder for creating OpenAPI specifications.
#[derive(Debug, Clone)]
pub struct OpenApiBuilder {
    spec: OpenApiSpec,
}

impl OpenApiBuilder {
    /// Create a new OpenAPI builder with default info.
    pub fn new() -> Self {
        Self {
            spec: OpenApiSpec::new(Info {
                title: "API".to_string(),
                description: None,
                terms_of_service: None,
                contact: None,
                license: None,
                version: "1.0.0".to_string(),
            }),
        }
    }

    /// Set the API title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.spec.info.title = title.into();
        self
    }

    /// Set the API version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.spec.info.version = version.into();
        self
    }

    /// Set the API description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.spec.info.description = Some(description.into());
        self
    }

    /// Set the terms of service URL.
    pub fn terms_of_service(mut self, url: impl Into<String>) -> Self {
        self.spec.info.terms_of_service = Some(url.into());
        self
    }

    /// Set contact information.
    pub fn contact(mut self, name: impl Into<String>, email: Option<String>, url: Option<String>) -> Self {
        self.spec.info.contact = Some(Contact {
            name: Some(name.into()),
            email,
            url,
        });
        self
    }

    /// Set license information.
    pub fn license(mut self, name: impl Into<String>, url: Option<String>) -> Self {
        self.spec.info.license = Some(License {
            name: name.into(),
            url,
        });
        self
    }

    /// Add a server.
    pub fn server(mut self, url: impl Into<String>, description: Option<String>) -> Self {
        self.spec.servers.push(Server {
            url: url.into(),
            description,
            variables: HashMap::new(),
        });
        self
    }

    /// Add a tag.
    pub fn tag(mut self, name: impl Into<String>, description: Option<String>) -> Self {
        self.spec.tags.push(Tag {
            name: name.into(),
            description,
            external_docs: None,
        });
        self
    }

    /// Add a path with operations.
    pub fn path(mut self, path: impl Into<String>, item: PathItem) -> Self {
        self.spec.paths.insert(path.into(), item);
        self
    }

    /// Add a schema to components.
    pub fn schema(mut self, name: impl Into<String>, schema: Schema) -> Self {
        let components = self.spec.components.get_or_insert_with(Components::default);
        components.schemas.insert(name.into(), schema);
        self
    }

    /// Add a security scheme.
    pub fn security_scheme(mut self, name: impl Into<String>, scheme: SecurityScheme) -> Self {
        let components = self.spec.components.get_or_insert_with(Components::default);
        components.security_schemes.insert(name.into(), scheme);
        self
    }

    /// Build the OpenAPI specification.
    pub fn build(self) -> OpenApiSpec {
        self.spec
    }
}

impl Default for OpenApiBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating path items.
#[derive(Debug, Clone, Default)]
pub struct PathItemBuilder {
    item: PathItem,
}

impl PathItemBuilder {
    /// Create a new path item builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set GET operation.
    pub fn get(mut self, operation: Operation) -> Self {
        self.item.get = Some(operation);
        self
    }

    /// Set POST operation.
    pub fn post(mut self, operation: Operation) -> Self {
        self.item.post = Some(operation);
        self
    }

    /// Set PUT operation.
    pub fn put(mut self, operation: Operation) -> Self {
        self.item.put = Some(operation);
        self
    }

    /// Set DELETE operation.
    pub fn delete(mut self, operation: Operation) -> Self {
        self.item.delete = Some(operation);
        self
    }

    /// Set PATCH operation.
    pub fn patch(mut self, operation: Operation) -> Self {
        self.item.patch = Some(operation);
        self
    }

    /// Set HEAD operation.
    pub fn head(mut self, operation: Operation) -> Self {
        self.item.head = Some(operation);
        self
    }

    /// Set OPTIONS operation.
    pub fn options(mut self, operation: Operation) -> Self {
        self.item.options = Some(operation);
        self
    }

    /// Add a parameter applicable to all operations.
    pub fn parameter(mut self, parameter: Parameter) -> Self {
        self.item.parameters.push(parameter);
        self
    }

    /// Build the path item.
    pub fn build(self) -> PathItem {
        self.item
    }
}

/// Builder for creating operations.
#[derive(Debug, Clone)]
pub struct OperationBuilder {
    operation: Operation,
}

impl OperationBuilder {
    /// Create a new operation builder.
    pub fn new() -> Self {
        Self {
            operation: Operation {
                tags: Vec::new(),
                summary: None,
                description: None,
                operation_id: None,
                parameters: Vec::new(),
                request_body: None,
                responses: HashMap::new(),
                security: Vec::new(),
                deprecated: None,
            },
        }
    }

    /// Add a tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.operation.tags.push(tag.into());
        self
    }

    /// Set the summary.
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.operation.summary = Some(summary.into());
        self
    }

    /// Set the description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.operation.description = Some(description.into());
        self
    }

    /// Set the operation ID.
    pub fn operation_id(mut self, id: impl Into<String>) -> Self {
        self.operation.operation_id = Some(id.into());
        self
    }

    /// Add a parameter.
    pub fn parameter(mut self, parameter: Parameter) -> Self {
        self.operation.parameters.push(parameter);
        self
    }

    /// Set the request body.
    pub fn request_body(mut self, body: RequestBody) -> Self {
        self.operation.request_body = Some(body);
        self
    }

    /// Add a response.
    pub fn response(mut self, status: impl Into<String>, response: Response) -> Self {
        self.operation.responses.insert(status.into(), response);
        self
    }

    /// Mark as deprecated.
    pub fn deprecated(mut self) -> Self {
        self.operation.deprecated = Some(true);
        self
    }

    /// Build the operation.
    pub fn build(self) -> Operation {
        self.operation
    }
}

impl Default for OperationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for creating common schemas.
pub mod schemas {
    use super::*;

    /// Create a string schema.
    pub fn string() -> Schema {
        Schema::Object(SchemaObject {
            schema_type: Some("string".to_string()),
            format: None,
            description: None,
            nullable: None,
            properties: HashMap::new(),
            required: Vec::new(),
            items: None,
            enum_values: Vec::new(),
            default: None,
            example: None,
            all_of: Vec::new(),
            one_of: Vec::new(),
            any_of: Vec::new(),
        })
    }

    /// Create an integer schema.
    pub fn integer() -> Schema {
        Schema::Object(SchemaObject {
            schema_type: Some("integer".to_string()),
            format: Some("int64".to_string()),
            description: None,
            nullable: None,
            properties: HashMap::new(),
            required: Vec::new(),
            items: None,
            enum_values: Vec::new(),
            default: None,
            example: None,
            all_of: Vec::new(),
            one_of: Vec::new(),
            any_of: Vec::new(),
        })
    }

    /// Create a number schema.
    pub fn number() -> Schema {
        Schema::Object(SchemaObject {
            schema_type: Some("number".to_string()),
            format: Some("double".to_string()),
            description: None,
            nullable: None,
            properties: HashMap::new(),
            required: Vec::new(),
            items: None,
            enum_values: Vec::new(),
            default: None,
            example: None,
            all_of: Vec::new(),
            one_of: Vec::new(),
            any_of: Vec::new(),
        })
    }

    /// Create a boolean schema.
    pub fn boolean() -> Schema {
        Schema::Object(SchemaObject {
            schema_type: Some("boolean".to_string()),
            format: None,
            description: None,
            nullable: None,
            properties: HashMap::new(),
            required: Vec::new(),
            items: None,
            enum_values: Vec::new(),
            default: None,
            example: None,
            all_of: Vec::new(),
            one_of: Vec::new(),
            any_of: Vec::new(),
        })
    }

    /// Create an array schema.
    pub fn array(items: Schema) -> Schema {
        Schema::Object(SchemaObject {
            schema_type: Some("array".to_string()),
            format: None,
            description: None,
            nullable: None,
            properties: HashMap::new(),
            required: Vec::new(),
            items: Some(Box::new(items)),
            enum_values: Vec::new(),
            default: None,
            example: None,
            all_of: Vec::new(),
            one_of: Vec::new(),
            any_of: Vec::new(),
        })
    }

    /// Create an object schema.
    pub fn object() -> Schema {
        Schema::Object(SchemaObject {
            schema_type: Some("object".to_string()),
            format: None,
            description: None,
            nullable: None,
            properties: HashMap::new(),
            required: Vec::new(),
            items: None,
            enum_values: Vec::new(),
            default: None,
            example: None,
            all_of: Vec::new(),
            one_of: Vec::new(),
            any_of: Vec::new(),
        })
    }

    /// Create a reference schema.
    pub fn reference(name: impl Into<String>) -> Schema {
        Schema::Ref {
            reference: format!("#/components/schemas/{}", name.into()),
        }
    }
}
