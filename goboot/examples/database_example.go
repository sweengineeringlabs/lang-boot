//go:build ignore

// Package main demonstrates the database module usage.
package main

import (
	"context"
	"fmt"

	"dev.engineeringlabs/goboot/database"
)

func main() {
	fmt.Println("=== Goboot Database Module Example ===\n")
	ctx := context.Background()

	// Note: This example requires a database driver to be imported.
	// In a real application, you would:
	// import _ "github.com/lib/pq"  // PostgreSQL
	// import _ "github.com/go-sql-driver/mysql"  // MySQL
	// import _ "github.com/mattn/go-sqlite3"  // SQLite

	fmt.Println("1. Database Configuration:")
	config := database.DBConfig{
		Driver: "postgres",
		DSN:    "postgres://user:pass@localhost:5432/mydb?sslmode=disable",
		Pool:   database.DefaultPoolConfig(),
	}
	fmt.Printf("   Driver: %s\n", config.Driver)
	fmt.Printf("   MaxOpenConns: %d\n", config.Pool.MaxOpenConns)
	fmt.Printf("   MaxIdleConns: %d\n", config.Pool.MaxIdleConns)

	// Example: Opening a connection (commented out - requires actual database)
	// db, err := database.Open(config)
	// if err != nil {
	//     log.Fatal(err)
	// }
	// defer db.Close()

	fmt.Println("\n2. Query Builder:")
	qb := database.NewQueryBuilder("SELECT * FROM users")
	qb.Where("active = ?", true).
		Where("age >= ?", 18).
		OrderBy("created_at", "DESC").
		Limit(10).
		Offset(0)

	query, args := qb.Build()
	fmt.Printf("   Query: %s\n", query)
	fmt.Printf("   Args: %v\n", args)

	fmt.Println("\n3. Complex Query:")
	qb2 := database.NewQueryBuilder("SELECT u.*, COUNT(o.id) as order_count FROM users u LEFT JOIN orders o ON u.id = o.user_id")
	qb2.Where("u.status = ?", "active").
		Where("u.created_at > ?", "2024-01-01").
		OrderBy("order_count", "DESC").
		Limit(20)

	query2, args2 := qb2.Build()
	fmt.Printf("   Query: %s\n", query2)
	fmt.Printf("   Args: %v\n", args2)

	fmt.Println("\n4. Transaction Pattern:")
	fmt.Println("   // Using WithTransaction helper:")
	fmt.Println("   err := database.WithTransaction(ctx, db, func(tx database.Tx) error {")
	fmt.Println("       _, err := tx.Exec(ctx, \"INSERT INTO users (name) VALUES ($1)\", \"John\")")
	fmt.Println("       if err != nil {")
	fmt.Println("           return err // Automatic rollback")
	fmt.Println("       }")
	fmt.Println("       _, err = tx.Exec(ctx, \"INSERT INTO profiles (user_id) VALUES ($1)\", 1)")
	fmt.Println("       return err // Commits if nil, rollbacks otherwise")
	fmt.Println("   })")

	fmt.Println("\n5. Repository Pattern:")
	fmt.Println("   // Define a repository interface:")
	fmt.Println("   type UserRepository interface {")
	fmt.Println("       database.Repository[User, int64]")
	fmt.Println("       FindByEmail(ctx context.Context, email string) (*User, error)")
	fmt.Println("       FindActive(ctx context.Context) ([]*User, error)")
	fmt.Println("   }")

	fmt.Println("\n6. Pagination:")
	pageReq := database.DefaultPageRequest()
	pageReq.Page = 0
	pageReq.PageSize = 20
	pageReq.SortBy = "created_at"
	pageReq.SortDir = "DESC"

	fmt.Printf("   Page: %d\n", pageReq.Page)
	fmt.Printf("   PageSize: %d\n", pageReq.PageSize)
	fmt.Printf("   SortBy: %s %s\n", pageReq.SortBy, pageReq.SortDir)

	fmt.Println("\n7. Health Check:")
	fmt.Println("   // Check database connectivity with timeout:")
	fmt.Println("   err := database.HealthCheck(ctx, db, 5*time.Second)")
	_ = ctx // Prevent unused variable warning
}
