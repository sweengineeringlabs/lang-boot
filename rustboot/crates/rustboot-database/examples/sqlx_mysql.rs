//! SQLx MySQL Example
//!
//! This example demonstrates how to use the SQLx driver with MySQL.
//!
//! Prerequisites:
//! - MySQL server running
//! - Database created
//!
//! Set the DATABASE_URL environment variable:
//! export DATABASE_URL="mysql://user:password@localhost/testdb"
//!
//! Run with: cargo run --example sqlx_mysql --features sqlx-mysql

use dev_engineeringlabs_rustboot_database::{Database, SqlxDatabase, SqlxMutTransaction, Value};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rustboot Database - SQLx MySQL Example ===\n");

    // Get database URL from environment
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:password@localhost/testdb".to_string());

    println!("1. Connecting to MySQL database...");
    println!("   URL: {}", mask_password(&database_url));

    let db = match SqlxDatabase::connect_mysql(&database_url).await {
        Ok(db) => {
            println!("   Connected successfully!\n");
            db
        }
        Err(e) => {
            eprintln!("   Failed to connect: {}", e);
            eprintln!("\n   Please ensure:");
            eprintln!("   - MySQL is running");
            eprintln!("   - Database exists");
            eprintln!("   - DATABASE_URL is set correctly");
            return Err(e.into());
        }
    };

    // Drop table if exists and create new one
    println!("2. Setting up orders table...");
    let _ = db.execute("DROP TABLE IF EXISTS orders").await;
    db.execute(
        "CREATE TABLE orders (
            id INT AUTO_INCREMENT PRIMARY KEY,
            customer_name VARCHAR(255) NOT NULL,
            product VARCHAR(255) NOT NULL,
            quantity INT NOT NULL,
            total_price DECIMAL(10, 2) NOT NULL,
            status VARCHAR(50) DEFAULT 'pending',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .await?;
    println!("   Table created!\n");

    // Insert orders
    println!("3. Inserting orders...");
    db.execute(
        "INSERT INTO orders (customer_name, product, quantity, total_price, status)
         VALUES ('John Doe', 'Laptop', 1, 999.99, 'pending')",
    )
    .await?;

    db.execute(
        "INSERT INTO orders (customer_name, product, quantity, total_price, status)
         VALUES ('Jane Smith', 'Mouse', 3, 89.97, 'shipped')",
    )
    .await?;

    db.execute(
        "INSERT INTO orders (customer_name, product, quantity, total_price, status)
         VALUES ('Bob Johnson', 'Keyboard', 2, 159.98, 'pending')",
    )
    .await?;
    println!("   Inserted 3 orders\n");

    // Query all orders
    println!("4. Querying all orders...");
    let rows = db
        .query("SELECT id, customer_name, product, quantity, total_price, status FROM orders ORDER BY id")
        .await?;
    println!("   Found {} orders:", rows.len());
    for row in &rows {
        let id = match row.get("id") {
            Some(Value::Int(v)) => *v,
            _ => 0,
        };
        let customer = match row.get("customer_name") {
            Some(Value::String(v)) => v.as_str(),
            _ => "unknown",
        };
        let product = match row.get("product") {
            Some(Value::String(v)) => v.as_str(),
            _ => "unknown",
        };
        let quantity = match row.get("quantity") {
            Some(Value::Int(v)) => *v,
            _ => 0,
        };
        let price = match row.get("total_price") {
            Some(Value::Float(v)) => *v,
            Some(Value::String(v)) => v.parse().unwrap_or(0.0),
            _ => 0.0,
        };
        let status = match row.get("status") {
            Some(Value::String(v)) => v.as_str(),
            _ => "unknown",
        };
        println!(
            "   - Order #{}: {} ordered {} x {} (${:.2}) - Status: {}",
            id, customer, quantity, product, price, status
        );
    }
    println!();

    // Transaction example: Process an order
    println!("5. Processing order with transaction...");
    {
        let mut tx = SqlxMutTransaction::begin(db.pool()).await?;
        println!("   Transaction started");

        // Update order status
        tx.execute("UPDATE orders SET status = 'processing' WHERE id = 1")
            .await?;
        println!("   Updated order status to 'processing'");

        // Simulate inventory update (in a real app, this would update inventory table)
        println!("   Would update inventory here...");

        tx.commit().await?;
        println!("   Transaction committed\n");
    }

    // Query pending orders
    println!("6. Querying pending orders...");
    let rows = db
        .query("SELECT customer_name, product FROM orders WHERE status = 'pending'")
        .await?;
    println!("   Pending orders: {}", rows.len());
    for row in &rows {
        let customer = match row.get("customer_name") {
            Some(Value::String(v)) => v.as_str(),
            _ => "unknown",
        };
        let product = match row.get("product") {
            Some(Value::String(v)) => v.as_str(),
            _ => "unknown",
        };
        println!("   - {}: {}", customer, product);
    }
    println!();

    // Aggregation queries
    println!("7. Order statistics...");
    let rows = db
        .query(
            "SELECT
                COUNT(*) as total_orders,
                SUM(quantity) as total_items,
                SUM(total_price) as total_revenue,
                AVG(total_price) as avg_order_value
             FROM orders",
        )
        .await?;

    if let Some(row) = rows.first() {
        let total_orders = match row.get("total_orders") {
            Some(Value::Int(v)) => *v,
            _ => 0,
        };
        let total_items = match row.get("total_items") {
            Some(Value::Int(v)) => *v,
            _ => 0,
        };
        let total_revenue = match row.get("total_revenue") {
            Some(Value::Float(v)) => *v,
            Some(Value::String(v)) => v.parse().unwrap_or(0.0),
            _ => 0.0,
        };
        let avg_value = match row.get("avg_order_value") {
            Some(Value::Float(v)) => *v,
            Some(Value::String(v)) => v.parse().unwrap_or(0.0),
            _ => 0.0,
        };

        println!("   Total orders: {}", total_orders);
        println!("   Total items sold: {}", total_items);
        println!("   Total revenue: ${:.2}", total_revenue);
        println!("   Average order value: ${:.2}", avg_value);
    }
    println!();

    // Group by status
    println!("8. Orders by status...");
    let rows = db
        .query("SELECT status, COUNT(*) as count FROM orders GROUP BY status")
        .await?;
    for row in &rows {
        let status = match row.get("status") {
            Some(Value::String(v)) => v.as_str(),
            _ => "unknown",
        };
        let count = match row.get("count") {
            Some(Value::Int(v)) => *v,
            _ => 0,
        };
        println!("   - {}: {} orders", status, count);
    }
    println!();

    // Cleanup
    println!("9. Cleaning up...");
    db.execute("DROP TABLE orders").await?;
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
