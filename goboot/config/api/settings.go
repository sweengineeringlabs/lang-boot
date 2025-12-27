// Package api contains the public interfaces and types for the config module.
package api

import (
	"strconv"
	"time"
)

// ConfigSource represents the source of a configuration value.
type ConfigSource string

const (
	SourceDefault ConfigSource = "default"
	SourceEnv     ConfigSource = "env"
	SourceFile    ConfigSource = "file"
	SourceRemote  ConfigSource = "remote"
)

// ConfigValue represents a configuration value with metadata.
type ConfigValue struct {
	raw    string
	source ConfigSource
	exists bool
}

// NewConfigValue creates a new ConfigValue.
func NewConfigValue(raw string, source ConfigSource) *ConfigValue {
	return &ConfigValue{
		raw:    raw,
		source: source,
		exists: true,
	}
}

// EmptyConfigValue creates an empty ConfigValue.
func EmptyConfigValue() *ConfigValue {
	return &ConfigValue{
		exists: false,
	}
}

// Exists returns true if the value exists.
func (v *ConfigValue) Exists() bool {
	return v.exists
}

// Source returns the source of the value.
func (v *ConfigValue) Source() ConfigSource {
	return v.source
}

// AsString returns the value as a string.
func (v *ConfigValue) AsString(defaultVal string) string {
	if !v.exists {
		return defaultVal
	}
	return v.raw
}

// AsInt returns the value as an integer.
func (v *ConfigValue) AsInt(defaultVal int) int {
	if !v.exists {
		return defaultVal
	}
	val, err := strconv.Atoi(v.raw)
	if err != nil {
		return defaultVal
	}
	return val
}

// AsInt64 returns the value as an int64.
func (v *ConfigValue) AsInt64(defaultVal int64) int64 {
	if !v.exists {
		return defaultVal
	}
	val, err := strconv.ParseInt(v.raw, 10, 64)
	if err != nil {
		return defaultVal
	}
	return val
}

// AsFloat returns the value as a float64.
func (v *ConfigValue) AsFloat(defaultVal float64) float64 {
	if !v.exists {
		return defaultVal
	}
	val, err := strconv.ParseFloat(v.raw, 64)
	if err != nil {
		return defaultVal
	}
	return val
}

// AsBool returns the value as a boolean.
func (v *ConfigValue) AsBool(defaultVal bool) bool {
	if !v.exists {
		return defaultVal
	}
	val, err := strconv.ParseBool(v.raw)
	if err != nil {
		return defaultVal
	}
	return val
}

// AsDuration returns the value as a duration.
func (v *ConfigValue) AsDuration(defaultVal time.Duration) time.Duration {
	if !v.exists {
		return defaultVal
	}
	val, err := time.ParseDuration(v.raw)
	if err != nil {
		return defaultVal
	}
	return val
}

// Settings represents a hierarchical configuration store.
type Settings struct {
	values map[string]*ConfigValue
}

// NewSettings creates a new Settings instance.
func NewSettings() *Settings {
	return &Settings{
		values: make(map[string]*ConfigValue),
	}
}

// Get retrieves a configuration value by key.
// Supports dot notation for nested keys (e.g., "api.timeout").
func (s *Settings) Get(key string) *ConfigValue {
	if val, ok := s.values[key]; ok {
		return val
	}
	return EmptyConfigValue()
}

// Set sets a configuration value.
func (s *Settings) Set(key string, value string, source ConfigSource) {
	s.values[key] = NewConfigValue(value, source)
}

// SetValue sets a ConfigValue directly.
func (s *Settings) SetValue(key string, value *ConfigValue) {
	s.values[key] = value
}

// Has returns true if the key exists.
func (s *Settings) Has(key string) bool {
	_, ok := s.values[key]
	return ok
}

// Keys returns all configuration keys.
func (s *Settings) Keys() []string {
	keys := make([]string, 0, len(s.values))
	for k := range s.values {
		keys = append(keys, k)
	}
	return keys
}

// Merge merges another Settings into this one.
// Values from other take precedence.
func (s *Settings) Merge(other *Settings) {
	for k, v := range other.values {
		s.values[k] = v
	}
}
