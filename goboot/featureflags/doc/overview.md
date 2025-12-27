# Feature Flags Module Overview

## WHAT: Feature Toggles

Runtime feature flags with gradual rollouts.

Key capabilities:
- **Toggles** - On/off feature flags
- **Gradual Rollout** - Percentage-based
- **User Targeting** - User-specific flags
- **Remote Config** - External flag sources

## WHY: Safe Deployments

**Problems Solved**: Big-bang releases, rollback complexity

**When to Use**: Continuous deployment, A/B testing

## HOW: Usage Guide

```go
flags := featureflags.New(featureflags.Config{
    Source: featureflags.InMemory(map[string]bool{
        "new-checkout": true,
        "dark-mode":    false,
    }),
})

if flags.IsEnabled("new-checkout") {
    renderNewCheckout()
} else {
    renderOldCheckout()
}
```

---

**Status**: Stable
