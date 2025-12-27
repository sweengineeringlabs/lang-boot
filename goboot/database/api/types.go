// Package api contains the public interfaces and types for the database module.
package api

import (
	"context"
	"time"
)

// DB is the interface for database connections.
type DB interface {
	// Exec executes a query without returning rows.
	Exec(ctx context.Context, query string, args ...any) (Result, error)

	// Query executes a query that returns rows.
	Query(ctx context.Context, query string, args ...any) (Rows, error)

	// QueryRow executes a query that returns a single row.
	QueryRow(ctx context.Context, query string, args ...any) Row

	// Begin starts a transaction.
	Begin(ctx context.Context) (Tx, error)

	// Close closes the database connection.
	Close() error

	// Ping verifies the database connection.
	Ping(ctx context.Context) error
}

// Tx is the interface for database transactions.
type Tx interface {
	// Exec executes a query without returning rows.
	Exec(ctx context.Context, query string, args ...any) (Result, error)

	// Query executes a query that returns rows.
	Query(ctx context.Context, query string, args ...any) (Rows, error)

	// QueryRow executes a query that returns a single row.
	QueryRow(ctx context.Context, query string, args ...any) Row

	// Commit commits the transaction.
	Commit() error

	// Rollback aborts the transaction.
	Rollback() error
}

// Result is the interface for query results.
type Result interface {
	// LastInsertId returns the last inserted ID.
	LastInsertId() (int64, error)

	// RowsAffected returns the number of rows affected.
	RowsAffected() (int64, error)
}

// Rows is the interface for query result rows.
type Rows interface {
	// Next advances to the next row.
	Next() bool

	// Scan copies the columns into dest.
	Scan(dest ...any) error

	// Columns returns the column names.
	Columns() ([]string, error)

	// Close closes the rows.
	Close() error

	// Err returns any error that occurred.
	Err() error
}

// Row is the interface for a single row.
type Row interface {
	// Scan copies the columns into dest.
	Scan(dest ...any) error
}

// PoolConfig configures the connection pool.
type PoolConfig struct {
	// MaxOpenConns is the maximum number of open connections.
	MaxOpenConns int
	// MaxIdleConns is the maximum number of idle connections.
	MaxIdleConns int
	// ConnMaxLifetime is the maximum lifetime of a connection.
	ConnMaxLifetime time.Duration
	// ConnMaxIdleTime is the maximum idle time of a connection.
	ConnMaxIdleTime time.Duration
}

// DefaultPoolConfig returns a default pool configuration.
func DefaultPoolConfig() PoolConfig {
	return PoolConfig{
		MaxOpenConns:    10,
		MaxIdleConns:    5,
		ConnMaxLifetime: time.Hour,
		ConnMaxIdleTime: 10 * time.Minute,
	}
}

// DBConfig configures the database connection.
type DBConfig struct {
	// Driver is the database driver name.
	Driver string
	// DSN is the data source name.
	DSN string
	// Pool is the pool configuration.
	Pool PoolConfig
}

// Repository is the base interface for repositories.
type Repository[T any, ID any] interface {
	// FindByID finds an entity by ID.
	FindByID(ctx context.Context, id ID) (*T, error)

	// FindAll finds all entities.
	FindAll(ctx context.Context) ([]*T, error)

	// Save saves an entity.
	Save(ctx context.Context, entity *T) error

	// Delete deletes an entity by ID.
	Delete(ctx context.Context, id ID) error

	// Exists checks if an entity exists.
	Exists(ctx context.Context, id ID) (bool, error)

	// Count returns the total count.
	Count(ctx context.Context) (int64, error)
}

// PageRequest represents a pagination request.
type PageRequest struct {
	Page     int
	PageSize int
	SortBy   string
	SortDir  string
}

// DefaultPageRequest returns a default page request.
func DefaultPageRequest() PageRequest {
	return PageRequest{
		Page:     0,
		PageSize: 20,
		SortDir:  "ASC",
	}
}

// PageResult represents a paginated result.
type PageResult[T any] struct {
	Content       []*T
	TotalElements int64
	TotalPages    int
	Page          int
	PageSize      int
	HasNext       bool
	HasPrevious   bool
}

// PaginatedRepository extends Repository with pagination.
type PaginatedRepository[T any, ID any] interface {
	Repository[T, ID]

	// FindAllPaginated finds entities with pagination.
	FindAllPaginated(ctx context.Context, request PageRequest) (*PageResult[T], error)
}
