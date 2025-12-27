// Package messaging provides message queue utilities for the goboot framework.
//
// This module provides:
//   - API layer: Message, Publisher, Subscriber, EventBus interfaces
//   - Core layer: InMemoryEventBus, InMemoryPublisher, typed Channel
//   - SPI layer: Broker, Serializer, MessageInterceptor interfaces
//
// Example:
//
//	import "dev.engineeringlabs/goboot/messaging"
//
//	// In-process event bus
//	bus := messaging.NewEventBus()
//
//	// Subscribe to events
//	unsubscribe, _ := bus.Subscribe("user.created", func(event any) {
//	    user := event.(User)
//	    fmt.Println("User created:", user.Name)
//	})
//	defer unsubscribe()
//
//	// Publish events
//	bus.Publish("user.created", User{Name: "John"})
//
//	// Typed channels
//	channel := messaging.NewChannel[User](bus, "user.created")
//	channel.Subscribe(func(user User) {
//	    fmt.Println("User:", user.Name)
//	})
//	channel.Publish(User{Name: "Jane"})
package messaging

import (
	"dev.engineeringlabs/goboot/messaging/api"
	"dev.engineeringlabs/goboot/messaging/core"
	"dev.engineeringlabs/goboot/messaging/spi"
)

// Re-export API types
type (
	// Message represents a message.
	Message = api.Message
	// Publisher is the interface for message publishers.
	Publisher = api.Publisher
	// Handler is the function signature for message handlers.
	Handler = api.Handler
	// Subscriber is the interface for message subscribers.
	Subscriber = api.Subscriber
	// EventBus is the interface for in-process event buses.
	EventBus = api.EventBus
	// QueueConfig configures a message queue.
	QueueConfig = api.QueueConfig
	// Acknowledgment represents message acknowledgment options.
	Acknowledgment = api.Acknowledgment
	// ConsumerConfig configures a message consumer.
	ConsumerConfig = api.ConsumerConfig
)

// Re-export API constants
const (
	AckAuto   = api.AckAuto
	AckManual = api.AckManual
	AckNone   = api.AckNone
)

// Re-export API functions
var (
	NewMessage            = api.NewMessage
	DefaultQueueConfig    = api.DefaultQueueConfig
	DefaultConsumerConfig = api.DefaultConsumerConfig
)

// Re-export Core types
type (
	// InMemoryEventBus is an in-process event bus implementation.
	InMemoryEventBus = core.InMemoryEventBus
	// InMemoryPublisher is an in-memory publisher for testing.
	InMemoryPublisher = core.InMemoryPublisher
	// InMemorySubscriber is an in-memory subscriber for testing.
	InMemorySubscriber = core.InMemorySubscriber
	// Channel is a typed pub/sub channel.
	Channel[T any] struct {
		*core.Channel[T]
	}
)

// Re-export Core functions
var (
	NewEventBus          = core.NewEventBus
	NewInMemoryPublisher = core.NewInMemoryPublisher
	NewInMemorySubscriber = core.NewInMemorySubscriber
)

// NewChannel creates a new typed channel.
func NewChannel[T any](bus *InMemoryEventBus, topic string) *Channel[T] {
	return &Channel[T]{core.NewChannel[T](bus, topic)}
}

// Re-export SPI types
type (
	// Broker is the interface for message brokers.
	Broker = spi.Broker
	// Serializer is the interface for message serialization.
	Serializer = spi.Serializer
	// MessageInterceptor is the interface for message interception.
	MessageInterceptor = spi.MessageInterceptor
	// DeadLetterHandler is the interface for dead letter handling.
	DeadLetterHandler = spi.DeadLetterHandler
)
