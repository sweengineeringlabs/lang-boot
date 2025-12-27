//! Pool configuration example.
//!
//! This example demonstrates various pool configuration options
//! and their effects on pool behavior.

use dev_engineeringlabs_rustboot_database::pool::prelude::*;
use dev_engineeringlabs_rustboot_database::pool::{DeadpoolConnectionPool, ExampleManager};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Pool Configuration Examples ===\n");

    // Example 1: Minimal configuration
    println!("1. Minimal Configuration");
    let minimal_config = PoolConfig::new("postgres://localhost/db");
    print_config("Minimal", &minimal_config);

    // Example 2: High-performance configuration
    println!("\n2. High-Performance Configuration");
    let high_perf_config = PoolConfig::new("postgres://localhost/db")
        .with_max_size(50)
        .with_min_idle(10)
        .with_connection_timeout(Duration::from_secs(5))
        .with_max_lifetime(Duration::from_secs(60 * 60)) // 1 hour
        .with_idle_timeout(Duration::from_secs(5 * 60)); // 5 minutes
    print_config("High-Performance", &high_perf_config);

    // Example 3: Conservative configuration (for limited resources)
    println!("\n3. Conservative Configuration");
    let conservative_config = PoolConfig::new("postgres://localhost/db")
        .with_max_size(5)
        .with_min_idle(1)
        .with_connection_timeout(Duration::from_secs(60))
        .with_max_lifetime(Duration::from_secs(15 * 60)) // 15 minutes
        .with_idle_timeout(Duration::from_secs(2 * 60)); // 2 minutes
    print_config("Conservative", &conservative_config);

    // Example 4: Development configuration (with validation)
    println!("\n4. Development Configuration (with validation)");
    let dev_config = PoolConfig::new("postgres://localhost/dev_db")
        .with_max_size(10)
        .with_min_idle(2)
        .with_test_on_acquire(true)
        .with_test_on_release(true)
        .with_connection_timeout(Duration::from_secs(30));
    print_config("Development", &dev_config);

    // Example 5: Production configuration
    println!("\n5. Production Configuration");
    let prod_config = PoolConfig::new("postgres://prod-host/prod_db")
        .with_max_size(100)
        .with_min_idle(20)
        .with_connection_timeout(Duration::from_secs(10))
        .with_max_lifetime(Duration::from_secs(30 * 60)) // 30 minutes
        .with_idle_timeout(Duration::from_secs(10 * 60)) // 10 minutes
        .with_test_on_acquire(false); // Disabled for performance
    print_config("Production", &prod_config);

    // Example 6: Create and use a pool with custom configuration
    println!("\n6. Creating Pool with Custom Configuration");
    let manager = ExampleManager::new("postgres://localhost/db".to_string());
    let pool = DeadpoolConnectionPool::new(manager, high_perf_config);

    println!("Pool created successfully!");
    println!("  Max size: {}", pool.max_size());
    println!("  Timeout: {:?}", pool.timeout());

    // Get a connection to verify it works
    let conn = pool.get().await?;
    println!("  Test connection acquired: ✓");
    drop(conn);

    // Show pool status
    let status = pool.status();
    println!("\nPool Status:");
    println!("  Size: {}/{}", status.size, status.max_size);
    println!("  Idle: {}", status.idle);
    println!("  Active: {}", status.active);

    println!("\n=== Example Complete ===");
    Ok(())
}

fn print_config(name: &str, config: &PoolConfig) {
    println!("{} Config:", name);
    println!("  Connection String: {}",
        if config.connection_string.is_empty() {
            "<not set>"
        } else {
            &config.connection_string
        });
    println!("  Max Size: {}", config.max_size);
    println!("  Min Idle: {:?}", config.min_idle);
    println!("  Connection Timeout: {:?}", config.connection_timeout);
    println!("  Max Lifetime: {:?}", config.max_lifetime);
    println!("  Idle Timeout: {:?}", config.idle_timeout);
    println!("  Test on Acquire: {}", config.test_on_acquire);
    println!("  Test on Release: {}", config.test_on_release);
}
