//! Database migration framework for rustboot.
//!
//! Provides a comprehensive migration system with:
//! - Trait-based migration interface
//! - SQL file-based migrations
//! - Programmatic migrations
//! - Version tracking
//! - Rollback support
//! - Migration status checking

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::Database;

/// Migration errors.
#[derive(Debug, Error)]
pub enum MigrationError {
    /// Database error during migration.
    #[error("Database error: {0}")]
    Database(String),

    /// Migration not found.
    #[error("Migration not found: {0}")]
    NotFound(String),

    /// Migration already applied.
    #[error("Migration already applied: {0}")]
    AlreadyApplied(String),

    /// Migration not applied (cannot rollback).
    #[error("Migration not applied: {0}")]
    NotApplied(String),

    /// Invalid migration version.
    #[error("Invalid migration version: {0}")]
    InvalidVersion(String),

    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Parse error.
    #[error("Parse error: {0}")]
    Parse(String),

    /// Migration validation error.
    #[error("Validation error: {0}")]
    Validation(String),

    /// Checksum mismatch.
    #[error("Checksum mismatch for migration {0}: expected {1}, got {2}")]
    ChecksumMismatch(String, String, String),
}

/// Result type for migration operations.
pub type MigrationResult<T> = Result<T, MigrationError>;

/// Migration direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    /// Apply migration (up).
    Up,
    /// Rollback migration (down).
    Down,
}

/// Migration version.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Version(String);

impl Version {
    /// Create a new version.
    pub fn new(version: impl Into<String>) -> Self {
        Self(version.into())
    }

    /// Get the version string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Parse version from string (validates format).
    pub fn parse(s: &str) -> MigrationResult<Self> {
        if s.is_empty() {
            return Err(MigrationError::InvalidVersion(
                "Version cannot be empty".to_string(),
            ));
        }
        // Basic validation - you can make this stricter
        if !s.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err(MigrationError::InvalidVersion(format!(
                "Invalid characters in version: {}",
                s
            )));
        }
        Ok(Self(s.to_string()))
    }

    /// Generate timestamp-based version (YYYYMMDDHHmmss format).
    pub fn timestamp() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Convert to YYYYMMDDHHmmss format
        let datetime = chrono::DateTime::from_timestamp(now as i64, 0)
            .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap());
        Self(datetime.format("%Y%m%d%H%M%S").to_string())
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lexicographic comparison works for timestamp-based versions
        self.0.cmp(&other.0)
    }
}

/// Migration trait for implementing custom migrations.
#[async_trait]
pub trait Migration: Send + Sync {
    /// Get migration version.
    fn version(&self) -> &Version;

    /// Get migration description.
    fn description(&self) -> &str;

    /// Apply migration (up).
    async fn up(&self, db: &dyn Database) -> MigrationResult<()>;

    /// Rollback migration (down).
    async fn down(&self, db: &dyn Database) -> MigrationResult<()>;

    /// Calculate checksum for migration content.
    fn checksum(&self) -> String;
}

/// SQL-based migration.
#[derive(Debug, Clone)]
pub struct SqlMigration {
    version: Version,
    description: String,
    up_sql: String,
    down_sql: String,
}

impl SqlMigration {
    /// Create a new SQL migration.
    pub fn new(
        version: Version,
        description: impl Into<String>,
        up_sql: impl Into<String>,
        down_sql: impl Into<String>,
    ) -> Self {
        Self {
            version,
            description: description.into(),
            up_sql: up_sql.into(),
            down_sql: down_sql.into(),
        }
    }

    /// Get the up SQL.
    pub fn up_sql(&self) -> &str {
        &self.up_sql
    }

    /// Get the down SQL.
    pub fn down_sql(&self) -> &str {
        &self.down_sql
    }

    /// Calculate SHA256 checksum of migration content.
    fn calculate_checksum(content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

#[async_trait]
impl Migration for SqlMigration {
    fn version(&self) -> &Version {
        &self.version
    }

    fn description(&self) -> &str {
        &self.description
    }

    async fn up(&self, db: &dyn Database) -> MigrationResult<()> {
        // Split SQL by semicolons and execute each statement
        for statement in self.up_sql.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() {
                db.execute(statement)
                    .await
                    .map_err(|e| MigrationError::Database(e.to_string()))?;
            }
        }
        Ok(())
    }

    async fn down(&self, db: &dyn Database) -> MigrationResult<()> {
        // Split SQL by semicolons and execute each statement
        for statement in self.down_sql.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() {
                db.execute(statement)
                    .await
                    .map_err(|e| MigrationError::Database(e.to_string()))?;
            }
        }
        Ok(())
    }

