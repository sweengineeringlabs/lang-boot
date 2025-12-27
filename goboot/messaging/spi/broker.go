// Package spi contains the Service Provider Interface for the messaging module.
package spi

import (
	"context"

	"dev.engineeringlabs/goboot/messaging/api"
)

// Broker is the interface for message brokers.
//
// Implement this for Kafka, RabbitMQ, etc.
//
// Example:
//
//	type KafkaBroker struct {
//	    client *kafka.Client
//	}
//
//	func (b *KafkaBroker) CreatePublisher(topic string) (api.Publisher, error) {
//	    return &KafkaPublisher{client: b.client, topic: topic}, nil
//	}
type Broker interface {
	// Connect connects to the broker.
	Connect(ctx context.Context) error

	// Disconnect disconnects from the broker.
	Disconnect(ctx context.Context) error

	// CreatePublisher creates a publisher for a topic.
	CreatePublisher(topic string) (api.Publisher, error)

	// CreateSubscriber creates a subscriber for a topic.
	CreateSubscriber(config api.ConsumerConfig) (api.Subscriber, error)

	// CreateQueue creates a queue.
	CreateQueue(config api.QueueConfig) error

	// DeleteQueue deletes a queue.
	DeleteQueue(name string) error

	// IsConnected returns true if connected.
	IsConnected() bool
}

// Serializer is the interface for message serialization.
type Serializer interface {
	// Serialize serializes a value to bytes.
	Serialize(value any) ([]byte, error)

	// Deserialize deserializes bytes to a value.
	Deserialize(data []byte, target any) error

	// ContentType returns the content type.
	ContentType() string
}

// MessageInterceptor is the interface for message interception.
//
// Implement this for logging, tracing, etc.
type MessageInterceptor interface {
	// Before is called before a message is published.
	BeforePublish(ctx context.Context, message *api.Message) error

	// AfterPublish is called after a message is published.
	AfterPublish(ctx context.Context, message *api.Message, err error)

	// BeforeConsume is called before a message is consumed.
	BeforeConsume(ctx context.Context, message *api.Message) error

	// AfterConsume is called after a message is consumed.
	AfterConsume(ctx context.Context, message *api.Message, err error)
}

// DeadLetterHandler is the interface for dead letter handling.
type DeadLetterHandler interface {
	// Handle handles a dead letter message.
	Handle(ctx context.Context, message *api.Message, err error) error
}
