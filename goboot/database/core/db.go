// Package core contains the implementation details for the database module.
package core

import (
	"context"
	"database/sql"
	"fmt"
	"time"

	"dev.engineeringlabs/goboot/database/api"
)

// SqlDB wraps the standard sql.DB.
type SqlDB struct {
	db *sql.DB
}

// Open opens a database connection.
func Open(config api.DBConfig) (*SqlDB, error) {
	db, err := sql.Open(config.Driver, config.DSN)
	if err != nil {
		return nil, fmt.Errorf("failed to open database: %w", err)
	}

	// Apply pool configuration
	db.SetMaxOpenConns(config.Pool.MaxOpenConns)
	db.SetMaxIdleConns(config.Pool.MaxIdleConns)
	db.SetConnMaxLifetime(config.Pool.ConnMaxLifetime)
	db.SetConnMaxIdleTime(config.Pool.ConnMaxIdleTime)

	return &SqlDB{db: db}, nil
}

// OpenWithDSN opens a database connection with just DSN.
func OpenWithDSN(driver, dsn string) (*SqlDB, error) {
	return Open(api.DBConfig{
		Driver: driver,
		DSN:    dsn,
		Pool:   api.DefaultPoolConfig(),
	})
}

// Exec executes a query without returning rows.
func (d *SqlDB) Exec(ctx context.Context, query string, args ...any) (api.Result, error) {
	return d.db.ExecContext(ctx, query, args...)
}

// Query executes a query that returns rows.
func (d *SqlDB) Query(ctx context.Context, query string, args ...any) (api.Rows, error) {
	return d.db.QueryContext(ctx, query, args...)
}

// QueryRow executes a query that returns a single row.
func (d *SqlDB) QueryRow(ctx context.Context, query string, args ...any) api.Row {
	return d.db.QueryRowContext(ctx, query, args...)
}

// Begin starts a transaction.
func (d *SqlDB) Begin(ctx context.Context) (api.Tx, error) {
	tx, err := d.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	return &SqlTx{tx: tx}, nil
}

// Close closes the database connection.
func (d *SqlDB) Close() error {
	return d.db.Close()
}

// Ping verifies the database connection.
func (d *SqlDB) Ping(ctx context.Context) error {
	return d.db.PingContext(ctx)
}

// DB returns the underlying sql.DB.
func (d *SqlDB) DB() *sql.DB {
	return d.db
}

// SqlTx wraps the standard sql.Tx.
type SqlTx struct {
	tx *sql.Tx
}

// Exec executes a query without returning rows.
func (t *SqlTx) Exec(ctx context.Context, query string, args ...any) (api.Result, error) {
	return t.tx.ExecContext(ctx, query, args...)
}

// Query executes a query that returns rows.
func (t *SqlTx) Query(ctx context.Context, query string, args ...any) (api.Rows, error) {
	return t.tx.QueryContext(ctx, query, args...)
}

// QueryRow executes a query that returns a single row.
func (t *SqlTx) QueryRow(ctx context.Context, query string, args ...any) api.Row {
	return t.tx.QueryRowContext(ctx, query, args...)
}

// Commit commits the transaction.
func (t *SqlTx) Commit() error {
	return t.tx.Commit()
}

// Rollback aborts the transaction.
func (t *SqlTx) Rollback() error {
	return t.tx.Rollback()
}

// WithTransaction executes a function within a transaction.
func WithTransaction(ctx context.Context, db api.DB, fn func(tx api.Tx) error) error {
	tx, err := db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}

	defer func() {
		if p := recover(); p != nil {
			tx.Rollback()
			panic(p)
		}
	}()

	if err := fn(tx); err != nil {
		if rbErr := tx.Rollback(); rbErr != nil {
			return fmt.Errorf("tx failed: %v, rollback failed: %w", err, rbErr)
		}
		return err
	}

	if err := tx.Commit(); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

// HealthCheck checks database health with a timeout.
func HealthCheck(ctx context.Context, db api.DB, timeout time.Duration) error {
	ctx, cancel := context.WithTimeout(ctx, timeout)
	defer cancel()

	if err := db.Ping(ctx); err != nil {
		return fmt.Errorf("database health check failed: %w", err)
	}
	return nil
}

// QueryBuilder helps build SQL queries.
type QueryBuilder struct {
	query  string
	args   []any
	where  []string
	order  string
	limit  int
	offset int
}

// NewQueryBuilder creates a new QueryBuilder.
func NewQueryBuilder(baseQuery string) *QueryBuilder {
	return &QueryBuilder{
		query: baseQuery,
		args:  make([]any, 0),
		where: make([]string, 0),
	}
}

// Where adds a WHERE clause.
func (b *QueryBuilder) Where(condition string, args ...any) *QueryBuilder {
	b.where = append(b.where, condition)
	b.args = append(b.args, args...)
	return b
}

// OrderBy sets the ORDER BY clause.
func (b *QueryBuilder) OrderBy(column, direction string) *QueryBuilder {
	b.order = fmt.Sprintf("%s %s", column, direction)
	return b
}

// Limit sets the LIMIT clause.
func (b *QueryBuilder) Limit(limit int) *QueryBuilder {
	b.limit = limit
	return b
}

// Offset sets the OFFSET clause.
func (b *QueryBuilder) Offset(offset int) *QueryBuilder {
	b.offset = offset
	return b
}

// Build builds the final query.
func (b *QueryBuilder) Build() (string, []any) {
	query := b.query

	if len(b.where) > 0 {
		query += " WHERE " + join(b.where, " AND ")
	}

	if b.order != "" {
		query += " ORDER BY " + b.order
	}

	if b.limit > 0 {
		query += fmt.Sprintf(" LIMIT %d", b.limit)
	}

	if b.offset > 0 {
		query += fmt.Sprintf(" OFFSET %d", b.offset)
	}

	return query, b.args
}

func join(parts []string, sep string) string {
	if len(parts) == 0 {
		return ""
	}
	result := parts[0]
	for i := 1; i < len(parts); i++ {
		result += sep + parts[i]
	}
	return result
}
