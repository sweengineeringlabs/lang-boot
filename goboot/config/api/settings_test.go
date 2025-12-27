package api

import (
	"testing"
	"time"
)

func TestConfigValue(t *testing.T) {
	t.Run("NewConfigValue", func(t *testing.T) {
		v := NewConfigValue("test", SourceEnv)
		if !v.Exists() {
			t.Error("Value should exist")
		}
		if v.Source() != SourceEnv {
			t.Error("Source should be env")
		}
	})

	t.Run("EmptyConfigValue", func(t *testing.T) {
		v := EmptyConfigValue()
		if v.Exists() {
			t.Error("Empty value should not exist")
		}
	})

	t.Run("AsString", func(t *testing.T) {
		v := NewConfigValue("hello", SourceDefault)
		if v.AsString("default") != "hello" {
			t.Error("AsString should return value")
		}

		empty := EmptyConfigValue()
		if empty.AsString("default") != "default" {
			t.Error("AsString should return default for empty")
		}
	})

	t.Run("AsInt", func(t *testing.T) {
		v := NewConfigValue("42", SourceDefault)
		if v.AsInt(0) != 42 {
			t.Error("AsInt should parse integer")
		}

		invalid := NewConfigValue("not-a-number", SourceDefault)
		if invalid.AsInt(100) != 100 {
			t.Error("AsInt should return default for invalid")
		}

		empty := EmptyConfigValue()
		if empty.AsInt(99) != 99 {
			t.Error("AsInt should return default for empty")
		}
	})

	t.Run("AsFloat", func(t *testing.T) {
		v := NewConfigValue("3.14", SourceDefault)
		if v.AsFloat(0) != 3.14 {
			t.Error("AsFloat should parse float")
		}
	})

	t.Run("AsBool", func(t *testing.T) {
		trueVal := NewConfigValue("true", SourceDefault)
		if !trueVal.AsBool(false) {
			t.Error("AsBool should parse true")
		}

		falseVal := NewConfigValue("false", SourceDefault)
		if falseVal.AsBool(true) {
			t.Error("AsBool should parse false")
		}
	})

	t.Run("AsDuration", func(t *testing.T) {
		v := NewConfigValue("5s", SourceDefault)
		if v.AsDuration(0) != 5*time.Second {
			t.Error("AsDuration should parse duration")
		}

		invalid := NewConfigValue("invalid", SourceDefault)
		if invalid.AsDuration(10*time.Second) != 10*time.Second {
			t.Error("AsDuration should return default for invalid")
		}
	})
}

func TestSettings(t *testing.T) {
	t.Run("GetSet", func(t *testing.T) {
		s := NewSettings()
		s.Set("key", "value", SourceDefault)

		v := s.Get("key")
		if v.AsString("") != "value" {
			t.Error("Get should return set value")
		}
	})

	t.Run("GetNonExistent", func(t *testing.T) {
		s := NewSettings()
		v := s.Get("nonexistent")
		if v.Exists() {
			t.Error("Get should return empty for nonexistent key")
		}
	})

	t.Run("Has", func(t *testing.T) {
		s := NewSettings()
		s.Set("existing", "value", SourceDefault)

		if !s.Has("existing") {
			t.Error("Has should return true for existing key")
		}
		if s.Has("nonexistent") {
			t.Error("Has should return false for nonexistent key")
		}
	})

	t.Run("Keys", func(t *testing.T) {
		s := NewSettings()
		s.Set("a", "1", SourceDefault)
		s.Set("b", "2", SourceDefault)
		s.Set("c", "3", SourceDefault)

		keys := s.Keys()
		if len(keys) != 3 {
			t.Errorf("Expected 3 keys, got %d", len(keys))
		}
	})

	t.Run("Merge", func(t *testing.T) {
		s1 := NewSettings()
		s1.Set("a", "1", SourceDefault)
		s1.Set("b", "2", SourceDefault)

		s2 := NewSettings()
		s2.Set("b", "override", SourceEnv)
		s2.Set("c", "3", SourceEnv)

		s1.Merge(s2)

		if s1.Get("a").AsString("") != "1" {
			t.Error("Merge should preserve unaffected keys")
		}
		if s1.Get("b").AsString("") != "override" {
			t.Error("Merge should override existing keys")
		}
		if s1.Get("c").AsString("") != "3" {
			t.Error("Merge should add new keys")
		}
	})
}
