// Package core contains the implementation details for the config module.
package core

import (
	"os"
	"strings"

	"dev.engineeringlabs/goboot/config/api"
	"dev.engineeringlabs/goboot/config/spi"
)

// ConfigLoader loads configuration from multiple sources.
type ConfigLoader struct {
	sources []spi.ConfigurationSource
}

// NewConfigLoader creates a new ConfigLoader.
func NewConfigLoader() *ConfigLoader {
	return &ConfigLoader{
		sources: make([]spi.ConfigurationSource, 0),
	}
}

// AddSource adds a configuration source.
func (l *ConfigLoader) AddSource(source spi.ConfigurationSource) *ConfigLoader {
	l.sources = append(l.sources, source)
	return l
}

// Load loads configuration from all sources.
// Sources are loaded in order, with later sources taking precedence.
func (l *ConfigLoader) Load() (*api.Settings, error) {
	settings := api.NewSettings()

	for _, source := range l.sources {
		values, err := source.Load()
		if err != nil {
			return nil, err
		}

		for k, v := range values {
			if str, ok := v.(string); ok {
				settings.Set(k, str, api.ConfigSource(source.Name()))
			}
		}
	}

	return settings, nil
}

// LoadWithEnv loads configuration and merges with environment variables.
func (l *ConfigLoader) LoadWithEnv(prefix string) (*api.Settings, error) {
	settings, err := l.Load()
	if err != nil {
		return nil, err
	}

	// Load environment variables with the given prefix
	envSettings := loadEnvVars(prefix)
	settings.Merge(envSettings)

	return settings, nil
}

// loadEnvVars loads environment variables with the given prefix.
func loadEnvVars(prefix string) *api.Settings {
	settings := api.NewSettings()
	prefix = strings.ToUpper(prefix)

	for _, env := range os.Environ() {
		parts := strings.SplitN(env, "=", 2)
		if len(parts) != 2 {
			continue
		}

		key := parts[0]
		value := parts[1]

		if strings.HasPrefix(key, prefix) {
			// Remove prefix and convert to dot notation
			configKey := strings.TrimPrefix(key, prefix)
			configKey = strings.ToLower(configKey)
			configKey = strings.ReplaceAll(configKey, "_", ".")

			settings.Set(configKey, value, api.SourceEnv)
		}
	}

	return settings
}

// Global settings management

var globalSettings *api.Settings

// Configure sets the global settings.
func Configure(settings *api.Settings) {
	globalSettings = settings
}

// GetSettings returns the global settings.
func GetSettings() *api.Settings {
	if globalSettings == nil {
		globalSettings = api.NewSettings()
	}
	return globalSettings
}
