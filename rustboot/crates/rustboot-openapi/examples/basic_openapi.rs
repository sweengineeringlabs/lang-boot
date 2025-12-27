//! Basic OpenAPI Example
//!
//! This example demonstrates building a basic OpenAPI specification manually.
//!
//! Run with: cargo run --example basic_openapi

use dev_engineeringlabs_rustboot_openapi::*;
use dev_engineeringlabs_rustboot_openapi::builder::{PathItemBuilder, OperationBuilder};
use dev_engineeringlabs_rustboot_openapi::spec::{Response, Parameter, ParameterLocation, RequestBody, MediaType};
use std::collections::HashMap;

fn main() {
    println!("=== Basic OpenAPI Example ===\n");

    // Build a complete API specification
    let spec = OpenApiBuilder::new()
        .title("Todo API")
        .version("1.0.0")
        .description("A simple Todo list API")
        .server("https://api.example.com", Some("Production server".to_string()))
        .server("http://localhost:8080", Some("Development server".to_string()))
        .tag("todos", Some("Todo list operations".to_string()))
        .license("MIT", Some("https://opensource.org/licenses/MIT".to_string()))
        .contact("API Support", Some("support@example.com".to_string()), Some("https://example.com".to_string()))
        .path("/todos", create_list_todos_path())
        .path("/todos/{id}", create_todo_by_id_path())
        .schema("Todo", create_todo_schema())
        .schema("CreateTodoRequest", create_create_todo_schema())
        .build();

    // Generate JSON
    println!("1. OpenAPI Specification (JSON):");
    let json = spec.to_json().unwrap();
    println!("{}\n", json);

    // Generate YAML (if feature enabled)
    #[cfg(feature = "yaml")]
    {
        println!("2. OpenAPI Specification (YAML):");
        let yaml = spec.to_yaml().unwrap();
        println!("{}\n", yaml);
    }

    println!("=== Example Complete ===");
}

fn create_list_todos_path() -> PathItem {
    // GET /todos - List all todos
    let get_todos = OperationBuilder::new()
        .tag("todos")
        .summary("List all todos")
        .operation_id("listTodos")
        .description("Returns a list of all todo items")
        .response("200", Response {
            description: "List of todos".to_string(),
            content: {
                let mut content = HashMap::new();
                content.insert("application/json".to_string(), MediaType {
                    schema: Some(builder::schemas::array(builder::schemas::reference("Todo"))),
                    example: None,
                    examples: HashMap::new(),
                });
                content
            },
            headers: HashMap::new(),
        })
        .build();

    // POST /todos - Create a new todo
    let create_todo = OperationBuilder::new()
        .tag("todos")
        .summary("Create a new todo")
        .operation_id("createTodo")
        .description("Creates a new todo item")
        .request_body(RequestBody {
            description: Some("Todo to create".to_string()),
            content: {
                let mut content = HashMap::new();
                content.insert("application/json".to_string(), MediaType {
                    schema: Some(builder::schemas::reference("CreateTodoRequest")),
                    example: Some(serde_json::json!({
                        "title": "Buy groceries",
                        "completed": false
                    })),
                    examples: HashMap::new(),
                });
                content
            },
            required: Some(true),
        })
        .response("201", Response {
            description: "Todo created".to_string(),
            content: {
                let mut content = HashMap::new();
                content.insert("application/json".to_string(), MediaType {
                    schema: Some(builder::schemas::reference("Todo")),
                    example: None,
                    examples: HashMap::new(),
                });
                content
            },
            headers: HashMap::new(),
        })
        .response("400", Response {
            description: "Invalid request".to_string(),
            content: HashMap::new(),
            headers: HashMap::new(),
        })
        .build();

    PathItemBuilder::new()
        .get(get_todos)
        .post(create_todo)
        .build()
}

fn create_todo_by_id_path() -> PathItem {
    // GET /todos/{id} - Get a specific todo
    let get_todo = OperationBuilder::new()
        .tag("todos")
        .summary("Get todo by ID")
        .operation_id("getTodoById")
        .parameter(Parameter {
            name: "id".to_string(),
            location: ParameterLocation::Path,
            description: Some("Todo ID".to_string()),
            required: Some(true),
            deprecated: None,
            schema: Some(builder::schemas::integer()),
        })
        .response("200", Response {
            description: "Todo found".to_string(),
            content: {
                let mut content = HashMap::new();
                content.insert("application/json".to_string(), MediaType {
                    schema: Some(builder::schemas::reference("Todo")),
                    example: None,
                    examples: HashMap::new(),
                });
                content
            },
            headers: HashMap::new(),
        })
        .response("404", Response {
            description: "Todo not found".to_string(),
            content: HashMap::new(),
            headers: HashMap::new(),
        })
        .build();

    // DELETE /todos/{id} - Delete a todo
    let delete_todo = OperationBuilder::new()
        .tag("todos")
        .summary("Delete todo")
        .operation_id("deleteTodo")
        .parameter(Parameter {
            name: "id".to_string(),
            location: ParameterLocation::Path,
            description: Some("Todo ID".to_string()),
            required: Some(true),
            deprecated: None,
            schema: Some(builder::schemas::integer()),
        })
        .response("204", Response {
            description: "Todo deleted".to_string(),
            content: HashMap::new(),
            headers: HashMap::new(),
        })
        .response("404", Response {
            description: "Todo not found".to_string(),
            content: HashMap::new(),
            headers: HashMap::new(),
        })
        .build();

    PathItemBuilder::new()
        .get(get_todo)
        .delete(delete_todo)
        .build()
}

fn create_todo_schema() -> Schema {
    use spec::SchemaObject;

    let mut properties = HashMap::new();
    properties.insert("id".to_string(), builder::schemas::integer());
    properties.insert("title".to_string(), builder::schemas::string());
    properties.insert("completed".to_string(), builder::schemas::boolean());

    Schema::Object(SchemaObject {
        schema_type: Some("object".to_string()),
        format: None,
        description: Some("A todo item".to_string()),
        nullable: None,
        properties,
        required: vec!["id".to_string(), "title".to_string(), "completed".to_string()],
        items: None,
        enum_values: Vec::new(),
        default: None,
        example: Some(serde_json::json!({
            "id": 1,
            "title": "Buy groceries",
            "completed": false
        })),
        all_of: Vec::new(),
        one_of: Vec::new(),
        any_of: Vec::new(),
    })
}

fn create_create_todo_schema() -> Schema {
    use spec::SchemaObject;

    let mut properties = HashMap::new();
    properties.insert("title".to_string(), builder::schemas::string());
    properties.insert("completed".to_string(), builder::schemas::boolean());

    Schema::Object(SchemaObject {
        schema_type: Some("object".to_string()),
        format: None,
        description: Some("Request body for creating a todo".to_string()),
        nullable: None,
        properties,
        required: vec!["title".to_string()],
        items: None,
        enum_values: Vec::new(),
        default: None,
        example: Some(serde_json::json!({
            "title": "Buy groceries",
            "completed": false
        })),
        all_of: Vec::new(),
        one_of: Vec::new(),
        any_of: Vec::new(),
    })
}
