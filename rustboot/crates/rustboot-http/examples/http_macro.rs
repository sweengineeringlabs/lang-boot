//! HTTP Macro Example
//!
//! This example demonstrates using the `#[http_request]` macro to define
//! declarative API clients.
//!
//! Run with: cargo run --example http_macro --features reqwest

// Import all required types for the macro to work
use dev_engineeringlabs_rustboot_http::{
    HttpClient, HttpError, HttpResult, Method, Request, ReqwestClient,
};
use rustboot_macros::http_request;
use serde::{Deserialize, Serialize};

// API response types
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct HttpBinGet {
    args: std::collections::HashMap<String, String>,
    headers: std::collections::HashMap<String, String>,
    origin: String,
    url: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct HttpBinPost {
    data: String,
    json: Option<serde_json::Value>,
    url: String,
}

#[derive(Debug, Serialize)]
struct CreateTodo {
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct Todo {
    #[allow(dead_code)]
    id: Option<u64>,
    title: String,
    completed: bool,
}

/// Example API client using the http_request macro
struct HttpBinApi {
    base_url: String,
    client: ReqwestClient,
}

impl HttpBinApi {
    fn new() -> Self {
        Self {
            base_url: "https://httpbin.org".to_string(),
            client: ReqwestClient::new(),
        }
    }

    /// GET request with no parameters
    #[http_request(method = "GET", path = "/get")]
    async fn get_simple(&self) -> HttpResult<HttpBinGet> {}

    /// GET request with path parameter
    #[http_request(method = "GET", path = "/status/{code}")]
    async fn get_status(&self, code: u16) -> HttpResult<()> {}

    /// POST request with JSON body
    #[http_request(method = "POST", path = "/post", body)]
    async fn post_json(&self, data: CreateTodo) -> HttpResult<HttpBinPost> {}

    /// PUT request with path parameter and body
    #[http_request(method = "PUT", path = "/put", body)]
    async fn put_data(&self, data: CreateTodo) -> HttpResult<HttpBinPost> {}

    /// DELETE request with path parameter
    #[http_request(method = "DELETE", path = "/delete")]
    async fn delete_resource(&self) -> HttpResult<HttpBinGet> {}
}

/// Another example: JSONPlaceholder API client
struct JsonPlaceholderApi {
    base_url: String,
    client: ReqwestClient,
}

impl JsonPlaceholderApi {
    fn new() -> Self {
        Self {
            base_url: "https://jsonplaceholder.typicode.com".to_string(),
            client: ReqwestClient::new(),
        }
    }

    /// Get all todos
    #[http_request(method = "GET", path = "/todos")]
    async fn list_todos(&self) -> HttpResult<Vec<Todo>> {}

    /// Get a single todo by ID
    #[http_request(method = "GET", path = "/todos/{id}")]
    async fn get_todo(&self, id: u64) -> HttpResult<Todo> {}

    /// Create a new todo
    #[http_request(method = "POST", path = "/todos", body)]
    async fn create_todo(&self, todo: CreateTodo) -> HttpResult<Todo> {}

    /// Update a todo
    #[http_request(method = "PUT", path = "/todos/{id}", body)]
    async fn update_todo(&self, id: u64, todo: CreateTodo) -> HttpResult<Todo> {}

    /// Delete a todo
    #[http_request(method = "DELETE", path = "/todos/{id}")]
    async fn delete_todo(&self, id: u64) -> HttpResult<serde_json::Value> {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== HTTP Macro Example ===\n");

    // Example 1: HttpBin API
    println!("1. HttpBin API - Simple GET:");
    let httpbin = HttpBinApi::new();
    let response = httpbin.get_simple().await?;
    println!("   Origin: {}", response.origin);
    println!("   URL: {}\n", response.url);

    // Example 2: POST with JSON body
    println!("2. HttpBin API - POST with JSON:");
    let todo = CreateTodo {
        title: "Learn Rust macros".to_string(),
        completed: false,
    };
    let response = httpbin.post_json(todo).await?;
    println!("   Posted to: {}", response.url);
    println!("   JSON received: {:?}\n", response.json);

    // Example 3: JSONPlaceholder API
    println!("3. JSONPlaceholder API - Get single todo:");
    let placeholder = JsonPlaceholderApi::new();
    let todo = placeholder.get_todo(1).await?;
    println!("   Title: {}", todo.title);
    println!("   Completed: {}\n", todo.completed);

    // Example 4: Create a new todo
    println!("4. JSONPlaceholder API - Create todo:");
    let new_todo = CreateTodo {
        title: "Write documentation".to_string(),
        completed: true,
    };
    let created = placeholder.create_todo(new_todo).await?;
    println!("   Created: {} (completed: {})\n", created.title, created.completed);

    // Example 5: List first 3 todos
    println!("5. JSONPlaceholder API - List todos (first 3):");
    let todos = placeholder.list_todos().await?;
    for todo in todos.iter().take(3) {
        println!("   - {} [{}]", todo.title, if todo.completed { "x" } else { " " });
    }
    println!();

    println!("=== Example Complete ===");
    Ok(())
}
