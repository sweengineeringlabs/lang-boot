package core

import (
	"context"
	"errors"
	"testing"
	"time"

	"dev.engineeringlabs/goboot/health/api"
)

func TestDefaultRegistry_Check(t *testing.T) {
	registry := NewRegistry()

	registry.RegisterFunc("test", func(ctx context.Context) api.Check {
		return *api.NewCheck("test", api.StatusUp)
	})

	report := registry.Check(context.Background())

	if report.Status != api.StatusUp {
		t.Errorf("Expected UP, got %s", report.Status)
	}
	if len(report.Checks) != 1 {
		t.Errorf("Expected 1 check, got %d", len(report.Checks))
	}
}

func TestDefaultRegistry_MultipleCheckers(t *testing.T) {
	registry := NewRegistry()

	registry.RegisterFunc("db", func(ctx context.Context) api.Check {
		return *api.NewCheck("db", api.StatusUp)
	})
	registry.RegisterFunc("cache", func(ctx context.Context) api.Check {
		return *api.NewCheck("cache", api.StatusUp)
	})
	registry.RegisterFunc("external", func(ctx context.Context) api.Check {
		return *api.NewCheck("external", api.StatusDown).WithMessage("unreachable")
	})

	report := registry.Check(context.Background())

	if report.Status != api.StatusDown {
		t.Error("Overall status should be DOWN when any check is DOWN")
	}
	if len(report.Checks) != 3 {
		t.Errorf("Expected 3 checks, got %d", len(report.Checks))
	}
}

func TestPingChecker(t *testing.T) {
	t.Run("Healthy", func(t *testing.T) {
		checker := NewPingChecker("db", func(ctx context.Context) error {
			return nil
		})

		check := checker.Check(context.Background())
		if check.Status != api.StatusUp {
			t.Error("Expected UP")
		}
	})

	t.Run("Unhealthy", func(t *testing.T) {
		checker := NewPingChecker("db", func(ctx context.Context) error {
			return errors.New("connection refused")
		})

		check := checker.Check(context.Background())
		if check.Status != api.StatusDown {
			t.Error("Expected DOWN")
		}
		if check.Message != "connection refused" {
			t.Errorf("Unexpected message: %s", check.Message)
		}
	})
}

func TestTimeoutChecker(t *testing.T) {
	slowChecker := &slowChecker{}
	timeoutChecker := NewTimeoutChecker(slowChecker, 50*time.Millisecond)

	check := timeoutChecker.Check(context.Background())

	if check.Status != api.StatusDown {
		t.Error("Expected DOWN due to timeout")
	}
	if check.Message != "health check timed out" {
		t.Errorf("Unexpected message: %s", check.Message)
	}
}

type slowChecker struct{}

func (c *slowChecker) Name() string {
	return "slow"
}

func (c *slowChecker) Check(ctx context.Context) api.Check {
	time.Sleep(200 * time.Millisecond)
	return *api.NewCheck("slow", api.StatusUp)
}

func TestCompositeChecker(t *testing.T) {
	checker1 := NewPingChecker("a", func(ctx context.Context) error { return nil })
	checker2 := NewPingChecker("b", func(ctx context.Context) error { return nil })
	
	composite := NewCompositeChecker("all", checker1, checker2)
	check := composite.Check(context.Background())

	if check.Status != api.StatusUp {
		t.Error("Expected UP when all sub-checks pass")
	}
}

func TestLivenessCheck(t *testing.T) {
	checker := LivenessCheck()
	check := checker(context.Background())

	if check.Status != api.StatusUp {
		t.Error("Liveness check should always be UP")
	}
}

func TestReadinessCheck(t *testing.T) {
	ready := false
	checker := ReadinessCheck(func() bool { return ready })

	check := checker(context.Background())
	if check.Status != api.StatusDown {
		t.Error("Should be DOWN when not ready")
	}

	ready = true
	check = checker(context.Background())
	if check.Status != api.StatusUp {
		t.Error("Should be UP when ready")
	}
}

func TestCheck_IsHealthy(t *testing.T) {
	up := api.NewCheck("test", api.StatusUp)
	if !up.IsHealthy() {
		t.Error("UP should be healthy")
	}

	down := api.NewCheck("test", api.StatusDown)
	if down.IsHealthy() {
		t.Error("DOWN should not be healthy")
	}
}

func TestReport_IsHealthy(t *testing.T) {
	report := api.NewReport()
	if !report.IsHealthy() {
		t.Error("Empty report should be healthy")
	}

	report.AddCheck(*api.NewCheck("a", api.StatusUp))
	if !report.IsHealthy() {
		t.Error("All UP should be healthy")
	}

	report.AddCheck(*api.NewCheck("b", api.StatusDown))
	if report.IsHealthy() {
		t.Error("Any DOWN should be unhealthy")
	}
}
