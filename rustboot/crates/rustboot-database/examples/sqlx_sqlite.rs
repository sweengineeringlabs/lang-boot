//! SQLx SQLite Example
//!
//! This example demonstrates how to use the SQLx driver with SQLite.
//! Run with: cargo run --example sqlx_sqlite --features sqlx-sqlite

use dev_engineeringlabs_rustboot_database::{Database, SqlxDatabase, SqlxMutTransaction, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rustboot Database - SQLx SQLite Example ===\n");

    // Connect to in-memory SQLite database
    println!("1. Connecting to SQLite database...");
    let db = SqlxDatabase::connect_sqlite("sqlite::memory:").await?;
    println!("   Connected successfully!\n");

    // Create a users table
    println!("2. Creating users table...");
    db.execute(
        "CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            active BOOLEAN DEFAULT 1,
            age INTEGER
        )",
    )
    .await?;
    println!("   Table created!\n");

    // Insert some data
    println!("3. Inserting users...");
    let affected = db
        .execute("INSERT INTO users (name, email, age) VALUES ('Alice', 'alice@example.com', 30)")
        .await?;
    println!("   Inserted {} row(s)", affected);

    db.execute("INSERT INTO users (name, email, age) VALUES ('Bob', 'bob@example.com', 25)")
        .await?;
    db.execute("INSERT INTO users (name, email, age) VALUES ('Charlie', 'charlie@example.com', 35)")
        .await?;
    println!("   Total users inserted: 3\n");

    // Query all users
    println!("4. Querying all users...");
    let rows = db.query("SELECT id, name, email, age FROM users").await?;
    println!("   Found {} users:", rows.len());
    for row in &rows {
        let id = match row.get("id") {
            Some(Value::Int(v)) => *v,
            _ => 0,
        };
        let name = match row.get("name") {
            Some(Value::String(v)) => v.as_str(),
            _ => "unknown",
        };
        let email = match row.get("email") {
            Some(Value::String(v)) => v.as_str(),
            _ => "unknown",
        };
        let age = match row.get("age") {
            Some(Value::Int(v)) => *v,
            _ => 0,
        };
        println!("   - ID: {}, Name: {}, Email: {}, Age: {}", id, name, email, age);
    }
    println!();

    // Update a user
    println!("5. Updating user...");
    let affected = db
        .execute("UPDATE users SET age = 31 WHERE name = 'Alice'")
        .await?;
    println!("   Updated {} row(s)\n", affected);

    // Query updated user
    println!("6. Querying updated user...");
    let rows = db
        .query("SELECT name, age FROM users WHERE name = 'Alice'")
        .await?;
    if let Some(row) = rows.first() {
        let name = match row.get("name") {
            Some(Value::String(v)) => v.as_str(),
            _ => "unknown",
        };
        let age = match row.get("age") {
            Some(Value::Int(v)) => *v,
            _ => 0,
        };
        println!("   {}'s age is now {}\n", name, age);
    }

    // Transaction example - commit
    println!("7. Testing transaction (commit)...");
    {
        let mut tx = SqlxMutTransaction::begin(db.pool()).await?;
        println!("   Transaction started");

        tx.execute("INSERT INTO users (name, email, age) VALUES ('David', 'david@example.com', 28)")
            .await?;
        println!("   Inserted David in transaction");

        tx.commit().await?;
        println!("   Transaction committed\n");
    }

    // Verify transaction commit
    let rows = db
        .query("SELECT name FROM users WHERE name = 'David'")
        .await?;
    println!("   David exists: {}\n", !rows.is_empty());

    // Transaction example - rollback
    println!("8. Testing transaction (rollback)...");
    {
        let mut tx = SqlxMutTransaction::begin(db.pool()).await?;
        println!("   Transaction started");

        tx.execute("INSERT INTO users (name, email, age) VALUES ('Eve', 'eve@example.com', 22)")
            .await?;
        println!("   Inserted Eve in transaction");

        tx.rollback().await?;
        println!("   Transaction rolled back\n");
    }

    // Verify transaction rollback
    let rows = db
        .query("SELECT name FROM users WHERE name = 'Eve'")
        .await?;
    println!("   Eve exists: {}\n", rows.is_empty() == false);

    // Delete a user
    println!("9. Deleting user...");
    let affected = db
        .execute("DELETE FROM users WHERE name = 'Bob'")
        .await?;
    println!("   Deleted {} row(s)\n", affected);

    // Final count
    println!("10. Final user count...");
    let rows = db.query("SELECT COUNT(*) as count FROM users").await?;
    if let Some(row) = rows.first() {
        if let Some(Value::Int(count)) = row.get("count") {
            println!("   Total users: {}\n", count);
        }
    }

    // Complex query with aggregation
    println!("11. Average age of users...");
    let rows = db.query("SELECT AVG(age) as avg_age FROM users").await?;
    if let Some(row) = rows.first() {
        let avg_age = match row.get("avg_age") {
            Some(Value::Float(v)) => *v,
            Some(Value::Int(v)) => *v as f64,
            _ => 0.0,
        };
        println!("   Average age: {:.1}\n", avg_age);
    }

    // Close the connection
    println!("12. Closing database connection...");
    db.close().await;
    println!("   Connection closed successfully!");

    println!("\n=== Example completed successfully! ===");
    Ok(())
}
