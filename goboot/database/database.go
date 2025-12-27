// Package database provides database utilities for the goboot framework.
//
// This module provides:
//   - API layer: DB, Tx, Repository interfaces, pagination
//   - Core layer: SqlDB wrapper, transaction helpers, QueryBuilder
//   - SPI layer: Driver, Migrator, EntityMapper interfaces
//
// Example:
//
//	import (
//	    "dev.engineeringlabs/goboot/database"
//	    _ "github.com/lib/pq" // PostgreSQL driver
//	)
//
//	db, err := database.OpenWithDSN("postgres", "postgres://localhost/mydb")
//	if err != nil {
//	    log.Fatal(err)
//	}
//	defer db.Close()
//
//	// Execute within transaction
//	err = database.WithTransaction(ctx, db, func(tx database.Tx) error {
//	    _, err := tx.Exec(ctx, "INSERT INTO users (name) VALUES ($1)", "John")
//	    return err
//	})
//
//	// Use query builder
//	qb := database.NewQueryBuilder("SELECT * FROM users")
//	qb.Where("active = ?", true).OrderBy("created_at", "DESC").Limit(10)
//	query, args := qb.Build()
package database

import (
	"dev.engineeringlabs/goboot/database/api"
	"dev.engineeringlabs/goboot/database/core"
	"dev.engineeringlabs/goboot/database/spi"
)

// Re-export API types
type (
	// DB is the interface for database connections.
	DB = api.DB
	// Tx is the interface for database transactions.
	Tx = api.Tx
	// Result is the interface for query results.
	Result = api.Result
	// Rows is the interface for query result rows.
	Rows = api.Rows
	// Row is the interface for a single row.
	Row = api.Row
	// PoolConfig configures the connection pool.
	PoolConfig = api.PoolConfig
	// DBConfig configures the database connection.
	DBConfig = api.DBConfig
	// Repository is the base interface for repositories.
	Repository[T any, ID any] interface {
		api.Repository[T, ID]
	}
	// PageRequest represents a pagination request.
	PageRequest = api.PageRequest
	// PageResult represents a paginated result.
	PageResult[T any] struct {
		api.PageResult[T]
	}
	// PaginatedRepository extends Repository with pagination.
	PaginatedRepository[T any, ID any] interface {
		api.PaginatedRepository[T, ID]
	}
)

// Re-export API functions
var (
	DefaultPoolConfig   = api.DefaultPoolConfig
	DefaultPageRequest  = api.DefaultPageRequest
)

// Re-export Core types
type (
	// SqlDB wraps the standard sql.DB.
	SqlDB = core.SqlDB
	// SqlTx wraps the standard sql.Tx.
	SqlTx = core.SqlTx
	// QueryBuilder helps build SQL queries.
	QueryBuilder = core.QueryBuilder
)

// Re-export Core functions
var (
	Open            = core.Open
	OpenWithDSN     = core.OpenWithDSN
	WithTransaction = core.WithTransaction
	HealthCheck     = core.HealthCheck
	NewQueryBuilder = core.NewQueryBuilder
)

// Re-export SPI types
type (
	// Driver is the interface for database drivers.
	Driver = spi.Driver
	// Migrator is the interface for database migrations.
	Migrator = spi.Migrator
	// ConnectionPool is the interface for connection pool management.
	ConnectionPool = spi.ConnectionPool
	// EntityMapper is the interface for mapping rows to entities.
	EntityMapper[T any] interface {
		spi.EntityMapper[T]
	}
	// QueryLogger is the interface for query logging.
	QueryLogger = spi.QueryLogger
)
