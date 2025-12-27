// Package core contains the implementation details for the feature flags module.
package core

import (
	"context"
	"fmt"
	"hash/fnv"
	"regexp"
	"sync"

	"dev.engineeringlabs/goboot/featureflags/api"
)

// InMemoryFeatureFlags is an in-memory feature flags implementation.
type InMemoryFeatureFlags struct {
	flags map[string]*api.Flag
	mu    sync.RWMutex
}

// NewFeatureFlags creates a new InMemoryFeatureFlags.
func NewFeatureFlags() *InMemoryFeatureFlags {
	return &InMemoryFeatureFlags{
		flags: make(map[string]*api.Flag),
	}
}

// IsEnabled checks if a flag is enabled.
func (f *InMemoryFeatureFlags) IsEnabled(ctx context.Context, flagKey string, evalCtx *api.EvaluationContext) bool {
	result := f.Evaluate(ctx, flagKey, evalCtx)
	return result.Enabled
}

// GetValue gets the flag value.
func (f *InMemoryFeatureFlags) GetValue(ctx context.Context, flagKey string, evalCtx *api.EvaluationContext) any {
	result := f.Evaluate(ctx, flagKey, evalCtx)
	return result.Value
}

// Evaluate evaluates a flag.
func (f *InMemoryFeatureFlags) Evaluate(ctx context.Context, flagKey string, evalCtx *api.EvaluationContext) *api.EvaluationResult {
	f.mu.RLock()
	flag, ok := f.flags[flagKey]
	f.mu.RUnlock()

	if !ok {
		return &api.EvaluationResult{
			FlagKey: flagKey,
			Enabled: false,
			Reason:  "flag not found",
		}
	}

	// Check if flag is globally disabled
	if !flag.Enabled {
		return &api.EvaluationResult{
			FlagKey: flagKey,
			Enabled: false,
			Value:   flag.DefaultValue,
			Reason:  "flag disabled",
		}
	}

	// Evaluate rules
	for _, rule := range flag.Rules {
		if !rule.Enabled {
			continue
		}

		if f.evaluateRule(rule, evalCtx) {
			variant := f.getVariant(flag, rule.Variant)
			return &api.EvaluationResult{
				FlagKey: flagKey,
				Enabled: true,
				Value:   variant.Value,
				Variant: variant.Name,
				Reason:  fmt.Sprintf("rule matched: %s", rule.ID),
			}
		}
	}

	// Check percentage-based rollout
	if len(flag.Variants) > 0 && evalCtx != nil && evalCtx.UserID != "" {
		variant := f.selectVariant(flag, evalCtx.UserID)
		if variant != nil {
			return &api.EvaluationResult{
				FlagKey: flagKey,
				Enabled: true,
				Value:   variant.Value,
				Variant: variant.Name,
				Reason:  "percentage rollout",
			}
		}
	}

	// Return default - when no rules match and no percentage rollout applies,
	// the enabled state depends on the DefaultValue (if it's a boolean)
	enabled := false
	if defaultBool, ok := flag.DefaultValue.(bool); ok {
		enabled = defaultBool
	}
	return &api.EvaluationResult{
		FlagKey: flagKey,
		Enabled: enabled,
		Value:   flag.DefaultValue,
		Reason:  "default value",
	}
}

func (f *InMemoryFeatureFlags) evaluateRule(rule api.Rule, evalCtx *api.EvaluationContext) bool {
	if evalCtx == nil {
		return false
	}

	for _, condition := range rule.Conditions {
		attrValue, ok := evalCtx.Attributes[condition.Attribute]
		if !ok {
			// Special handling for userID
			if condition.Attribute == "userID" {
				attrValue = evalCtx.UserID
			} else {
				return false
			}
		}

		if !f.evaluateCondition(condition, attrValue) {
			return false
		}
	}

	return true
}

