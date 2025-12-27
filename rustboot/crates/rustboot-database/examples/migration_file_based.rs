//! File-based migration example
//!
//! This example demonstrates how to:
//! - Load migrations from SQL files
//! - Create new migration files
//! - Run file-based migrations

use dev_engineeringlabs_rustboot_database::{Migration, MigrationLoader};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rustboot File-Based Migration Example ===\n");

    // Get migrations directory (use temp directory for example)
    let migrations_dir = env::temp_dir().join("rustboot_migrations_example");
    tokio::fs::create_dir_all(&migrations_dir).await?;

    println!("Migrations directory: {}\n", migrations_dir.display());

    // Create migration loader
    let loader = MigrationLoader::new(&migrations_dir);

    // Create some example migration files
    println!("1. Creating migration files...");

    let (version1, path1) = loader.create_migration("create users table").await?;
    println!("   ✓ Created: {} at {:?}", version1, path1.file_name());

    // Wait a bit to ensure different timestamps
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let (version2, path2) = loader.create_migration("create posts table").await?;
    println!("   ✓ Created: {} at {:?}", version2, path2.file_name());

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let (version3, path3) = loader
        .create_migration("add indexes to users")
        .await?;
    println!("   ✓ Created: {} at {:?}\n", version3, path3.file_name());

    // Write actual SQL to the files
    println!("2. Writing SQL to migration files...");

    let migration1_content = r#"-- Migration: create users table
-- Created: 2023-12-01 12:00:00

-- migrate:up
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    created_at TEXT NOT NULL
);

-- migrate:down
DROP TABLE users;
"#;
    tokio::fs::write(&path1, migration1_content).await?;
    println!("   ✓ Wrote SQL to {}", path1.file_name().unwrap().to_string_lossy());

    let migration2_content = r#"-- Migration: create posts table
-- Created: 2023-12-01 13:00:00

-- migrate:up
CREATE TABLE posts (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- migrate:down
DROP TABLE posts;
"#;
    tokio::fs::write(&path2, migration2_content).await?;
    println!("   ✓ Wrote SQL to {}", path2.file_name().unwrap().to_string_lossy());

    let migration3_content = r#"-- Migration: add indexes to users
-- Created: 2023-12-01 14:00:00

-- migrate:up
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);

-- migrate:down
DROP INDEX idx_users_username;
DROP INDEX idx_users_email;
"#;
    tokio::fs::write(&path3, migration3_content).await?;
    println!("   ✓ Wrote SQL to {}\n", path3.file_name().unwrap().to_string_lossy());

    // Load migrations from files
    println!("3. Loading migrations from files...");
    let migrations = loader.load_migrations().await?;
    println!("   ✓ Loaded {} migrations:", migrations.len());
    for migration in &migrations {
        println!(
            "     - {}: {}",
            migration.version(),
            migration.description()
        );
        println!("       Checksum: {}", migration.checksum());
    }
    println!();

    // Display migration details
    println!("4. Migration details:\n");
    for migration in &migrations {
        println!("   Version: {}", migration.version());
        println!("   Description: {}", migration.description());
        println!("   Up SQL:");
        for line in migration.up_sql().lines().take(5) {
            println!("     {}", line);
        }
        if migration.up_sql().lines().count() > 5 {
            println!("     ...");
        }
        println!("   Down SQL:");
        for line in migration.down_sql().lines().take(3) {
            println!("     {}", line);
        }
        if migration.down_sql().lines().count() > 3 {
            println!("     ...");
        }
        println!();
    }

    // Note: To actually run these migrations, you would need a real database connection
    println!("5. To run these migrations:");
    println!("   ```rust");
    println!("   let db = /* your database implementation */;");
    println!("   let runner = MigrationRunner::new(Box::new(db));");
    println!("   runner.init().await?;");
    println!("   runner.migrate(&migrations).await?;");
    println!("   ```\n");

    // Cleanup
    println!("6. Cleanup...");
    tokio::fs::remove_dir_all(&migrations_dir).await?;
    println!("   ✓ Removed temporary migration files\n");

    println!("=== File-Based Migration Example Complete ===");

    Ok(())
}
