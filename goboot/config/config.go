// Package config provides configuration management for the goboot framework.
//
// This module provides:
//   - API layer: Settings, ConfigValue, ConfigSource
//   - Core layer: ConfigLoader, global settings management
//   - SPI layer: ConfigurationSource interface for custom sources
//
// Example:
//
//	import "dev.engineeringlabs/goboot/config"
//
//	loader := config.NewConfigLoader()
//	settings, _ := loader.LoadWithEnv("APP_")
//
//	timeout := settings.Get("api.timeout").AsInt(30)
package config

import (
	"dev.engineeringlabs/goboot/config/api"
	"dev.engineeringlabs/goboot/config/core"
	"dev.engineeringlabs/goboot/config/spi"
)

// Re-export API types
type (
	// ConfigSource represents the source of a configuration value.
	ConfigSource = api.ConfigSource
	// ConfigValue represents a configuration value with metadata.
	ConfigValue = api.ConfigValue
	// Settings represents a hierarchical configuration store.
	Settings = api.Settings
)

// Re-export API constants
const (
	SourceDefault = api.SourceDefault
	SourceEnv     = api.SourceEnv
	SourceFile    = api.SourceFile
	SourceRemote  = api.SourceRemote
)

// Re-export API functions
var (
	NewConfigValue    = api.NewConfigValue
	EmptyConfigValue  = api.EmptyConfigValue
	NewSettings       = api.NewSettings
)

// Re-export Core types
type ConfigLoader = core.ConfigLoader

// Re-export Core functions
var (
	NewConfigLoader = core.NewConfigLoader
	Configure       = core.Configure
	GetSettings     = core.GetSettings
)

// Re-export SPI types
type ConfigurationSource = spi.ConfigurationSource
type RefreshableConfigurationSource = spi.RefreshableConfigurationSource
