//! Basic migration example
//!
//! This example demonstrates how to:
//! - Create SQL migrations programmatically
//! - Define migration versions and descriptions
//! - Structure up and down migration SQL

use async_trait::async_trait;
use dev_engineeringlabs_rustboot_database::{
    Database, DatabaseError, DatabaseResult, MigrationRunner, Row, SqlMigration, Transaction, Version,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Simple in-memory database for demonstration
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

    fn table_exists(&self, name: &str) -> bool {
        self.tables.lock().unwrap().contains_key(name)
    }
}

#[async_trait]
impl Database for InMemoryDatabase {
    async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>> {
        println!("QUERY: {}", sql);

        let tables = self.tables.lock().unwrap();

        // Simple parsing for SELECT
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
        println!("EXECUTE: {}", sql);

        let sql_upper = sql.to_uppercase();
        let mut tables = self.tables.lock().unwrap();

        // Parse CREATE TABLE
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

                println!("  → Created table: {}", table_name);
                tables.insert(table_name, Vec::new());
                return Ok(1);
            }
        }

        // Parse DROP TABLE
        if sql_upper.starts_with("DROP TABLE") {
            if let Some(table_start) = sql_upper.find("TABLE") {
                let after_table = &sql[table_start + 5..].trim();
                let table_name = after_table.split_whitespace().next().unwrap_or("").trim();

                println!("  → Dropped table: {}", table_name);
                tables.remove(table_name);
                return Ok(1);
            }
        }

        // Parse INSERT
        if sql_upper.starts_with("INSERT INTO") {
            if let Some(into_idx) = sql_upper.find("INTO") {
                let after_into = &sql[into_idx + 4..].trim();
                let table_name = after_into.split_whitespace().next().unwrap_or("").trim();

                if let Some(rows) = tables.get_mut(table_name) {
                    rows.push(Row::new());
                    return Ok(1);
                }
            }
        }

        // Parse DELETE
        if sql_upper.starts_with("DELETE FROM") {
            if let Some(from_idx) = sql_upper.find("FROM") {
                let after_from = &sql[from_idx + 4..].trim();
                let table_name = after_from.split_whitespace().next().unwrap_or("").trim();

                if let Some(rows) = tables.get_mut(table_name) {
                    let count = rows.len() as u64;
                    rows.clear();
                    return Ok(count);
                }
            }
        }

        Ok(0)
    }

    async fn begin_transaction(&self) -> DatabaseResult<Box<dyn Transaction>> {
        Err(DatabaseError::Query(
            "Transactions not implemented in example".to_string(),
        ))
    }
}

#[tokio::main]
async fn main() {
    println!("=== Rustboot Database Migration Example ===\n");

    // Create database
    let db = InMemoryDatabase::new();
    let runner = MigrationRunner::new(Box::new(db.clone()));

    // Initialize migrations table
    println!("1. Initializing migration system...");
    runner.init().await.unwrap();
    println!("   ✓ Migration table created\n");

    // Create migrations
    println!("2. Creating migrations...");
    let migrations = vec![
        SqlMigration::new(
            Version::new("20231201120000"),
            "create users table",
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL, email TEXT UNIQUE);",
            "DROP TABLE users;",
        ),
        SqlMigration::new(
            Version::new("20231201130000"),
            "create posts table",
            "CREATE TABLE posts (id INTEGER PRIMARY KEY, user_id INTEGER, title TEXT, content TEXT);",
            "DROP TABLE posts;",
        ),
        SqlMigration::new(
            Version::new("20231201140000"),
            "create comments table",
            "CREATE TABLE comments (id INTEGER PRIMARY KEY, post_id INTEGER, user_id INTEGER, content TEXT);",
            "DROP TABLE comments;",
        ),
    ];
    println!("   ✓ Created {} migrations\n", migrations.len());

    // Check status before migration
    println!("3. Checking migration status (before)...");
    let status = runner.get_status(&migrations).await.unwrap();
    for record in &status {
        let status_str = match record.status {
            dev_engineeringlabs_rustboot_database::MigrationStatus::Pending => "Pending",
            dev_engineeringlabs_rustboot_database::MigrationStatus::Applied => "Applied",
        };
        println!(
            "   - {}: {} ({})",
            record.version, record.description, status_str
        );
    }
    println!();

    // Run migrations
    println!("4. Running migrations...");
    let executed = runner.migrate(&migrations).await.unwrap();
    println!("   ✓ Applied {} migrations:", executed.len());
    for version in &executed {
        println!("     - {}", version);
    }
    println!();

    // Verify tables exist
    println!("5. Verifying tables...");
    assert!(db.table_exists("users"));
    assert!(db.table_exists("posts"));
    assert!(db.table_exists("comments"));
    println!("   ✓ All tables created successfully\n");

    // Check status after migration
    println!("6. Checking migration status (after)...");
    let applied = runner.get_applied_migrations().await.unwrap();
    println!("   ✓ {} migrations applied", applied.len());
    println!();

    // Rollback last migration
    println!("7. Rolling back last migration...");
    let rolled_back = runner.rollback(&migrations, 1).await.unwrap();
    println!("   ✓ Rolled back:");
    for version in &rolled_back {
        println!("     - {}", version);
    }
    println!();

    // Verify rollback
    println!("8. Verifying rollback...");
    assert!(db.table_exists("users"));
    assert!(db.table_exists("posts"));
    assert!(!db.table_exists("comments"));
    println!("   ✓ Comments table removed\n");

    // Run migrations again to re-apply
    println!("9. Re-running migrations...");
    let executed = runner.migrate(&migrations).await.unwrap();
    println!("   ✓ Applied {} migration(s)\n", executed.len());

    // Rollback to specific version
    println!("10. Rolling back to version 20231201120000...");
    runner
        .rollback_to(&migrations, &Version::new("20231201120000"))
        .await
        .unwrap();
    println!("   ✓ Rolled back to version 20231201120000\n");

    // Final verification
    println!("11. Final verification...");
    assert!(db.table_exists("users"));
    assert!(!db.table_exists("posts"));
    assert!(!db.table_exists("comments"));
    println!("   ✓ Only users table remains\n");

    println!("=== Migration Example Complete ===");
}
