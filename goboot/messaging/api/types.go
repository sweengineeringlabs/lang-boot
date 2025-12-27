// Package api contains the public interfaces and types for the messaging module.
package api

import (
	"context"
	"time"
)

// Message represents a message.
type Message struct {
	ID          string
	Topic       string
	Key         string
	Payload     []byte
	Headers     map[string]string
	Timestamp   time.Time
	ContentType string
}

// NewMessage creates a new Message.
func NewMessage(topic string, payload []byte) *Message {
	return &Message{
		Topic:     topic,
		Payload:   payload,
		Headers:   make(map[string]string),
		Timestamp: time.Now(),
	}
}

// WithKey sets the message key.
func (m *Message) WithKey(key string) *Message {
	m.Key = key
	return m
}

// WithHeader sets a header.
func (m *Message) WithHeader(key, value string) *Message {
	m.Headers[key] = value
	return m
}

// WithContentType sets the content type.
func (m *Message) WithContentType(contentType string) *Message {
	m.ContentType = contentType
	return m
}

// Publisher is the interface for message publishers.
type Publisher interface {
	// Publish publishes a message.
	Publish(ctx context.Context, message *Message) error

	// PublishBatch publishes multiple messages.
	PublishBatch(ctx context.Context, messages []*Message) error

	// Close closes the publisher.
	Close() error
}

// Handler is the function signature for message handlers.
type Handler func(ctx context.Context, message *Message) error

// Subscriber is the interface for message subscribers.
type Subscriber interface {
	// Subscribe subscribes to a topic.
	Subscribe(ctx context.Context, topic string, handler Handler) error

	// Unsubscribe unsubscribes from a topic.
	Unsubscribe(topic string) error

	// Close closes the subscriber.
	Close() error
}

// EventBus is the interface for in-process event buses.
type EventBus interface {
	// Publish publishes an event to a topic.
	Publish(topic string, event any) error

	// Subscribe subscribes to a topic.
	Subscribe(topic string, handler func(event any)) (func(), error)

	// SubscribeAsync subscribes with async handler.
	SubscribeAsync(topic string, handler func(event any)) (func(), error)
}

// QueueConfig configures a message queue.
type QueueConfig struct {
	// Name is the queue name.
	Name string
	// Durable indicates if the queue survives broker restart.
	Durable bool
	// AutoDelete indicates if the queue is deleted when unused.
	AutoDelete bool
	// MaxRetries is the maximum number of retries for failed messages.
	MaxRetries int
	// RetryDelay is the delay between retries.
	RetryDelay time.Duration
}

// DefaultQueueConfig returns a default queue configuration.
func DefaultQueueConfig(name string) QueueConfig {
	return QueueConfig{
		Name:       name,
		Durable:    true,
		AutoDelete: false,
		MaxRetries: 3,
		RetryDelay: time.Second,
	}
}

// Acknowledgment represents message acknowledgment options.
type Acknowledgment int

const (
	// AckAuto automatically acknowledges messages.
	AckAuto Acknowledgment = iota
	// AckManual requires manual acknowledgment.
	AckManual
	// AckNone does not acknowledge messages.
	AckNone
)

// ConsumerConfig configures a message consumer.
type ConsumerConfig struct {
	// Queue is the queue to consume from.
	Queue string
	// GroupID is the consumer group ID.
	GroupID string
	// Ack is the acknowledgment mode.
	Ack Acknowledgment
	// Concurrency is the number of concurrent handlers.
	Concurrency int
}

// DefaultConsumerConfig returns a default consumer configuration.
func DefaultConsumerConfig(queue string) ConsumerConfig {
	return ConsumerConfig{
		Queue:       queue,
		Ack:         AckManual,
		Concurrency: 1,
	}
}
