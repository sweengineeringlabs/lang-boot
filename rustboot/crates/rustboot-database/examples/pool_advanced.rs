//! Advanced connection pooling example.
//!
//! This example demonstrates:
//! - Using the builder pattern for pool creation
//! - Concurrent connection access
//! - Pool lifecycle management
//! - Error handling

use dev_engineeringlabs_rustboot_database::pool::prelude::*;
use dev_engineeringlabs_rustboot_database::pool::{DeadpoolConnectionPool, ExampleManager};
use dev_engineeringlabs_rustboot_database::Database;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rustboot Database Pool - Advanced Example ===\n");

    // 1. Create pool using builder pattern
    let manager = ExampleManager::new("postgres://localhost/mydb".to_string());
    let pool = DeadpoolConnectionPool::builder(manager)
        .max_size(20)
        .timeout(Duration::from_secs(30))
        .build();

    println!("Pool created with builder pattern");
    println!("Max size: {}", pool.max_size());
    println!("Timeout: {:?}\n", pool.timeout());

    // Wrap pool in Arc for sharing across tasks
    let pool = Arc::new(pool);

    // 2. Concurrent connection usage
    println!("Spawning concurrent tasks...");
    let mut tasks = JoinSet::new();

    for task_id in 0..10 {
        let pool_clone = Arc::clone(&pool);
        tasks.spawn(async move {
            simulate_database_work(task_id, pool_clone).await
        });
    }

    // Wait for all tasks to complete
    let mut success_count = 0;
    let mut error_count = 0;

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(Ok(_)) => success_count += 1,
            Ok(Err(e)) => {
                error_count += 1;
                eprintln!("Task failed: {}", e);
            }
            Err(e) => {
                error_count += 1;
                eprintln!("Task panicked: {}", e);
            }
        }
    }

    println!("\nConcurrent tasks completed:");
    println!("  Successful: {}", success_count);
    println!("  Failed: {}", error_count);

    // 3. Monitor pool status
    let status = pool.status();
    println!("\nFinal pool status:");
    println!("  Total connections: {}", status.size);
    println!("  Idle connections: {}", status.idle);
    println!("  Active connections: {}", status.active);
    println!("  Utilization: {:.1}%", status.utilization() * 100.0);

    // 4. Demonstrate pool stress test
    println!("\nRunning stress test...");
    stress_test(Arc::clone(&pool), 50).await?;

    // 5. Clean shutdown
    println!("\nClosing pool...");
    pool.close().await?;
    println!("Pool closed: {}", pool.is_closed());

    println!("\n=== Example Complete ===");
    Ok(())
}

/// Simulates database work with a connection from the pool.
async fn simulate_database_work(
    task_id: usize,
    pool: Arc<DeadpoolConnectionPool<ExampleManager>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Get connection from pool
    let conn = pool.get().await?;

    // Simulate some work
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Execute a query
    let _rows = conn.query("SELECT * FROM data").await?;

    println!("Task {} completed query", task_id);

    // Connection is automatically returned to pool when dropped
    Ok(())
}

/// Stress tests the connection pool with many concurrent requests.
async fn stress_test(
    pool: Arc<DeadpoolConnectionPool<ExampleManager>>,
    num_requests: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    let mut tasks = JoinSet::new();

    for i in 0..num_requests {
        let pool_clone = Arc::clone(&pool);
        tasks.spawn(async move {
            let conn = pool_clone.get().await?;
            tokio::time::sleep(Duration::from_millis(10)).await;
            let _result = conn.query("SELECT 1").await?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(i)
        });
    }

    let mut completed = 0;
    while let Some(result) = tasks.join_next().await {
        if result.is_ok() {
            completed += 1;
        }
    }

    let elapsed = start.elapsed();

    println!("Stress test results:");
    println!("  Requests: {}", num_requests);
    println!("  Completed: {}", completed);
    println!("  Duration: {:?}", elapsed);
    println!("  Throughput: {:.2} req/sec", num_requests as f64 / elapsed.as_secs_f64());

    Ok(())
}
