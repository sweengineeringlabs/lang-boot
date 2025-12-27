//! Integration tests for database migrations

use async_trait::async_trait;
use dev_engineeringlabs_rustboot_database::{
    Database, DatabaseError, DatabaseResult, Migration, MigrationError, MigrationLoader,
    MigrationRunner, MigrationStatus, Row, SqlMigration, Transaction, Value, Version,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Mock database for testing migrations
#[derive(Clone)]
struct MockDatabase {
    tables: Arc<Mutex<HashMap<String, Vec<Row>>>>,
    executed_statements: Arc<Mutex<Vec<String>>>,
}

impl MockDatabase {
    fn new() -> Self {
        Self {
            tables: Arc::new(Mutex::new(HashMap::new())),
            executed_statements: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_executed_statements(&self) -> Vec<String> {
        self.executed_statements.lock().unwrap().clone()
    }

    fn clear_executed_statements(&self) {
        self.executed_statements.lock().unwrap().clear();
    }

    fn table_exists(&self, table_name: &str) -> bool {
        self.tables.lock().unwrap().contains_key(table_name)
    }

    fn get_table_rows(&self, table_name: &str) -> Vec<Row> {
        self.tables
            .lock()
            .unwrap()
            .get(table_name)
            .cloned()
            .unwrap_or_default()
    }
}

#[async_trait]
impl Database for MockDatabase {
    async fn query(&self, sql: &str) -> DatabaseResult<Vec<Row>> {
        self.executed_statements
            .lock()
            .unwrap()
            .push(sql.to_string());

        // Parse simple SELECT queries
        if sql.to_uppercase().starts_with("SELECT") {
            let tables = self.tables.lock().unwrap();

            // Extract table name (very basic parsing)
            if let Some(from_idx) = sql.to_uppercase().find("FROM") {
                let after_from = sql[from_idx + 4..].trim();
                let table_name = after_from
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim();

                if let Some(rows) = tables.get(table_name) {
                    return Ok(rows.clone());
                }
            }

            return Ok(Vec::new());
        }

        Ok(Vec::new())
    }

    async fn execute(&self, sql: &str) -> DatabaseResult<u64> {
        self.executed_statements
            .lock()
            .unwrap()
            .push(sql.to_string());

        let sql_upper = sql.to_uppercase();
        let mut tables = self.tables.lock().unwrap();

        // Parse simple CREATE TABLE
        if sql_upper.starts_with("CREATE TABLE") {
            if let Some(table_start) = sql_upper.find("TABLE") {
                let after_table = sql[table_start + 5..].trim();
                // Handle "IF NOT EXISTS"
                let after_table = if after_table.to_uppercase().starts_with("IF NOT EXISTS") {
                    after_table[13..].trim()
                } else {
                    after_table
                };

                let table_name = after_table
                    .split(|c: char| c.is_whitespace() || c == '(')
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();

                tables.insert(table_name, Vec::new());
                return Ok(1);
            }
        }

        // Parse simple DROP TABLE
        if sql_upper.starts_with("DROP TABLE") {
            if let Some(table_start) = sql_upper.find("TABLE") {
                let after_table = sql[table_start + 5..].trim();
                let table_name = after_table
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim()
                    .trim_end_matches(';')
                    .to_string();

                tables.remove(&table_name);
                return Ok(1);
            }
        }

        // Parse simple INSERT
        if sql_upper.starts_with("INSERT INTO") {
            if let Some(into_idx) = sql_upper.find("INTO") {
                let after_into = sql[into_idx + 4..].trim();
                let table_name = after_into
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();

                if let Some(rows) = tables.get_mut(&table_name) {
                    let mut row = Row::new();

                    // Special handling for migrations table
                    if table_name == "_migrations" {
                        // Extract VALUES clause
                        if let Some(values_idx) = sql_upper.find("VALUES") {
                            let values_part = sql[values_idx + 6..].trim();
                            // Extract values between parentheses
                            if let Some(start) = values_part.find('(') {
                                if let Some(end) = values_part.find(')') {
                                    let values = &values_part[start + 1..end];
                                    let parts: Vec<&str> =
                                        values.split(',').map(|s| s.trim()).collect();

                                    if parts.len() >= 4 {
                                        // Remove quotes from values
                                        let version = parts[0].trim_matches('\'');
                                        let description = parts[1].trim_matches('\'');
                                        let checksum = parts[2].trim_matches('\'');
                                        let applied_at = parts[3].trim_matches('\'');

                                        row.set("version", Value::String(version.to_string()));
                                        row.set(
                                            "description",
                                            Value::String(description.to_string()),
                                        );
                                        row.set("checksum", Value::String(checksum.to_string()));
                                        row.set(
                                            "applied_at",
                                            Value::String(applied_at.to_string()),
                                        );
                                    }
                                }
                            }
                        }
                    } else {
                        // For other tables, just add a dummy row
                        row.set("dummy", Value::String("value".to_string()));
                    }

                    rows.push(row);
                    return Ok(1);
                }
            }
        }

        // Parse simple DELETE
        if sql_upper.starts_with("DELETE FROM") {
            if let Some(from_idx) = sql_upper.find("FROM") {
                let after_from = sql[from_idx + 4..].trim();
                let parts: Vec<&str> = after_from.split_whitespace().collect();
                let table_name = parts[0].to_string();

                if let Some(rows) = tables.get_mut(&table_name) {
                    // Check for WHERE clause
                    if let Some(where_idx) = sql_upper.find("WHERE") {
                        let where_clause = &sql[where_idx + 5..].trim();

                        // Simple WHERE version = 'value' parsing
                        if where_clause.to_uppercase().starts_with("VERSION") {
                            if let Some(eq_idx) = where_clause.find('=') {
                                let value = where_clause[eq_idx + 1..].trim().trim_matches('\'');

                                // Remove rows matching the version
                                let before_len = rows.len();
                                rows.retain(|row| {
                                    if let Some(Value::String(v)) = row.get("version") {
                                        v != value
                                    } else {
                                        true
                                    }
                                });
                                let removed = (before_len - rows.len()) as u64;
                                return Ok(removed);
                            }
                        }
                    } else {
                        // No WHERE clause - delete all
                        let count = rows.len() as u64;
                        rows.clear();
                        return Ok(count);
                    }
                }
            }
        }

        Ok(0)
    }

    async fn begin_transaction(&self) -> DatabaseResult<Box<dyn Transaction>> {
        Err(DatabaseError::Query(
            "Transactions not supported in mock".to_string(),
        ))
    }
}

#[tokio::test]
async fn test_version_ordering() {
    let v1 = Version::new("20231201120000");
    let v2 = Version::new("20231201130000");
    let v3 = Version::new("20231202120000");

    assert!(v1 < v2);
    assert!(v2 < v3);
    assert!(v1 < v3);

    let mut versions = vec![v3.clone(), v1.clone(), v2.clone()];
    versions.sort();

    assert_eq!(versions, vec![v1, v2, v3]);
}

#[tokio::test]
async fn test_version_parse() {
    assert!(Version::parse("20231201120000").is_ok());
    assert!(Version::parse("v1_initial").is_ok());
    assert!(Version::parse("001").is_ok());
    assert!(Version::parse("").is_err());
    assert!(Version::parse("invalid!version").is_err());
}

#[tokio::test]
async fn test_sql_migration_creation() {
    let migration = SqlMigration::new(
        Version::new("001"),
        "create users table",
        "CREATE TABLE users (id INTEGER PRIMARY KEY);",
        "DROP TABLE users;",
    );

    assert_eq!(migration.version().as_str(), "001");
    assert_eq!(migration.description(), "create users table");
    assert!(migration.up_sql().contains("CREATE TABLE users"));
    assert!(migration.down_sql().contains("DROP TABLE users"));
}

#[tokio::test]
async fn test_sql_migration_checksum() {
    let migration1 = SqlMigration::new(
        Version::new("001"),
        "test",
        "CREATE TABLE test;",
        "DROP TABLE test;",
    );

    let migration2 = SqlMigration::new(
        Version::new("001"),
        "test",
        "CREATE TABLE test;",
        "DROP TABLE test;",
    );

    let migration3 = SqlMigration::new(
        Version::new("001"),
        "test",
        "CREATE TABLE different;",
        "DROP TABLE different;",
    );

    assert_eq!(migration1.checksum(), migration2.checksum());
    assert_ne!(migration1.checksum(), migration3.checksum());
}

#[tokio::test]
async fn test_migration_up() {
    let db = MockDatabase::new();
    let migration = SqlMigration::new(
        Version::new("001"),
        "create users table",
        "CREATE TABLE users (id INTEGER PRIMARY KEY);",
        "DROP TABLE users;",
    );

    migration.up(&db).await.unwrap();

    assert!(db.table_exists("users"));
    let statements = db.get_executed_statements();
    assert!(statements
        .iter()
        .any(|s| s.contains("CREATE TABLE users")));
}

#[tokio::test]
async fn test_migration_down() {
    let db = MockDatabase::new();
    let migration = SqlMigration::new(
        Version::new("001"),
        "create users table",
        "CREATE TABLE users (id INTEGER PRIMARY KEY);",
        "DROP TABLE users;",
    );

    // First create the table
    migration.up(&db).await.unwrap();
    assert!(db.table_exists("users"));

    // Then drop it
    db.clear_executed_statements();
    migration.down(&db).await.unwrap();

    assert!(!db.table_exists("users"));
    let statements = db.get_executed_statements();
    assert!(statements.iter().any(|s| s.contains("DROP TABLE users")));
}

#[tokio::test]
async fn test_migration_runner_init() {
    let db = MockDatabase::new();
    let runner = MigrationRunner::new(Box::new(db.clone()));

    runner.init().await.unwrap();

    assert!(db.table_exists("_migrations"));
}

#[tokio::test]
async fn test_migration_runner_get_applied_migrations() {
    let db = MockDatabase::new();
    let runner = MigrationRunner::new(Box::new(db.clone()));

    runner.init().await.unwrap();

    // Initially, no migrations should be applied
    let applied = runner.get_applied_migrations().await.unwrap();
    assert_eq!(applied.len(), 0);
}

#[tokio::test]
async fn test_migration_runner_migrate() {
    let db = MockDatabase::new();
    let runner = MigrationRunner::new(Box::new(db.clone()));

    runner.init().await.unwrap();

    let migrations = vec![
        SqlMigration::new(
            Version::new("001"),
            "create users table",
            "CREATE TABLE users (id INTEGER PRIMARY KEY);",
            "DROP TABLE users;",
        ),
        SqlMigration::new(
            Version::new("002"),
            "create posts table",
            "CREATE TABLE posts (id INTEGER PRIMARY KEY);",
            "DROP TABLE posts;",
        ),
    ];

    let executed = runner.migrate(&migrations).await.unwrap();

    assert_eq!(executed.len(), 2);
    assert!(db.table_exists("users"));
    assert!(db.table_exists("posts"));

    // Check that migrations are recorded
    let applied = runner.get_applied_migrations().await.unwrap();
    assert_eq!(applied.len(), 2);
}

#[tokio::test]
async fn test_migration_runner_migrate_idempotent() {
    let db = MockDatabase::new();
    let runner = MigrationRunner::new(Box::new(db.clone()));

    runner.init().await.unwrap();

    let migrations = vec![SqlMigration::new(
        Version::new("001"),
        "create users table",
        "CREATE TABLE users (id INTEGER PRIMARY KEY);",
        "DROP TABLE users;",
    )];

    // Run migrations twice
    let executed1 = runner.migrate(&migrations).await.unwrap();
    let executed2 = runner.migrate(&migrations).await.unwrap();

    assert_eq!(executed1.len(), 1);
    assert_eq!(executed2.len(), 0); // Should not re-apply
}

#[tokio::test]
async fn test_migration_runner_migrate_to() {
    let db = MockDatabase::new();
    let runner = MigrationRunner::new(Box::new(db.clone()));

    runner.init().await.unwrap();

    let migrations = vec![
        SqlMigration::new(
            Version::new("001"),
            "create users table",
            "CREATE TABLE users (id INTEGER PRIMARY KEY);",
            "DROP TABLE users;",
        ),
        SqlMigration::new(
            Version::new("002"),
            "create posts table",
            "CREATE TABLE posts (id INTEGER PRIMARY KEY);",
            "DROP TABLE posts;",
        ),
        SqlMigration::new(
            Version::new("003"),
            "create comments table",
            "CREATE TABLE comments (id INTEGER PRIMARY KEY);",
            "DROP TABLE comments;",
        ),
    ];

    // Migrate only to version 002
    let executed = runner
        .migrate_to(&migrations, &Version::new("002"))
        .await
        .unwrap();

    assert_eq!(executed.len(), 2);
    assert!(db.table_exists("users"));
    assert!(db.table_exists("posts"));
    assert!(!db.table_exists("comments"));
}

#[tokio::test]
async fn test_migration_runner_rollback() {
    let db = MockDatabase::new();
    let runner = MigrationRunner::new(Box::new(db.clone()));

    runner.init().await.unwrap();

    let migrations = vec![
        SqlMigration::new(
            Version::new("001"),
            "create users table",
            "CREATE TABLE users (id INTEGER PRIMARY KEY);",
            "DROP TABLE users;",
        ),
        SqlMigration::new(
            Version::new("002"),
            "create posts table",
            "CREATE TABLE posts (id INTEGER PRIMARY KEY);",
            "DROP TABLE posts;",
        ),
    ];

    // Apply all migrations
    runner.migrate(&migrations).await.unwrap();
    assert!(db.table_exists("users"));
    assert!(db.table_exists("posts"));

    // Rollback last migration
    let rolled_back = runner.rollback(&migrations, 1).await.unwrap();

    assert_eq!(rolled_back.len(), 1);
    assert_eq!(rolled_back[0].as_str(), "002");
    assert!(db.table_exists("users"));
    assert!(!db.table_exists("posts"));
}

#[tokio::test]
async fn test_migration_runner_rollback_multiple() {
    let db = MockDatabase::new();
    let runner = MigrationRunner::new(Box::new(db.clone()));

    runner.init().await.unwrap();

    let migrations = vec![
        SqlMigration::new(
            Version::new("001"),
            "create users table",
            "CREATE TABLE users (id INTEGER PRIMARY KEY);",
            "DROP TABLE users;",
        ),
        SqlMigration::new(
            Version::new("002"),
            "create posts table",
            "CREATE TABLE posts (id INTEGER PRIMARY KEY);",
            "DROP TABLE posts;",
        ),
        SqlMigration::new(
            Version::new("003"),
            "create comments table",
            "CREATE TABLE comments (id INTEGER PRIMARY KEY);",
            "DROP TABLE comments;",
        ),
    ];

    // Apply all migrations
    runner.migrate(&migrations).await.unwrap();

    // Rollback last 2 migrations
    let rolled_back = runner.rollback(&migrations, 2).await.unwrap();

    assert_eq!(rolled_back.len(), 2);
    assert!(db.table_exists("users"));
    assert!(!db.table_exists("posts"));
    assert!(!db.table_exists("comments"));
}

#[tokio::test]
async fn test_migration_runner_rollback_to() {
    let db = MockDatabase::new();
    let runner = MigrationRunner::new(Box::new(db.clone()));

    runner.init().await.unwrap();

    let migrations = vec![
        SqlMigration::new(
            Version::new("001"),
            "create users table",
            "CREATE TABLE users (id INTEGER PRIMARY KEY);",
            "DROP TABLE users;",
        ),
        SqlMigration::new(
            Version::new("002"),
            "create posts table",
            "CREATE TABLE posts (id INTEGER PRIMARY KEY);",
            "DROP TABLE posts;",
        ),
        SqlMigration::new(
            Version::new("003"),
            "create comments table",
            "CREATE TABLE comments (id INTEGER PRIMARY KEY);",
            "DROP TABLE comments;",
        ),
    ];

    // Apply all migrations
    runner.migrate(&migrations).await.unwrap();

    // Rollback to version 001
    let rolled_back = runner
        .rollback_to(&migrations, &Version::new("001"))
        .await
        .unwrap();

    assert_eq!(rolled_back.len(), 2);
    assert!(db.table_exists("users"));
    assert!(!db.table_exists("posts"));
    assert!(!db.table_exists("comments"));
}

#[tokio::test]
async fn test_migration_runner_get_status() {
    let db = MockDatabase::new();
    let runner = MigrationRunner::new(Box::new(db.clone()));

    runner.init().await.unwrap();

    let migrations = vec![
        SqlMigration::new(
            Version::new("001"),
            "create users table",
            "CREATE TABLE users (id INTEGER PRIMARY KEY);",
            "DROP TABLE users;",
        ),
        SqlMigration::new(
            Version::new("002"),
            "create posts table",
            "CREATE TABLE posts (id INTEGER PRIMARY KEY);",
            "DROP TABLE posts;",
        ),
    ];

    // Get status before applying migrations
    let status = runner.get_status(&migrations).await.unwrap();
    assert_eq!(status.len(), 2);
    assert_eq!(status[0].status, MigrationStatus::Pending);
    assert_eq!(status[1].status, MigrationStatus::Pending);

    // Apply first migration
    runner
        .migrate_to(&migrations, &Version::new("001"))
        .await
        .unwrap();

    // Get status after applying first migration
    let status = runner.get_status(&migrations).await.unwrap();
    assert_eq!(status.len(), 2);
    assert_eq!(status[0].status, MigrationStatus::Applied);
    assert_eq!(status[1].status, MigrationStatus::Pending);
}

#[tokio::test]
async fn test_migration_loader_parse_content() {
    let loader = MigrationLoader::new("/tmp/migrations");

    let content = r#"
-- Some initial comment

-- migrate:up
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

-- migrate:down
DROP TABLE users;
    "#;

    let (up_sql, down_sql) = loader.parse_migration_content(content).unwrap();

    assert!(up_sql.contains("CREATE TABLE users"));
    assert!(up_sql.contains("id INTEGER PRIMARY KEY"));
    assert!(down_sql.contains("DROP TABLE users"));
}

#[tokio::test]
async fn test_migration_loader_parse_content_no_down() {
    let loader = MigrationLoader::new("/tmp/migrations");

    let content = r#"
-- migrate:up
CREATE TABLE users (id INTEGER PRIMARY KEY);
    "#;

    let (up_sql, down_sql) = loader.parse_migration_content(content).unwrap();

    assert!(up_sql.contains("CREATE TABLE users"));
    assert!(down_sql.is_empty());
}

#[tokio::test]
async fn test_migration_loader_parse_content_missing_up() {
    let loader = MigrationLoader::new("/tmp/migrations");

    let content = r#"
-- migrate:down
DROP TABLE users;
    "#;

    let result = loader.parse_migration_content(content);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_custom_migration_trait() {
    struct CustomMigration {
        version: Version,
    }

    #[async_trait]
    impl Migration for CustomMigration {
        fn version(&self) -> &Version {
            &self.version
        }

        fn description(&self) -> &str {
            "custom migration"
        }

        async fn up(&self, db: &dyn Database) -> Result<(), MigrationError> {
            db.execute("CREATE TABLE custom (id INTEGER);")
                .await
                .map_err(|e| MigrationError::Database(e.to_string()))?;
            Ok(())
        }

        async fn down(&self, db: &dyn Database) -> Result<(), MigrationError> {
            db.execute("DROP TABLE custom;")
                .await
                .map_err(|e| MigrationError::Database(e.to_string()))?;
            Ok(())
        }

        fn checksum(&self) -> String {
            "custom_checksum".to_string()
        }
    }

    let db = MockDatabase::new();
    let migration = CustomMigration {
        version: Version::new("custom_001"),
    };

    migration.up(&db).await.unwrap();
    assert!(db.table_exists("custom"));

    migration.down(&db).await.unwrap();
    assert!(!db.table_exists("custom"));
}
