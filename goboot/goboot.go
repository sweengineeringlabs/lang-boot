// Package goboot provides a collection of independent, production-ready
// infrastructure modules for Go applications.
//
// Goboot follows the Stratified Encapsulation Architecture (SEA) pattern,
// where each module has:
//   - API layer: Public interfaces and types
//   - Core layer: Implementations
//   - SPI layer: Extension points (optional)
//
// Each module is standalone with no internal dependencies.
//
// Modules:
//   - errors: Error types, Result monad
//   - config: Configuration management
//   - di: Dependency injection container
//   - resilience: Circuit breakers, retries, timeouts
//   - validation: Input validation
//   - cache: Multi-backend caching
//   - observability: Logging, metrics, tracing
//
// Note: Each module is independent. Import from the specific module you need:
//
//	import (
//	    "dev.engineeringlabs/goboot/resilience"
//	    "dev.engineeringlabs/goboot/errors"
//	    "dev.engineeringlabs/goboot/config"
//	)
package goboot

// Version is the current goboot version.
const Version = "0.1.0"
