// Package spi contains the Service Provider Interface for the config module.
// Implement these interfaces to create custom configuration sources.
package spi

// ConfigurationSource is the interface for configuration sources.
//
// Implement this to create custom configuration sources like:
//   - Remote configuration services (e.g., Consul, etcd)
//   - Database-backed configuration
//   - Custom file formats
//
// Example:
//
//	type ConsulConfigSource struct {
//	    client ConsulClient
//	}
//
//	func (s *ConsulConfigSource) Name() string {
//	    return "consul"
//	}
//
//	func (s *ConsulConfigSource) Load() (map[string]any, error) {
//	    return s.client.GetAll()
//	}
//
//	func (s *ConsulConfigSource) SupportsRefresh() bool {
//	    return true
//	}
type ConfigurationSource interface {
	// Name returns the source name.
	Name() string

	// Load loads configuration from this source.
	Load() (map[string]any, error)

	// SupportsRefresh returns true if this source supports refresh.
	SupportsRefresh() bool
}

// RefreshableConfigurationSource is an optional interface for sources that support refresh.
type RefreshableConfigurationSource interface {
	ConfigurationSource

	// Refresh reloads configuration from this source.
	Refresh() (map[string]any, error)
}
