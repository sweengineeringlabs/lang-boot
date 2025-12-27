//! Basic connection pooling example using deadpool.
//!
//! This example demonstrates:
//! - Creating a connection pool with configuration
//! - Getting connections from the pool
//! - Executing queries
//! - Pool status monitoring

use dev_engineeringlabs_rustboot_database::pool::prelude::*;
use dev_engineeringlabs_rustboot_database::pool::{DeadpoolConnectionPool, ExampleManager};
use dev_engineeringlabs_rustboot_database::Database;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rustboot Database Pool - Basic Example ===\n");

    // 1. Create pool configuration
    let config = PoolConfig::new("postgres://user:pass@localhost:5432/mydb")
        .with_max_size(10)
        .with_min_idle(2)
        .with_connection_timeout(Duration::from_secs(30))
        .with_max_lifetime(Duration::from_secs(30 * 60))
        .with_idle_timeout(Duration::from_secs(10 * 60));

    println!("Pool Configuration:");
    println!("  Max Size: {}", config.max_size);
    println!("  Min Idle: {:?}", config.min_idle);
    println!("  Connection Timeout: {:?}", config.connection_timeout);
    println!("  Max Lifetime: {:?}", config.max_lifetime);
    println!("  Idle Timeout: {:?}\n", config.idle_timeout);

    // 2. Create the connection pool
    // Note: In a real application, you would use a real database manager
    // instead of ExampleManager
    let manager = ExampleManager::new(config.connection_string.clone());
    let pool = DeadpoolConnectionPool::new(manager, config);

    println!("Pool created successfully!");
    println!("Max pool size: {}\n", pool.max_size());

    // 3. Check initial pool status
    let status = pool.status();
    print_pool_status(&status);

    // 4. Get a connection from the pool
    println!("\nGetting connection from pool...");
    let conn = pool.get().await?;
    println!("Connection acquired!");

    // 5. Check pool status after acquiring connection
    let status = pool.status();
    print_pool_status(&status);

    // 6. Use the connection (mock query)
    println!("\nExecuting query...");
    let _rows = conn.query("SELECT * FROM users").await?;
    println!("Query executed successfully!");

    // 7. Connection is automatically returned to pool when dropped
    drop(conn);
    println!("\nConnection returned to pool");

    // 8. Check final pool status
    let status = pool.status();
    print_pool_status(&status);

    // 9. Demonstrate getting multiple connections
    println!("\nGetting multiple connections...");
    let mut connections = Vec::new();
    for i in 0..3 {
        let conn = pool.get().await?;
        println!("  Connection {} acquired", i + 1);
        connections.push(conn);
    }

    let status = pool.status();
    print_pool_status(&status);

    // 10. Release all connections
    println!("\nReleasing all connections...");
    connections.clear();

    let status = pool.status();
    print_pool_status(&status);

    // 11. Demonstrate timeout
    println!("\nTesting connection with custom timeout...");
    let timeout_conn = pool.get_timeout(Duration::from_secs(5)).await?;
    println!("Connection acquired with custom timeout!");
    drop(timeout_conn);

    println!("\n=== Example Complete ===");
    Ok(())
}

fn print_pool_status(status: &PoolStatus) {
    println!("\nPool Status:");
    println!("  Total Connections: {}", status.size);
    println!("  Idle Connections: {}", status.idle);
    println!("  Active Connections: {}", status.active);
    println!("  Max Size: {}", status.max_size);
    println!("  Waiting: {}", status.waiting);
    println!("  Utilization: {:.1}%", status.utilization() * 100.0);
    println!("  Is Full: {}", status.is_full());
    println!("  Has Idle: {}", status.has_idle());
}
