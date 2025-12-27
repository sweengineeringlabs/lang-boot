package core

import (
	"testing"
	"time"
)

func TestSystemClock(t *testing.T) {
	clock := NewSystemClock()

	before := time.Now()
	now := clock.Now()
	after := time.Now()

	if now.Before(before) || now.After(after) {
		t.Error("SystemClock.Now should return current time")
	}
}

func TestFrozenClock(t *testing.T) {
	frozen := time.Date(2024, 1, 1, 12, 0, 0, 0, time.UTC)
	clock := NewFrozenClock(frozen)

	if !clock.Now().Equal(frozen) {
		t.Error("FrozenClock should return frozen time")
	}

	// Advance
	clock.Advance(time.Hour)
	expected := frozen.Add(time.Hour)
	if !clock.Now().Equal(expected) {
		t.Error("Advance should move time forward")
	}

	// Set
	newTime := time.Date(2025, 6, 15, 0, 0, 0, 0, time.UTC)
	clock.Set(newTime)
	if !clock.Now().Equal(newTime) {
		t.Error("Set should change frozen time")
	}
}

func TestOffsetClock(t *testing.T) {
	offset := 24 * time.Hour
	clock := NewOffsetClock(offset)

	systemNow := time.Now()
	clockNow := clock.Now()

	diff := clockNow.Sub(systemNow)
	if diff < 23*time.Hour || diff > 25*time.Hour {
		t.Errorf("OffsetClock should be ~24h ahead, got %v", diff)
	}
}

func TestStartOfDay(t *testing.T) {
	input := time.Date(2024, 6, 15, 14, 30, 45, 123, time.UTC)
	expected := time.Date(2024, 6, 15, 0, 0, 0, 0, time.UTC)

	result := StartOfDay(input)
	if !result.Equal(expected) {
		t.Errorf("Expected %v, got %v", expected, result)
	}
}

func TestEndOfDay(t *testing.T) {
	input := time.Date(2024, 6, 15, 14, 30, 45, 0, time.UTC)
	result := EndOfDay(input)

	if result.Hour() != 23 || result.Minute() != 59 || result.Second() != 59 {
		t.Errorf("End of day should be 23:59:59, got %v", result)
	}
}

func TestStartOfMonth(t *testing.T) {
	input := time.Date(2024, 6, 15, 14, 30, 0, 0, time.UTC)
	expected := time.Date(2024, 6, 1, 0, 0, 0, 0, time.UTC)

	result := StartOfMonth(input)
	if !result.Equal(expected) {
		t.Errorf("Expected %v, got %v", expected, result)
	}
}

func TestEndOfMonth(t *testing.T) {
	input := time.Date(2024, 6, 15, 0, 0, 0, 0, time.UTC)
	result := EndOfMonth(input)

	if result.Day() != 30 {
		t.Errorf("June ends on 30th, got %d", result.Day())
	}
}

func TestIsWeekday(t *testing.T) {
	monday := time.Date(2024, 12, 23, 0, 0, 0, 0, time.UTC)
	saturday := time.Date(2024, 12, 28, 0, 0, 0, 0, time.UTC)

	if !IsWeekday(monday) {
		t.Error("Monday should be weekday")
	}
	if IsWeekday(saturday) {
		t.Error("Saturday should not be weekday")
	}
}

func TestIsWeekend(t *testing.T) {
	friday := time.Date(2024, 12, 27, 0, 0, 0, 0, time.UTC)
	sunday := time.Date(2024, 12, 29, 0, 0, 0, 0, time.UTC)

	if IsWeekend(friday) {
		t.Error("Friday should not be weekend")
	}
	if !IsWeekend(sunday) {
		t.Error("Sunday should be weekend")
	}
}

func TestDaysBetween(t *testing.T) {
	a := time.Date(2024, 1, 1, 0, 0, 0, 0, time.UTC)
	b := time.Date(2024, 1, 10, 0, 0, 0, 0, time.UTC)

	days := DaysBetween(a, b)
	if days != 9 {
		t.Errorf("Expected 9 days, got %d", days)
	}
}

func TestAge(t *testing.T) {
	// Use a fixed date that's clearly in the past
	now := time.Now()
	birthdate := time.Date(now.Year()-25, now.Month(), now.Day()-1, 0, 0, 0, 0, time.UTC)
	age := Age(birthdate)

	if age != 25 {
		t.Errorf("Expected age 25, got %d", age)
	}

	// Test someone born tomorrow 25 years ago (should be 24)
	birthdateTomorrow := time.Date(now.Year()-25, now.Month(), now.Day()+1, 0, 0, 0, 0, time.UTC)
	ageTomorrow := Age(birthdateTomorrow)

	if ageTomorrow != 24 {
		t.Errorf("Expected age 24 (birthday hasn't happened yet), got %d", ageTomorrow)
	}
}
