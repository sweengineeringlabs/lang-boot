//! Custom migration implementation example
//!
//! This example demonstrates how to:
//! - Implement the Migration trait for custom migrations
//! - Combine SQL and programmatic logic
//! - Create complex migration scenarios

use async_trait::async_trait;
use dev_engineeringlabs_rustboot_database::{
    Database, DatabaseError, DatabaseResult, Migration, MigrationError, MigrationRunner, Row,
    Transaction, Value, Version,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// In-memory database for demonstration
#[derive(Clone)]
struct InMemoryDatabase {
    tables: Arc<Mutex<HashMap<String, Vec<Row>>>>,
}

impl InMemoryDatabase {
    fn new() -> Self {
        Self {
            tables: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Database for InMemoryDatabase {
    async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>> {
        println!("  QUERY: {}", sql);
        let tables = self.tables.lock().unwrap();

        if let Some(from_idx) = sql.to_uppercase().find("FROM") {
            let after_from = &sql[from_idx + 4..].trim();
            let table_name = after_from.split_whitespace().next().unwrap_or("").trim();

            if let Some(rows) = tables.get(table_name) {
                return Ok(rows.clone());
            }
        }

        Ok(Vec::new())
    }

    async fn execute(&self, sql: &str) -> DatabaseResult<u64> {
        println!("  EXEC: {}", sql);
        let sql_upper = sql.to_uppercase();
        let mut tables = self.tables.lock().unwrap();

        if sql_upper.starts_with("CREATE TABLE") {
            if let Some(table_start) = sql_upper.find("TABLE") {
                let after_table = &sql[table_start + 5..].trim();
                let table_name = after_table
                    .split(|c: char| c.is_whitespace() || c == '(')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .replace("IF NOT EXISTS", "")
                    .trim()
                    .to_string();

                tables.insert(table_name, Vec::new());
                return Ok(1);
            }
        }

        if sql_upper.starts_with("DROP TABLE") {
            if let Some(table_start) = sql_upper.find("TABLE") {
                let after_table = &sql[table_start + 5..].trim();
                let table_name = after_table.split_whitespace().next().unwrap_or("").trim();
                tables.remove(table_name);
                return Ok(1);
            }
        }

        if sql_upper.starts_with("ALTER TABLE") {
            // Just acknowledge the ALTER for this example
            return Ok(1);
        }

        Ok(0)
    }

    async fn begin_transaction(&self) -> DatabaseResult<Box<dyn Transaction>> {
        Err(DatabaseError::Query(
            "Transactions not implemented".to_string(),
        ))
    }
}

/// Custom migration that creates a table with validation
struct CreateUsersTableMigration {
    version: Version,
}

impl CreateUsersTableMigration {
    fn new() -> Self {
        Self {
            version: Version::new("custom_001"),
        }
    }
}

#[async_trait]
impl Migration for CreateUsersTableMigration {
    fn version(&self) -> &Version {
        &self.version
    }

    fn description(&self) -> &str {
        "Create users table with validation"
    }

    async fn up(&self, db: &dyn Database) -> Result<(), MigrationError> {
        println!("    Running custom up migration: {}", self.description());

        // Create table
        db.execute(
            "CREATE TABLE users (\
                id INTEGER PRIMARY KEY,\
                username TEXT NOT NULL,\
                email TEXT NOT NULL\
            )",
        )
        .await
        .map_err(|e| MigrationError::Database(e.to_string()))?;

        // Create index
        db.execute("CREATE INDEX idx_users_username ON users(username)")
            .await
            .map_err(|e| MigrationError::Database(e.to_string()))?;

        println!("    ✓ Created users table with index");

        Ok(())
    }

    async fn down(&self, db: &dyn Database) -> Result<(), MigrationError> {
        println!("    Running custom down migration: {}", self.description());

        // Drop index first
        db.execute("DROP INDEX idx_users_username")
            .await
            .map_err(|e| MigrationError::Database(e.to_string()))?;

        // Drop table
        db.execute("DROP TABLE users")
            .await
            .map_err(|e| MigrationError::Database(e.to_string()))?;

        println!("    ✓ Dropped users table");

        Ok(())
    }

    fn checksum(&self) -> String {
        "custom_users_migration_v1".to_string()
    }
}

/// Custom migration with data transformation
struct AddUserStatusMigration {
    version: Version,
}

impl AddUserStatusMigration {
    fn new() -> Self {
        Self {
            version: Version::new("custom_002"),
        }
    }
}

#[async_trait]
impl Migration for AddUserStatusMigration {
    fn version(&self) -> &Version {
        &self.version
    }

    fn description(&self) -> &str {
        "Add status column to users with data transformation"
    }

    async fn up(&self, db: &dyn Database) -> Result<(), MigrationError> {
        println!("    Running custom up migration: {}", self.description());

        // Add column
        db.execute("ALTER TABLE users ADD COLUMN status TEXT")
            .await
            .map_err(|e| MigrationError::Database(e.to_string()))?;

        // In a real scenario, you might update existing rows here
        println!("    ✓ Added status column");

        // You could also query and update existing data
        let users = db
            .query("SELECT * FROM users")
            .await
            .map_err(|e| MigrationError::Database(e.to_string()))?;

        if !users.is_empty() {
            println!("    ✓ Found {} existing users to update", users.len());
            // In real code, you'd update each user's status
        }

        Ok(())
    }

    async fn down(&self, db: &dyn Database) -> Result<(), MigrationError> {
        println!("    Running custom down migration: {}", self.description());

        // Note: SQLite doesn't support DROP COLUMN easily
        // In a real scenario, you'd need to recreate the table
        println!("    ✓ Would remove status column (simplified for example)");

        Ok(())
    }

    fn checksum(&self) -> String {
        "custom_user_status_migration_v1".to_string()
    }
}

/// Migration that performs validation
struct ValidateUserDataMigration {
    version: Version,
}

impl ValidateUserDataMigration {
    fn new() -> Self {
        Self {
            version: Version::new("custom_003"),
        }
    }
}

#[async_trait]
impl Migration for ValidateUserDataMigration {
    fn version(&self) -> &Version {
        &self.version
    }

    fn description(&self) -> &str {
        "Validate and clean user data"
    }

    async fn up(&self, db: &dyn Database) -> Result<(), MigrationError> {
        println!("    Running custom up migration: {}", self.description());

        // Query all users
        let users = db
            .query("SELECT * FROM users")
            .await
            .map_err(|e| MigrationError::Database(e.to_string()))?;

        println!("    → Validating {} users", users.len());

        // In a real scenario, you'd validate and clean data here
        for (i, user) in users.iter().enumerate() {
            if let Some(username) = user.get("username") {
                println!("      User {}: {:?}", i + 1, username);
            }
        }

        println!("    ✓ Validation complete");

        Ok(())
    }

    async fn down(&self, db: &dyn Database) -> Result<(), MigrationError> {
        println!("    Running custom down migration: {}", self.description());
        println!("    ✓ No rollback needed for validation");
        Ok(())
    }

    fn checksum(&self) -> String {
        "custom_validate_users_v1".to_string()
    }
}

#[tokio::main]
async fn main() {
    println!("=== Custom Migration Example ===\n");

    // Create database and runner
    let db = InMemoryDatabase::new();
    let runner = MigrationRunner::new(Box::new(db.clone()));

    // Initialize
    println!("1. Initializing migration system...");
    runner.init().await.unwrap();
    println!("   ✓ Initialized\n");

    // Create custom migrations
    println!("2. Creating custom migrations...");
    let migration1 = CreateUsersTableMigration::new();
    let migration2 = AddUserStatusMigration::new();
    let migration3 = ValidateUserDataMigration::new();
    println!("   ✓ Created 3 custom migrations\n");

    // Display migration info
    println!("3. Migration details:");
    println!("   - {}: {}", migration1.version(), migration1.description());
    println!("   - {}: {}", migration2.version(), migration2.description());
    println!("   - {}: {}", migration3.version(), migration3.description());
    println!();

    // Run first migration
    println!("4. Running migration 1...");
    migration1.up(&db as &dyn Database).await.unwrap();
    println!("   ✓ Migration 1 complete\n");

    // Run second migration
    println!("5. Running migration 2...");
    migration2.up(&db as &dyn Database).await.unwrap();
    println!("   ✓ Migration 2 complete\n");

    // Run third migration
    println!("6. Running migration 3...");
    migration3.up(&db as &dyn Database).await.unwrap();
    println!("   ✓ Migration 3 complete\n");

    // Rollback third migration
    println!("7. Rolling back migration 3...");
    migration3.down(&db as &dyn Database).await.unwrap();
    println!("   ✓ Migration 3 rolled back\n");

    // Rollback second migration
    println!("8. Rolling back migration 2...");
    migration2.down(&db as &dyn Database).await.unwrap();
    println!("   ✓ Migration 2 rolled back\n");

    // Rollback first migration
    println!("9. Rolling back migration 1...");
    migration1.down(&db as &dyn Database).await.unwrap();
    println!("   ✓ Migration 1 rolled back\n");

    println!("=== Custom Migration Example Complete ===");
    println!("\nKey Takeaways:");
    println!("- Custom migrations allow complex logic beyond SQL");
    println!("- You can combine SQL execution with data validation");
    println!("- Each migration can have unique up/down behavior");
    println!("- Checksums help track migration changes");
}
