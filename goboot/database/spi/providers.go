// Package spi contains the Service Provider Interface for the database module.
package spi

import (
	"context"
	"database/sql"

	"dev.engineeringlabs/goboot/database/api"
)

// Driver is the interface for database drivers.
//
// Implement this for custom database drivers.
type Driver interface {
	// Name returns the driver name.
	Name() string

	// Open opens a database connection.
	Open(dsn string) (api.DB, error)

	// ParseDSN parses and validates a DSN.
	ParseDSN(dsn string) (map[string]string, error)
}

// Migrator is the interface for database migrations.
//
// Implement this for migration tools like golang-migrate.
//
// Example:
//
//	type GolangMigrate struct {
//	    migrationsPath string
//	}
//
//	func (m *GolangMigrate) Up(ctx context.Context, db api.DB) error {
//	    // Run migrations
//	}
type Migrator interface {
	// Up runs all pending migrations.
	Up(ctx context.Context, db api.DB) error

	// Down rolls back the last migration.
	Down(ctx context.Context, db api.DB) error

	// Version returns the current migration version.
	Version(ctx context.Context, db api.DB) (int, error)

	// Steps runs a specific number of migrations.
	Steps(ctx context.Context, db api.DB, n int) error

	// Goto migrates to a specific version.
	Goto(ctx context.Context, db api.DB, version int) error
}

// ConnectionPool is the interface for connection pool management.
type ConnectionPool interface {
	// Stats returns pool statistics.
	Stats() sql.DBStats

	// SetMaxOpenConns sets the maximum number of open connections.
	SetMaxOpenConns(n int)

	// SetMaxIdleConns sets the maximum number of idle connections.
	SetMaxIdleConns(n int)

	// SetConnMaxLifetime sets the maximum connection lifetime.
	SetConnMaxLifetime(d int)
}

// EntityMapper is the interface for mapping rows to entities.
//
// Implement this for custom ORM-like functionality.
type EntityMapper[T any] interface {
	// Map maps a row to an entity.
	Map(row api.Row) (*T, error)

	// MapAll maps rows to entities.
	MapAll(rows api.Rows) ([]*T, error)

	// TableName returns the table name.
	TableName() string

	// Columns returns the column names.
	Columns() []string
}

// QueryLogger is the interface for query logging.
type QueryLogger interface {
	// LogQuery logs a query execution.
	LogQuery(query string, args []any, duration int64, err error)
}