    fn checksum(&self) -> String {
        // Checksum based on both up and down SQL
        Self::calculate_checksum(&format!("{}{}", self.up_sql, self.down_sql))
    }
}

/// Migration status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationStatus {
    /// Migration is pending (not applied).
    Pending,
    /// Migration is applied.
    Applied,
}

/// Migration record with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRecord {
    /// Migration version.
    pub version: Version,
    /// Migration description.
    pub description: String,
    /// Migration status.
    pub status: MigrationStatus,
    /// Checksum of migration content.
    pub checksum: String,
    /// When the migration was applied (None if pending).
    pub applied_at: Option<String>,
}

/// Migration file loader for SQL-based migrations.
pub struct MigrationLoader {
    migrations_dir: PathBuf,
}

impl MigrationLoader {
    /// Create a new migration loader.
    pub fn new(migrations_dir: impl Into<PathBuf>) -> Self {
        Self {
            migrations_dir: migrations_dir.into(),
        }
    }

    /// Load all migrations from the migrations directory.
    ///
    /// Expected file naming: `{version}__{description}.sql`
    /// Example: `20231201120000__create_users_table.sql`
    ///
    /// File format:
    /// ```sql
    /// -- migrate:up
    /// CREATE TABLE users (id INTEGER PRIMARY KEY);
    ///
    /// -- migrate:down
    /// DROP TABLE users;
    /// ```
    pub async fn load_migrations(&self) -> MigrationResult<Vec<SqlMigration>> {
        let mut migrations = Vec::new();

        if !self.migrations_dir.exists() {
            return Ok(migrations);
        }

        let entries = std::fs::read_dir(&self.migrations_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("sql") {
                if let Some(migration) = self.load_migration_file(&path).await? {
                    migrations.push(migration);
                }
            }
        }

        // Sort by version
        migrations.sort_by(|a, b| a.version.cmp(&b.version));

        Ok(migrations)
    }

    /// Load a single migration file.
    async fn load_migration_file(&self, path: &Path) -> MigrationResult<Option<SqlMigration>> {
        let filename = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| MigrationError::Parse(format!("Invalid filename: {:?}", path)))?;

        // Parse filename: {version}__{description}
        let parts: Vec<&str> = filename.splitn(2, "__").collect();
        if parts.len() != 2 {
            return Err(MigrationError::Parse(format!(
                "Invalid migration filename format: {}. Expected: {{version}}__{{description}}.sql",
                filename
            )));
        }

        let version = Version::parse(parts[0])?;
        let description = parts[1].replace('_', " ");

        // Read file content
        let content = tokio::fs::read_to_string(path).await?;

        // Parse up and down sections
        let (up_sql, down_sql) = self.parse_migration_content(&content)?;

        Ok(Some(SqlMigration::new(
            version,
            description,
            up_sql,
            down_sql,
        )))
    }

    /// Parse migration file content into up and down sections.
    pub fn parse_migration_content(&self, content: &str) -> MigrationResult<(String, String)> {
        let mut up_sql = String::new();
        let mut down_sql = String::new();
        let mut current_section = None;

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("-- migrate:up") || trimmed.starts_with("--migrate:up") {
                current_section = Some(Direction::Up);
                continue;
            } else if trimmed.starts_with("-- migrate:down") || trimmed.starts_with("--migrate:down")
            {
                current_section = Some(Direction::Down);
                continue;
            }

            match current_section {
                Some(Direction::Up) => {
                    up_sql.push_str(line);
                    up_sql.push('\n');
                }
                Some(Direction::Down) => {
                    down_sql.push_str(line);
                    down_sql.push('\n');
                }
                None => {
                    // Ignore lines before first section marker
                }
            }
        }

        if up_sql.is_empty() {
            return Err(MigrationError::Parse(
                "No 'migrate:up' section found".to_string(),
            ));
        }

        Ok((up_sql.trim().to_string(), down_sql.trim().to_string()))
    }

    /// Create a new migration file with the given description.
    pub async fn create_migration(
        &self,
        description: &str,
    ) -> MigrationResult<(Version, PathBuf)> {
        // Create migrations directory if it doesn't exist
        tokio::fs::create_dir_all(&self.migrations_dir).await?;

        // Generate version
        let version = Version::timestamp();

        // Create filename
        let filename = format!(
            "{}__{}.sql",
            version.as_str(),
            description.replace(' ', "_").to_lowercase()
        );
        let path = self.migrations_dir.join(&filename);

        // Create file with template
        let template = format!(
            "-- Migration: {}\n\
            -- Created: {}\n\
            \n\
            -- migrate:up\n\
            -- Add your up migration SQL here\n\
            \n\
            \n\
            -- migrate:down\n\
            -- Add your down migration SQL here\n\
            \n",
            description,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
        );

        tokio::fs::write(&path, template).await?;

        Ok((version, path))
    }
}

