// Package api contains the public interfaces and types for the feature flags module.
package api

import (
	"context"
)

// Flag represents a feature flag.
type Flag struct {
	Key          string
	Name         string
	Description  string
	Enabled      bool
	Variants     []Variant
	DefaultValue any
	Rules        []Rule
}

// Variant represents a flag variant.
type Variant struct {
	Name   string
	Value  any
	Weight int // For percentage-based rollouts (0-100)
}

// Rule represents a targeting rule.
type Rule struct {
	ID         string
	Conditions []Condition
	Variant    string
	Enabled    bool
}

// Condition represents a rule condition.
type Condition struct {
	Attribute string
	Operator  Operator
	Value     any
}

// Operator represents a comparison operator.
type Operator string

const (
	// OpEquals checks equality.
	OpEquals Operator = "eq"
	// OpNotEquals checks inequality.
	OpNotEquals Operator = "neq"
	// OpContains checks if value contains.
	OpContains Operator = "contains"
	// OpIn checks if value is in list.
	OpIn Operator = "in"
	// OpNotIn checks if value is not in list.
	OpNotIn Operator = "notIn"
	// OpGreaterThan checks greater than.
	OpGreaterThan Operator = "gt"
	// OpLessThan checks less than.
	OpLessThan Operator = "lt"
	// OpMatches checks regex match.
	OpMatches Operator = "matches"
)

// EvaluationContext contains context for flag evaluation.
type EvaluationContext struct {
	UserID     string
	Attributes map[string]any
}

// NewContext creates a new EvaluationContext.
func NewContext(userID string) *EvaluationContext {
	return &EvaluationContext{
		UserID:     userID,
		Attributes: make(map[string]any),
	}
}

// WithAttribute adds an attribute.
func (c *EvaluationContext) WithAttribute(key string, value any) *EvaluationContext {
	c.Attributes[key] = value
	return c
}

// EvaluationResult represents the result of flag evaluation.
type EvaluationResult struct {
	FlagKey   string
	Enabled   bool
	Value     any
	Variant   string
	Reason    string
}

// FeatureFlags is the interface for feature flag management.
type FeatureFlags interface {
	// IsEnabled checks if a flag is enabled.
	IsEnabled(ctx context.Context, flagKey string, evalCtx *EvaluationContext) bool

	// GetValue gets the flag value.
	GetValue(ctx context.Context, flagKey string, evalCtx *EvaluationContext) any

	// Evaluate evaluates a flag and returns detailed result.
	Evaluate(ctx context.Context, flagKey string, evalCtx *EvaluationContext) *EvaluationResult

	// SetFlag sets a flag.
	SetFlag(flag *Flag) error

	// GetFlag gets a flag.
	GetFlag(flagKey string) (*Flag, error)

	// DeleteFlag deletes a flag.
	DeleteFlag(flagKey string) error

	// AllFlags returns all flags.
	AllFlags() []*Flag
}

// FlagStore is the interface for flag storage.
type FlagStore interface {
	// Save saves a flag.
	Save(flag *Flag) error

	// Get gets a flag.
	Get(key string) (*Flag, error)

	// Delete deletes a flag.
	Delete(key string) error

	// All returns all flags.
	All() ([]*Flag, error)
}
