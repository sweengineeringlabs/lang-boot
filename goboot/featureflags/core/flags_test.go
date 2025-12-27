package core

import (
	"context"
	"testing"

	"dev.engineeringlabs/goboot/featureflags/api"
)

func TestInMemoryFeatureFlags_SimpleFlag(t *testing.T) {
	ff := NewFeatureFlags()
	ctx := context.Background()

	ff.SetFlag(SimpleFlag("feature1", true))

	if !ff.IsEnabled(ctx, "feature1", nil) {
		t.Error("Feature should be enabled")
	}
}

func TestInMemoryFeatureFlags_DisabledFlag(t *testing.T) {
	ff := NewFeatureFlags()
	ctx := context.Background()

	ff.SetFlag(SimpleFlag("feature1", false))

	if ff.IsEnabled(ctx, "feature1", nil) {
		t.Error("Feature should be disabled")
	}
}

func TestInMemoryFeatureFlags_NonExistentFlag(t *testing.T) {
	ff := NewFeatureFlags()
	ctx := context.Background()

	if ff.IsEnabled(ctx, "nonexistent", nil) {
		t.Error("Non-existent flag should be disabled")
	}
}

func TestInMemoryFeatureFlags_PercentageRollout(t *testing.T) {
	ff := NewFeatureFlags()
	ctx := context.Background()

	ff.SetFlag(PercentageFlag("rollout", 100))

	evalCtx := api.NewContext("user-123")
	if !ff.IsEnabled(ctx, "rollout", evalCtx) {
		t.Error("100% rollout should be enabled for all users")
	}
}

func TestInMemoryFeatureFlags_RuleEvaluation(t *testing.T) {
	ff := NewFeatureFlags()
	ctx := context.Background()

	ff.SetFlag(&api.Flag{
		Key:     "premium_feature",
		Enabled: true,
		Rules: []api.Rule{
			{
				ID:      "premium-users",
				Enabled: true,
				Conditions: []api.Condition{
					{Attribute: "plan", Operator: api.OpEquals, Value: "premium"},
				},
				Variant: "enabled",
			},
		},
		Variants: []api.Variant{
			{Name: "enabled", Value: true, Weight: 0}, // Zero weight so rollout doesn't apply
		},
		DefaultValue: false,
	})

	// Premium user - rule matches
	premiumCtx := api.NewContext("user-1").WithAttribute("plan", "premium")
	if !ff.IsEnabled(ctx, "premium_feature", premiumCtx) {
		t.Error("Premium user should have feature enabled")
	}

	// Free user - rule doesn't match, default is false
	freeCtx := api.NewContext("user-2").WithAttribute("plan", "free")
	if ff.IsEnabled(ctx, "premium_feature", freeCtx) {
		t.Error("Free user should not have feature enabled")
	}
}

func TestInMemoryFeatureFlags_GetValue(t *testing.T) {
	ff := NewFeatureFlags()
	ctx := context.Background()

	ff.SetFlag(&api.Flag{
		Key:          "config",
		Enabled:      true,
		DefaultValue: "default",
	})

	value := ff.GetValue(ctx, "config", nil)
	if value != "default" {
		t.Errorf("Expected 'default', got %v", value)
	}
}

func TestInMemoryFeatureFlags_DeleteFlag(t *testing.T) {
	ff := NewFeatureFlags()

	ff.SetFlag(SimpleFlag("feature", true))
	ff.DeleteFlag("feature")

	_, err := ff.GetFlag("feature")
	if err == nil {
		t.Error("Deleted flag should not be found")
	}
}

func TestInMemoryFeatureFlags_AllFlags(t *testing.T) {
	ff := NewFeatureFlags()

	ff.SetFlag(SimpleFlag("a", true))
	ff.SetFlag(SimpleFlag("b", false))
	ff.SetFlag(SimpleFlag("c", true))

	flags := ff.AllFlags()
	if len(flags) != 3 {
		t.Errorf("Expected 3 flags, got %d", len(flags))
	}
}
