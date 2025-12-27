// Package featureflags provides feature flag utilities for the goboot framework.
//
// This module provides:
//   - API layer: Flag, Rule, Condition, FeatureFlags interface
//   - Core layer: InMemoryFeatureFlags, rule evaluation, percentage rollouts
//
// Example:
//
//	import "dev.engineeringlabs/goboot/featureflags"
//
//	ff := featureflags.NewFeatureFlags()
//
//	// Simple flag
//	ff.SetFlag(featureflags.SimpleFlag("dark_mode", true))
//
//	// Percentage rollout
//	ff.SetFlag(featureflags.PercentageFlag("new_checkout", 25))
//
//	// Check flag
//	ctx := featureflags.NewContext("user-123").
//	    WithAttribute("plan", "premium")
//
//	if ff.IsEnabled(context.Background(), "dark_mode", ctx) {
//	    // Feature is enabled
//	}
package featureflags

import (
	"dev.engineeringlabs/goboot/featureflags/api"
	"dev.engineeringlabs/goboot/featureflags/core"
)

// Re-export API types
type (
	// Flag represents a feature flag.
	Flag = api.Flag
	// Variant represents a flag variant.
	Variant = api.Variant
	// Rule represents a targeting rule.
	Rule = api.Rule
	// Condition represents a rule condition.
	Condition = api.Condition
	// Operator represents a comparison operator.
	Operator = api.Operator
	// EvaluationContext contains context for flag evaluation.
	EvaluationContext = api.EvaluationContext
	// EvaluationResult represents the result of flag evaluation.
	EvaluationResult = api.EvaluationResult
	// FeatureFlags is the interface for feature flag management.
	FeatureFlags = api.FeatureFlags
	// FlagStore is the interface for flag storage.
	FlagStore = api.FlagStore
)

// Re-export API constants
const (
	OpEquals      = api.OpEquals
	OpNotEquals   = api.OpNotEquals
	OpContains    = api.OpContains
	OpIn          = api.OpIn
	OpNotIn       = api.OpNotIn
	OpGreaterThan = api.OpGreaterThan
	OpLessThan    = api.OpLessThan
	OpMatches     = api.OpMatches
)

// Re-export API functions
var NewContext = api.NewContext

// Re-export Core types
type InMemoryFeatureFlags = core.InMemoryFeatureFlags

// Re-export Core functions
var (
	NewFeatureFlags = core.NewFeatureFlags
	SimpleFlag      = core.SimpleFlag
	PercentageFlag  = core.PercentageFlag
)
