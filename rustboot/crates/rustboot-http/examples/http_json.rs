//! JSON HTTP Client Example
//!
//! This example demonstrates JSON request/response handling.
//!
//! Run with: cargo run --example http_json --features reqwest

use dev_engineeringlabs_rustboot_http::{HttpClient, Method, Request, ReqwestClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct CreateUser {
    name: String,
    email: String,
    age: u32,
}

#[derive(Debug, Deserialize)]
struct HttpBinResponse {
    json: Option<serde_json::Value>,
    #[allow(dead_code)]
    data: Option<String>,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rustboot HTTP JSON Example ===\n");

    let client = ReqwestClient::new();

    // Example 1: POST JSON data
    println!("1. POST JSON data:");
    let user = CreateUser {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 30,
    };

    let request = Request::new(Method::Post, "https://httpbin.org/post".to_string())
        .json(&user)?;

    let response = client.send(request).await?;
    println!("   Status: {}", response.status);

    let body: HttpBinResponse = response.json()?;
    println!("   Echoed JSON: {:?}", body.json);
    println!("   URL: {}\n", body.url);

    // Example 2: Parse JSON response
    println!("2. Parse JSON response:");

    #[derive(Debug, Deserialize)]
    struct IpResponse {
        origin: String,
    }

    let response = client.get("https://httpbin.org/ip").await?;
    let ip_info: IpResponse = response.json()?;
    println!("   Your IP: {}\n", ip_info.origin);

    // Example 3: Handle JSON arrays
    println!("3. Working with JSON arrays:");

    #[derive(Debug, Serialize)]
    struct Item {
        id: u32,
        name: String,
    }

    let items = vec![
        Item { id: 1, name: "Apple".to_string() },
        Item { id: 2, name: "Banana".to_string() },
        Item { id: 3, name: "Cherry".to_string() },
    ];

    let request = Request::new(Method::Post, "https://httpbin.org/post".to_string())
        .json(&items)?;

    let response = client.send(request).await?;
    let body: HttpBinResponse = response.json()?;
    println!("   Sent {} items", items.len());
    println!("   Echoed: {:?}\n", body.json);

    // Example 4: Nested JSON structures
    println!("4. Nested JSON structures:");

    #[derive(Debug, Serialize)]
    struct Order {
        order_id: String,
        customer: Customer,
        items: Vec<OrderItem>,
    }

    #[derive(Debug, Serialize)]
    struct Customer {
        name: String,
        address: String,
    }

    #[derive(Debug, Serialize)]
    struct OrderItem {
        product: String,
        quantity: u32,
        price: f64,
    }

    let order = Order {
        order_id: "ORD-12345".to_string(),
        customer: Customer {
            name: "Jane Smith".to_string(),
            address: "123 Main St".to_string(),
        },
        items: vec![
            OrderItem { product: "Widget".to_string(), quantity: 2, price: 19.99 },
            OrderItem { product: "Gadget".to_string(), quantity: 1, price: 49.99 },
        ],
    };

    let request = Request::new(Method::Post, "https://httpbin.org/post".to_string())
        .json(&order)?;

    let response = client.send(request).await?;
    println!("   Status: {}", response.status);
    println!("   Order submitted successfully!\n");

    println!("=== Example Complete ===");
    Ok(())
}
