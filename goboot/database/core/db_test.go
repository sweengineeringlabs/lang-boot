package core

import (
	"testing"
)

func TestQueryBuilder_Basic(t *testing.T) {
	qb := NewQueryBuilder("SELECT * FROM users")
	query, args := qb.Build()

	if query != "SELECT * FROM users" {
		t.Errorf("Unexpected query: %s", query)
	}
	if len(args) != 0 {
		t.Error("Expected no args")
	}
}

func TestQueryBuilder_Where(t *testing.T) {
	qb := NewQueryBuilder("SELECT * FROM users")
	qb.Where("active = ?", true)

	query, args := qb.Build()

	expected := "SELECT * FROM users WHERE active = ?"
	if query != expected {
		t.Errorf("Expected '%s', got '%s'", expected, query)
	}
	if len(args) != 1 {
		t.Errorf("Expected 1 arg, got %d", len(args))
	}
	if args[0] != true {
		t.Error("Arg should be true")
	}
}

func TestQueryBuilder_MultipleWhere(t *testing.T) {
	qb := NewQueryBuilder("SELECT * FROM users")
	qb.Where("active = ?", true)
	qb.Where("age >= ?", 18)

	query, args := qb.Build()

	expected := "SELECT * FROM users WHERE active = ? AND age >= ?"
	if query != expected {
		t.Errorf("Expected '%s', got '%s'", expected, query)
	}
	if len(args) != 2 {
		t.Errorf("Expected 2 args, got %d", len(args))
	}
}

func TestQueryBuilder_OrderBy(t *testing.T) {
	qb := NewQueryBuilder("SELECT * FROM users")
	qb.OrderBy("created_at", "DESC")

	query, _ := qb.Build()

	expected := "SELECT * FROM users ORDER BY created_at DESC"
	if query != expected {
		t.Errorf("Expected '%s', got '%s'", expected, query)
	}
}

func TestQueryBuilder_Limit(t *testing.T) {
	qb := NewQueryBuilder("SELECT * FROM users")
	qb.Limit(10)

	query, _ := qb.Build()

	expected := "SELECT * FROM users LIMIT 10"
	if query != expected {
		t.Errorf("Expected '%s', got '%s'", expected, query)
	}
}

func TestQueryBuilder_Offset(t *testing.T) {
	qb := NewQueryBuilder("SELECT * FROM users")
	qb.Offset(20)

	query, _ := qb.Build()

	expected := "SELECT * FROM users OFFSET 20"
	if query != expected {
		t.Errorf("Expected '%s', got '%s'", expected, query)
	}
}

func TestQueryBuilder_Complete(t *testing.T) {
	qb := NewQueryBuilder("SELECT * FROM users")
	qb.Where("active = ?", true)
	qb.Where("role = ?", "admin")
	qb.OrderBy("name", "ASC")
	qb.Limit(10)
	qb.Offset(20)

	query, args := qb.Build()

	expected := "SELECT * FROM users WHERE active = ? AND role = ? ORDER BY name ASC LIMIT 10 OFFSET 20"
	if query != expected {
		t.Errorf("Expected '%s', got '%s'", expected, query)
	}
	if len(args) != 2 {
		t.Errorf("Expected 2 args, got %d", len(args))
	}
}

func TestQueryBuilder_Chaining(t *testing.T) {
	query, args := NewQueryBuilder("SELECT * FROM products").
		Where("category = ?", "electronics").
		Where("price <= ?", 1000).
		OrderBy("price", "ASC").
		Limit(20).
		Build()

	expected := "SELECT * FROM products WHERE category = ? AND price <= ? ORDER BY price ASC LIMIT 20"
	if query != expected {
		t.Errorf("Expected '%s', got '%s'", expected, query)
	}
	if len(args) != 2 {
		t.Errorf("Expected 2 args, got %d", len(args))
	}
}
