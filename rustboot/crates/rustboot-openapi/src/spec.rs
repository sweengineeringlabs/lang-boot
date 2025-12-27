//! OpenAPI specification data structures.
//!
//! This module defines the core OpenAPI 3.0 specification types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OpenAPI specification root object (OpenAPI 3.0).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    /// OpenAPI version (always "3.0.0" or "3.1.0")
    pub openapi: String,

    /// API metadata
    pub info: Info,

    /// Available servers for the API
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub servers: Vec<Server>,

    /// Available paths and operations
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub paths: HashMap<String, PathItem>,

    /// Reusable components (schemas, responses, parameters, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,

    /// Security requirements
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub security: Vec<HashMap<String, Vec<String>>>,

    /// Tags for grouping operations
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<Tag>,

    /// External documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
}

impl OpenApiSpec {
    /// Create a new OpenAPI specification with default version 3.0.3
    pub fn new(info: Info) -> Self {
        Self {
            openapi: "3.0.3".to_string(),
            info,
            servers: Vec::new(),
            paths: HashMap::new(),
            components: None,
            security: Vec::new(),
            tags: Vec::new(),
            external_docs: None,
        }
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> crate::Result<String> {
        serde_json::to_string_pretty(self).map_err(Into::into)
    }

    /// Convert to YAML string
    #[cfg(feature = "yaml")]
    pub fn to_yaml(&self) -> crate::Result<String> {
        serde_yaml::to_string(self).map_err(Into::into)
    }

    /// Add a path to the specification
    pub fn add_path(&mut self, path: String, item: PathItem) {
        self.paths.insert(path, item);
    }

    /// Add a schema to the components
    pub fn add_schema(&mut self, name: String, schema: Schema) {
        let components = self.components.get_or_insert_with(Components::default);
        components.schemas.insert(name, schema);
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }
}

/// API metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    /// API title
    pub title: String,

    /// API description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Terms of service URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,

    /// Contact information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    /// License information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,

    /// API version
    pub version: String,
}

/// Contact information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    /// Contact name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Contact URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Contact email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// License information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    /// License name
    pub name: String,

    /// License URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Server information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    /// Server URL
    pub url: String,

    /// Server description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Server variables
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub variables: HashMap<String, ServerVariable>,
}

/// Server variable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerVariable {
    /// Default value
    pub default: String,

    /// Possible values
    #[serde(skip_serializing_if = "Vec::is_empty", default, rename = "enum")]
    pub enum_values: Vec<String>,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Path item (set of operations for a path).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PathItem {
    /// GET operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,

    /// POST operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,

    /// PUT operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,

    /// DELETE operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,

    /// PATCH operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,

    /// HEAD operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,

    /// OPTIONS operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Parameters applicable to all operations
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub parameters: Vec<Parameter>,
}

/// Operation object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    /// Tags for grouping
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tags: Vec<String>,

    /// Operation summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Operation description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Operation ID (unique)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,

    /// Parameters
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub parameters: Vec<Parameter>,

    /// Request body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RequestBody>,

    /// Responses
    pub responses: HashMap<String, Response>,

    /// Security requirements
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub security: Vec<HashMap<String, Vec<String>>>,

    /// Deprecated flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}

/// Parameter object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    /// Parameter name
    pub name: String,

    /// Parameter location (query, header, path, cookie)
    #[serde(rename = "in")]
    pub location: ParameterLocation,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Required flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Deprecated flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

    /// Schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
}

/// Parameter location.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterLocation {
    Query,
    Header,
    Path,
    Cookie,
}

/// Request body object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Content (media type -> media type object)
    pub content: HashMap<String, MediaType>,

    /// Required flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// Media type object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaType {
    /// Schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,

    /// Example value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,

    /// Examples
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub examples: HashMap<String, Example>,
}

/// Example object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    /// Summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Example value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

/// Response object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// Description
    pub description: String,

    /// Content (media type -> media type object)
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub content: HashMap<String, MediaType>,

    /// Headers
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub headers: HashMap<String, Header>,
}

/// Header object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Required flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
}

/// Components object (reusable schemas, responses, etc.).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Components {
    /// Reusable schemas
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub schemas: HashMap<String, Schema>,

    /// Reusable responses
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub responses: HashMap<String, Response>,

    /// Reusable parameters
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub parameters: HashMap<String, Parameter>,

    /// Security schemes
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub security_schemes: HashMap<String, SecurityScheme>,
}

/// Security scheme object.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SecurityScheme {
    ApiKey {
        #[serde(rename = "in")]
        location: ParameterLocation,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    Http {
        scheme: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        bearer_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    OAuth2 {
        flows: OAuthFlows,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    OpenIdConnect {
        #[serde(rename = "openIdConnectUrl")]
        open_id_connect_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
}

/// OAuth flows.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OAuthFlows {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<OAuthFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<OAuthFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<OAuthFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<OAuthFlow>,
}

/// OAuth flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

/// Schema object (JSON Schema subset).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Schema {
    /// Reference to a component schema
    Ref {
        #[serde(rename = "$ref")]
        reference: String,
    },
    /// Inline schema
    Object(SchemaObject),
}

/// Schema object definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaObject {
    /// Schema type
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub schema_type: Option<String>,

    /// Format (e.g., "int32", "date-time")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Nullable flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,

    /// Properties (for object type)
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub properties: HashMap<String, Schema>,

    /// Required properties
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub required: Vec<String>,

    /// Items (for array type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,

    /// Enum values
    #[serde(skip_serializing_if = "Vec::is_empty", default, rename = "enum")]
    pub enum_values: Vec<serde_json::Value>,

    /// Default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    /// Example value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,

    /// AllOf (composition)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub all_of: Vec<Schema>,

    /// OneOf (composition)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub one_of: Vec<Schema>,

    /// AnyOf (composition)
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub any_of: Vec<Schema>,
}

/// Tag object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// Tag name
    pub name: String,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// External documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
}

/// External documentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentation {
    /// URL
    pub url: String,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
