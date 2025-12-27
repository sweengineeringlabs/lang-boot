//! SQLx PostgreSQL Example
//!
//! This example demonstrates how to use the SQLx driver with PostgreSQL.
//!
//! Prerequisites:
//! - PostgreSQL server running
//! - Database created
//!
//! Set the DATABASE_URL environment variable:
//! export DATABASE_URL="postgres://user:password@localhost/testdb"
//!
//! Run with: cargo run --example sqlx_postgres --features sqlx-postgres

use dev_engineeringlabs_rustboot_database::{Database, SqlxDatabase, SqlxMutTransaction, Value};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rustboot Database - SQLx PostgreSQL Example ===\n");

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/testdb".to_string());

    println!("1. Connecting to PostgreSQL database...");
    println!("   URL: {}", mask_password(&database_url));

    let db = match SqlxDatabase::connect_postgres(&database_url).await {
        Ok(db) => {
            println!("   Connected successfully!\n");
            db
        }
        Err(e) => {
            eprintln!("   Failed to connect: {}", e);
            eprintln!("\n   Please ensure:");
            eprintln!("   - PostgreSQL is running");
            eprintln!("   - Database exists");
            eprintln!("   - DATABASE_URL is set correctly");
            return Err(e.into());
        }
    };

    // Drop table if exists and create new one
    println!("2. Setting up products table...");
    let _ = db.execute("DROP TABLE IF EXISTS products").await;
    db.execute(
        "CREATE TABLE products (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            description TEXT,
            price DECIMAL(10, 2) NOT NULL,
            in_stock BOOLEAN DEFAULT true,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .await?;
    println!("   Table created!\n");

    // Insert products
    println!("3. Inserting products...");
    db.execute(
        "INSERT INTO products (name, description, price, in_stock)
         VALUES ('Laptop', 'High-performance laptop', 999.99, true)",
    )
    .await?;

    db.execute(
        "INSERT INTO products (name, description, price, in_stock)
         VALUES ('Mouse', 'Wireless mouse', 29.99, true)",
    )
    .await?;

    db.execute(
        "INSERT INTO products (name, description, price, in_stock)
         VALUES ('Keyboard', 'Mechanical keyboard', 79.99, false)",
    )
    .await?;
    println!("   Inserted 3 products\n");

    // Query all products
    println!("4. Querying all products...");
    let rows = db
        .query("SELECT id, name, price, in_stock FROM products ORDER BY id")
        .await?;
    println!("   Found {} products:", rows.len());
    for row in &rows {
        let id = match row.get("id") {
            Some(Value::Int(v)) => *v,
            _ => 0,
        };
        let name = match row.get("name") {
            Some(Value::String(v)) => v.as_str(),
            _ => "unknown",
        };
        let price = match row.get("price") {
            Some(Value::Float(v)) => *v,
            Some(Value::String(v)) => v.parse().unwrap_or(0.0),
            _ => 0.0,
        };
        let in_stock = match row.get("in_stock") {
            Some(Value::Bool(v)) => *v,
            _ => false,
        };
        println!(
            "   - ID: {}, Name: {}, Price: ${:.2}, In Stock: {}",
            id, name, price, in_stock
        );
    }
    println!();

    // Transaction example with multiple operations
    println!("5. Testing transaction with price update...");
    {
        let mut tx = SqlxMutTransaction::begin(db.pool()).await?;
        println!("   Transaction started");

        // Update prices
        tx.execute("UPDATE products SET price = price * 0.9 WHERE in_stock = true")
            .await?;
        println!("   Applied 10% discount to in-stock items");

        // Insert a new product
        tx.execute(
            "INSERT INTO products (name, description, price, in_stock)
             VALUES ('Monitor', '4K display', 399.99, true)",
        )
        .await?;
        println!("   Added new product");

        tx.commit().await?;
        println!("   Transaction committed\n");
    }

    // Query updated prices
    println!("6. Querying updated prices...");
    let rows = db
        .query("SELECT name, price FROM products WHERE in_stock = true ORDER BY name")
        .await?;
    for row in &rows {
        let name = match row.get("name") {
            Some(Value::String(v)) => v.as_str(),
            _ => "unknown",
        };
        let price = match row.get("price") {
            Some(Value::Float(v)) => *v,
            Some(Value::String(v)) => v.parse().unwrap_or(0.0),
            _ => 0.0,
        };
        println!("   - {}: ${:.2}", name, price);
    }
    println!();

    // Aggregation query
    println!("7. Product statistics...");
    let rows = db
        .query(
            "SELECT
                COUNT(*) as total_count,
                SUM(CASE WHEN in_stock THEN 1 ELSE 0 END) as in_stock_count,
                AVG(price) as avg_price,
                MAX(price) as max_price,
                MIN(price) as min_price
             FROM products",
        )
        .await?;

    if let Some(row) = rows.first() {
        let total = match row.get("total_count") {
            Some(Value::Int(v)) => *v,
            _ => 0,
        };
        let in_stock = match row.get("in_stock_count") {
            Some(Value::Int(v)) => *v,
            _ => 0,
        };
        let avg = match row.get("avg_price") {
            Some(Value::Float(v)) => *v,
            Some(Value::String(v)) => v.parse().unwrap_or(0.0),
            _ => 0.0,
        };
        let max = match row.get("max_price") {
            Some(Value::Float(v)) => *v,
            Some(Value::String(v)) => v.parse().unwrap_or(0.0),
            _ => 0.0,
        };
        let min = match row.get("min_price") {
            Some(Value::Float(v)) => *v,
            Some(Value::String(v)) => v.parse().unwrap_or(0.0),
            _ => 0.0,
        };

        println!("   Total products: {}", total);
        println!("   In stock: {}", in_stock);
        println!("   Average price: ${:.2}", avg);
        println!("   Max price: ${:.2}", max);
        println!("   Min price: ${:.2}", min);
    }
    println!();

    // Cleanup
    println!("8. Cleaning up...");
    db.execute("DROP TABLE products").await?;
    println!("   Table dropped");

    db.close().await;
    println!("   Connection closed\n");

    println!("=== Example completed successfully! ===");
    Ok(())
}

/// Mask password in database URL for display
fn mask_password(url: &str) -> String {
    if let Some(at_pos) = url.find('@') {
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            if let Some(scheme_end) = url.find("://") {
                let scheme = &url[..=scheme_end + 2];
                let user = &url[scheme_end + 3..colon_pos];
                let host = &url[at_pos..];
                return format!("{}{}:****{}", scheme, user, host);
            }
        }
    }
    url.to_string()
}