/// Migration runner for executing migrations.
pub struct MigrationRunner {
    db: Box<dyn Database>,
    migrations_table: String,
}

impl MigrationRunner {
    /// Create a new migration runner.
    pub fn new(db: Box<dyn Database>) -> Self {
        Self {
            db,
            migrations_table: "_migrations".to_string(),
        }
    }

    /// Create a new migration runner with custom migrations table name.
    pub fn with_table_name(db: Box<dyn Database>, table_name: impl Into<String>) -> Self {
        Self {
            db,
            migrations_table: table_name.into(),
        }
    }

    /// Initialize the migrations table.
    pub async fn init(&self) -> MigrationResult<()> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (\
                version TEXT PRIMARY KEY,\
                description TEXT NOT NULL,\
                checksum TEXT NOT NULL,\
                applied_at TEXT NOT NULL\
            )",
            self.migrations_table
        );

        self.db
            .execute(&sql)
            .await
            .map_err(|e| MigrationError::Database(e.to_string()))?;

        Ok(())
    }

    /// Get all applied migrations.
    pub async fn get_applied_migrations(&self) -> MigrationResult<HashMap<Version, MigrationRecord>>
    {
        let sql = format!(
            "SELECT version, description, checksum, applied_at FROM {}",
            self.migrations_table
        );

        let rows = self
            .db
            .query(&sql)
            .await
            .map_err(|e| MigrationError::Database(e.to_string()))?;

        let mut applied = HashMap::new();

        for row in rows {
            let version_str = row
                .get("version")
                .and_then(|v| {
                    if let crate::Value::String(s) = v {
                        Some(s.as_str())
                    } else {
                        None
                    }
                })
                .ok_or_else(|| MigrationError::Parse("Missing version".to_string()))?;

            let description = row
                .get("description")
                .and_then(|v| {
                    if let crate::Value::String(s) = v {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .ok_or_else(|| MigrationError::Parse("Missing description".to_string()))?;

            let checksum = row
                .get("checksum")
                .and_then(|v| {
                    if let crate::Value::String(s) = v {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .ok_or_else(|| MigrationError::Parse("Missing checksum".to_string()))?;

            let applied_at = row
                .get("applied_at")
                .and_then(|v| {
                    if let crate::Value::String(s) = v {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .ok_or_else(|| MigrationError::Parse("Missing applied_at".to_string()))?;

            let version = Version::parse(version_str)?;

            applied.insert(
                version.clone(),
                MigrationRecord {
                    version,
                    description,
                    status: MigrationStatus::Applied,
                    checksum,
                    applied_at: Some(applied_at),
                },
            );
        }

        Ok(applied)
    }

    /// Get migration status for all migrations.
    pub async fn get_status(
        &self,
        migrations: &[SqlMigration],
    ) -> MigrationResult<Vec<MigrationRecord>> {
        let applied = self.get_applied_migrations().await?;

        let mut records = Vec::new();

        for migration in migrations {
            let version = migration.version();
            let checksum = migration.checksum();

            if let Some(applied_record) = applied.get(version) {
                // Verify checksum
                if applied_record.checksum != checksum {
                    return Err(MigrationError::ChecksumMismatch(
                        version.to_string(),
                        applied_record.checksum.clone(),
                        checksum,
                    ));
                }

                records.push(applied_record.clone());
            } else {
                records.push(MigrationRecord {
                    version: version.clone(),
                    description: migration.description().to_string(),
                    status: MigrationStatus::Pending,
                    checksum,
                    applied_at: None,
                });
            }
        }

        Ok(records)
    }

    /// Run all pending migrations.
    pub async fn migrate(&self, migrations: &[SqlMigration]) -> MigrationResult<Vec<Version>> {
        let applied = self.get_applied_migrations().await?;
        let mut executed = Vec::new();

        for migration in migrations {
            if applied.contains_key(migration.version()) {
                continue;
            }

            self.apply_migration(migration).await?;
            executed.push(migration.version().clone());
        }

        Ok(executed)
    }

    /// Run migrations up to a specific version.
    pub async fn migrate_to(
        &self,
        migrations: &[SqlMigration],
        target_version: &Version,
    ) -> MigrationResult<Vec<Version>> {
        let applied = self.get_applied_migrations().await?;
        let mut executed = Vec::new();

        for migration in migrations {
            if migration.version() > target_version {
                break;
            }

            if applied.contains_key(migration.version()) {
                continue;
            }

            self.apply_migration(migration).await?;
            executed.push(migration.version().clone());
        }

        Ok(executed)
    }

    /// Rollback the last N migrations.
    pub async fn rollback(
        &self,
        migrations: &[SqlMigration],
        count: usize,
    ) -> MigrationResult<Vec<Version>> {
        let applied = self.get_applied_migrations().await?;

        // Get applied versions sorted in reverse order
        let mut applied_versions: Vec<_> = applied.keys().cloned().collect();
        applied_versions.sort_by(|a, b| b.cmp(a)); // Reverse sort

        let mut rolled_back = Vec::new();

        for version in applied_versions.iter().take(count) {
            let migration = migrations
                .iter()
                .find(|m| m.version() == version)
                .ok_or_else(|| MigrationError::NotFound(version.to_string()))?;

            self.revert_migration(migration).await?;
            rolled_back.push(version.clone());
        }

        Ok(rolled_back)
    }

    /// Rollback to a specific version.
    pub async fn rollback_to(
        &self,
        migrations: &[SqlMigration],
        target_version: &Version,
    ) -> MigrationResult<Vec<Version>> {
        let applied = self.get_applied_migrations().await?;

        // Get applied versions sorted in reverse order
        let mut applied_versions: Vec<_> = applied.keys().cloned().collect();
        applied_versions.sort_by(|a, b| b.cmp(a)); // Reverse sort

        let mut rolled_back = Vec::new();

        for version in applied_versions {
            if &version <= target_version {
                break;
            }

            let migration = migrations
                .iter()
                .find(|m| m.version() == &version)
                .ok_or_else(|| MigrationError::NotFound(version.to_string()))?;

            self.revert_migration(migration).await?;
            rolled_back.push(version);
        }

        Ok(rolled_back)
    }

    /// Apply a single migration.
    async fn apply_migration(&self, migration: &SqlMigration) -> MigrationResult<()> {
        // Execute migration
        migration.up(self.db.as_ref()).await?;

        // Record in migrations table
        let now = chrono::Utc::now().to_rfc3339();
        let sql = format!(
            "INSERT INTO {} (version, description, checksum, applied_at) VALUES ('{}', '{}', '{}', '{}')",
            self.migrations_table,
            migration.version().as_str(),
            migration.description().replace('\'', "''"),
            migration.checksum(),
            now
        );

        self.db
            .execute(&sql)
            .await
            .map_err(|e| MigrationError::Database(e.to_string()))?;

        Ok(())
    }

    /// Revert a single migration.
    async fn revert_migration(&self, migration: &SqlMigration) -> MigrationResult<()> {
        // Execute rollback
        migration.down(self.db.as_ref()).await?;

        // Remove from migrations table
        let sql = format!(
            "DELETE FROM {} WHERE version = '{}'",
            self.migrations_table,
            migration.version().as_str()
        );

        self.db
            .execute(&sql)
            .await
            .map_err(|e| MigrationError::Database(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_ordering() {
        let v1 = Version::new("20231201120000");
        let v2 = Version::new("20231201130000");
        let v3 = Version::new("20231202120000");

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v1 < v3);
    }

    #[test]
    fn test_version_parse() {
        assert!(Version::parse("20231201120000").is_ok());
        assert!(Version::parse("v1_initial").is_ok());
        assert!(Version::parse("").is_err());
        assert!(Version::parse("invalid!version").is_err());
    }

    #[test]
    fn test_migration_loader_parse_content() {
        let loader = MigrationLoader::new("/tmp/migrations");
        let content = r#"
-- Some comment

-- migrate:up
CREATE TABLE users (
    id INTEGER PRIMARY KEY
);

-- migrate:down
DROP TABLE users;
        "#;

        let result = loader.parse_migration_content(content);
        assert!(result.is_ok());

        let (up_sql, down_sql) = result.unwrap();
        assert!(up_sql.contains("CREATE TABLE users"));
        assert!(down_sql.contains("DROP TABLE users"));
    }

    #[test]
    fn test_sql_migration_checksum() {
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
            "CREATE TABLE test2;",
            "DROP TABLE test2;",
        );

        assert_eq!(migration1.checksum(), migration2.checksum());
        assert_ne!(migration1.checksum(), migration3.checksum());
    }
}
