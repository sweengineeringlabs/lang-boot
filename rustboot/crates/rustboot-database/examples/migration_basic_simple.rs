//! Basic migration example
//!
//! This example demonstrates how to:
//! - Create SQL migrations programmatically
//! - Define migration versions and descriptions
//! - Structure up and down migration SQL

use dev_engineeringlabs_rustboot_database::{Migration, SqlMigration, Version};

fn main() {
    println!("=== Rustboot Database Migration Example ===\n");

    // Create migrations programmatically
    println!("1. Creating SQL migrations...\n");

    let migration1 = SqlMigration::new(
        Version::new("20231201120000"),
        "create users table",
        r#"CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);"#,
        r#"DROP INDEX idx_users_email;
DROP INDEX idx_users_username;
DROP TABLE users;"#,
    );

    let migration2 = SqlMigration::new(
        Version::new("20231201130000"),
        "create posts table",
        r#"CREATE TABLE posts (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
CREATE INDEX idx_posts_user_id ON posts(user_id);"#,
        r#"DROP INDEX idx_posts_user_id;
DROP TABLE posts;"#,
    );

    let migration3 = SqlMigration::new(
        Version::new("20231201140000"),
        "create comments table",
        r#"CREATE TABLE comments (
    id INTEGER PRIMARY KEY,
    post_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (post_id) REFERENCES posts(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);
CREATE INDEX idx_comments_post_id ON comments(post_id);
CREATE INDEX idx_comments_user_id ON comments(user_id);"#,
        r#"DROP INDEX idx_comments_user_id;
DROP INDEX idx_comments_post_id;
DROP TABLE comments;"#,
    );

    // Display migration information
    let migrations = vec![migration1, migration2, migration3];

    println!("Created {} migrations:", migrations.len());
    for migration in &migrations {
        println!("\n  Version: {}", migration.version());
        println!("  Description: {}", migration.description());
        println!("  Checksum: {}", migration.checksum());
        println!("\n  Up SQL:");
        for line in migration.up_sql().lines().take(5) {
            println!("    {}", line);
        }
        if migration.up_sql().lines().count() > 5 {
            println!("    ...");
        }
        println!("\n  Down SQL:");
        for line in migration.down_sql().lines().take(3) {
            println!("    {}", line);
        }
        if migration.down_sql().lines().count() > 3 {
            println!("    ...");
        }
    }

    println!("\n\n2. Using migrations with MigrationRunner:");
    println!("   ```rust");
    println!("   use dev_engineeringlabs_rustboot_database::MigrationRunner;");
    println!();
    println!("   // Initialize migration runner");
    println!("   let db = /* your Database implementation */;");
    println!("   let runner = MigrationRunner::new(Box::new(db));");
    println!("   runner.init().await?;");
    println!();
    println!("   // Run all pending migrations");
    println!("   let executed = runner.migrate(&migrations).await?;");
    println!("   println!(\"Applied {{}} migrations\", executed.len());");
    println!();
    println!("   // Check migration status");
    println!("   let status = runner.get_status(&migrations).await?;");
    println!("   for record in status {{");
    println!("       println!(\"{{}} - {{:?}}\", record.version, record.status);");
    println!("   }}");
    println!();
    println!("   // Rollback last migration");
    println!("   runner.rollback(&migrations, 1).await?;");
    println!("   ```");

    println!("\n=== Migration Example Complete ===");
}