func (f *InMemoryFeatureFlags) evaluateCondition(condition api.Condition, attrValue any) bool {
	switch condition.Operator {
	case api.OpEquals:
		return fmt.Sprint(attrValue) == fmt.Sprint(condition.Value)
	case api.OpNotEquals:
		return fmt.Sprint(attrValue) != fmt.Sprint(condition.Value)
	case api.OpContains:
		return containsString(fmt.Sprint(attrValue), fmt.Sprint(condition.Value))
	case api.OpIn:
		if list, ok := condition.Value.([]any); ok {
			for _, v := range list {
				if fmt.Sprint(attrValue) == fmt.Sprint(v) {
					return true
				}
			}
		}
		return false
	case api.OpNotIn:
		if list, ok := condition.Value.([]any); ok {
			for _, v := range list {
				if fmt.Sprint(attrValue) == fmt.Sprint(v) {
					return false
				}
			}
		}
		return true
	case api.OpMatches:
		re, err := regexp.Compile(fmt.Sprint(condition.Value))
		if err != nil {
			return false
		}
		return re.MatchString(fmt.Sprint(attrValue))
	default:
		return false
	}
}

func (f *InMemoryFeatureFlags) getVariant(flag *api.Flag, name string) *api.Variant {
	for _, v := range flag.Variants {
		if v.Name == name {
			return &v
		}
	}
	return &api.Variant{Value: flag.DefaultValue}
}

func (f *InMemoryFeatureFlags) selectVariant(flag *api.Flag, userID string) *api.Variant {
	hash := hashString(flag.Key + userID)
	bucket := hash % 100

	cumulative := 0
	for _, variant := range flag.Variants {
		cumulative += variant.Weight
		if bucket < cumulative {
			return &variant
		}
	}
	return nil
}

func hashString(s string) int {
	h := fnv.New32a()
	h.Write([]byte(s))
	return int(h.Sum32())
}

func containsString(s, substr string) bool {
	return len(s) >= len(substr) && findSubstring(s, substr)
}

func findSubstring(s, substr string) bool {
	for i := 0; i <= len(s)-len(substr); i++ {
		if s[i:i+len(substr)] == substr {
			return true
		}
	}
	return false
}

// SetFlag sets a flag.
func (f *InMemoryFeatureFlags) SetFlag(flag *api.Flag) error {
	f.mu.Lock()
	f.flags[flag.Key] = flag
	f.mu.Unlock()
	return nil
}

// GetFlag gets a flag.
func (f *InMemoryFeatureFlags) GetFlag(flagKey string) (*api.Flag, error) {
	f.mu.RLock()
	flag, ok := f.flags[flagKey]
	f.mu.RUnlock()

	if !ok {
		return nil, fmt.Errorf("flag not found: %s", flagKey)
	}
	return flag, nil
}

// DeleteFlag deletes a flag.
func (f *InMemoryFeatureFlags) DeleteFlag(flagKey string) error {
	f.mu.Lock()
	delete(f.flags, flagKey)
	f.mu.Unlock()
	return nil
}

// AllFlags returns all flags.
func (f *InMemoryFeatureFlags) AllFlags() []*api.Flag {
	f.mu.RLock()
	defer f.mu.RUnlock()

	flags := make([]*api.Flag, 0, len(f.flags))
	for _, flag := range f.flags {
		flags = append(flags, flag)
	}
	return flags
}

// SimpleFlag creates a simple boolean flag.
func SimpleFlag(key string, enabled bool) *api.Flag {
	return &api.Flag{
		Key:          key,
		Name:         key,
		Enabled:      enabled,
		DefaultValue: enabled,
	}
}

// PercentageFlag creates a percentage-based rollout flag.
func PercentageFlag(key string, percentage int) *api.Flag {
	return &api.Flag{
		Key:     key,
		Name:    key,
		Enabled: true,
		Variants: []api.Variant{
			{Name: "enabled", Value: true, Weight: percentage},
			{Name: "disabled", Value: false, Weight: 100 - percentage},
		},
		DefaultValue: false,
	}
}
