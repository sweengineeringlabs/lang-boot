//! BB8 connection pooling example.
//!
//! This example demonstrates using the bb8 connection pool implementation.
//! To run this example, use:
//!   cargo run --example pool_bb8 --features pool-bb8 --no-default-features
//!
//! This example demonstrates:
//! - Creating a bb8-based connection pool
//! - Configuring bb8-specific options
//! - Using bb8 pool with the generic ConnectionPool trait

#[cfg(feature = "pool-bb8")]
use dev_engineeringlabs_rustboot_database::pool::prelude::*;
#[cfg(feature = "pool-bb8")]
use dev_engineeringlabs_rustboot_database::pool::{Bb8ConnectionPool, ExampleBb8Manager};
#[cfg(feature = "pool-bb8")]
use dev_engineeringlabs_rustboot_database::Database;
#[cfg(feature = "pool-bb8")]
use std::time::Duration;

#[cfg(feature = "pool-bb8")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rustboot Database Pool - BB8 Example ===\n");

    // 1. Create pool configuration
    let config = PoolConfig::new("postgres://user:pass@localhost:5432/mydb")
        .with_max_size(15)
        .with_connection_timeout(Duration::from_secs(30))
        .with_max_lifetime(Duration::from_secs(30 * 60))
        .with_idle_timeout(Duration::from_secs(10 * 60))
        .with_test_on_acquire(true);

    println!("BB8 Pool Configuration:");
    println!("  Max Size: {}", config.max_size);
    println!("  Connection Timeout: {:?}", config.connection_timeout);
    println!("  Max Lifetime: {:?}", config.max_lifetime);
    println!("  Idle Timeout: {:?}", config.idle_timeout);
    println!("  Test on Acquire: {}\n", config.test_on_acquire);

    // 2. Create the BB8 connection pool
    let manager = ExampleBb8Manager::new(config.connection_string.clone());
    let pool = Bb8ConnectionPool::new(manager, config).await?;

    println!("BB8 Pool created successfully!");
    println!("Max pool size: {}\n", pool.max_size());

    // 3. Check initial pool status
    let status = pool.status();
    print_pool_status(&status);

    // 4. Get a connection from the pool
    println!("\nGetting connection from pool...");
    let conn = pool.get().await?;
    println!("Connection acquired!");

    // 5. Use the connection
    println!("\nExecuting query...");
    let _rows = conn.query("SELECT * FROM users").await?;
    println!("Query executed successfully!");

    // 6. Connection is automatically returned to pool when dropped
    drop(conn);
    println!("\nConnection returned to pool");

    let status = pool.status();
    print_pool_status(&status);

    // 7. Demonstrate builder pattern
    println!("\nCreating pool with builder pattern...");
    let manager2 = ExampleBb8Manager::new("postgres://localhost/db2".to_string());
    let pool2 = Bb8ConnectionPool::builder(manager2)
        .max_size(20)
        .timeout(Duration::from_secs(15))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .build()
        .await?;

    println!("Second pool created!");
    println!("  Max size: {}", pool2.max_size());
    println!("  Timeout: {:?}", pool2.timeout());

    // 8. Test connection with custom timeout
    println!("\nTesting connection with custom timeout...");
    let timeout_conn = pool2.get_timeout(Duration::from_secs(5)).await?;
    println!("Connection acquired with custom timeout!");
    drop(timeout_conn);

    // 9. Get multiple connections
    println!("\nGetting multiple connections...");
    let mut connections = Vec::new();
    for i in 0..3 {
        let conn = pool.get().await?;
        println!("  Connection {} acquired", i + 1);
        connections.push(conn);
    }

    let status = pool.status();
    print_pool_status(&status);

    println!("\n=== Example Complete ===");
    Ok(())
}

#[cfg(feature = "pool-bb8")]
fn print_pool_status(status: &PoolStatus) {
    println!("\nPool Status:");
    println!("  Total Connections: {}", status.size);
    println!("  Idle Connections: {}", status.idle);
    println!("  Active Connections: {}", status.active);
    println!("  Max Size: {}", status.max_size);
    println!("  Utilization: {:.1}%", status.utilization() * 100.0);
}

#[cfg(not(feature = "pool-bb8"))]
fn main() {
    eprintln!("This example requires the 'pool-bb8' feature.");
    eprintln!("Run with: cargo run --example pool_bb8 --features pool-bb8 --no-default-features");
    std::process::exit(1);
}
