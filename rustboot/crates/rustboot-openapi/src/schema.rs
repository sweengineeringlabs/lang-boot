//! Schema generation utilities.
//!
//! Provides utilities for generating OpenAPI schemas from Rust types.

use crate::spec::{Schema, SchemaObject};
use std::collections::HashMap;

/// Schema type for basic type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaType {
    String,
    Integer,
    Number,
    Boolean,
    Array,
    Object,
}

impl SchemaType {
    /// Convert to OpenAPI type string.
    pub fn as_str(&self) -> &'static str {
        match self {
            SchemaType::String => "string",
            SchemaType::Integer => "integer",
            SchemaType::Number => "number",
            SchemaType::Boolean => "boolean",
            SchemaType::Array => "array",
            SchemaType::Object => "object",
        }
    }
}

/// Trait for types that can generate OpenAPI schemas.
pub trait SchemaGenerator {
    /// Generate the OpenAPI schema for this type.
    fn schema() -> Schema;

    /// Get the schema name (for component references).
    fn schema_name() -> Option<String> {
        None
    }
}

// Implementations for primitive types

impl SchemaGenerator for String {
    fn schema() -> Schema {
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
}

impl SchemaGenerator for &str {
    fn schema() -> Schema {
        String::schema()
    }
}

impl SchemaGenerator for i32 {
    fn schema() -> Schema {
        Schema::Object(SchemaObject {
            schema_type: Some("integer".to_string()),
            format: Some("int32".to_string()),
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
}

impl SchemaGenerator for i64 {
    fn schema() -> Schema {
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
}

impl SchemaGenerator for u32 {
    fn schema() -> Schema {
        Schema::Object(SchemaObject {
            schema_type: Some("integer".to_string()),
            format: Some("int32".to_string()),
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
}

impl SchemaGenerator for u64 {
    fn schema() -> Schema {
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
}

impl SchemaGenerator for f32 {
    fn schema() -> Schema {
        Schema::Object(SchemaObject {
            schema_type: Some("number".to_string()),
            format: Some("float".to_string()),
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
}

impl SchemaGenerator for f64 {
    fn schema() -> Schema {
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
}

impl SchemaGenerator for bool {
    fn schema() -> Schema {
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
}

impl<T: SchemaGenerator> SchemaGenerator for Vec<T> {
    fn schema() -> Schema {
        Schema::Object(SchemaObject {
            schema_type: Some("array".to_string()),
            format: None,
            description: None,
            nullable: None,
            properties: HashMap::new(),
            required: Vec::new(),
            items: Some(Box::new(T::schema())),
            enum_values: Vec::new(),
            default: None,
            example: None,
            all_of: Vec::new(),
            one_of: Vec::new(),
            any_of: Vec::new(),
        })
    }
}

impl<T: SchemaGenerator> SchemaGenerator for Option<T> {
    fn schema() -> Schema {
        let mut schema = T::schema();
        if let Schema::Object(ref mut obj) = schema {
            obj.nullable = Some(true);
        }
        schema
    }
}

impl<K: SchemaGenerator, V: SchemaGenerator> SchemaGenerator for HashMap<K, V> {
    fn schema() -> Schema {
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
}
