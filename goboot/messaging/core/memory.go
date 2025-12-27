// Package core contains the implementation details for the messaging module.
package core

import (
	"context"
	"sync"

	"dev.engineeringlabs/goboot/messaging/api"
)

// InMemoryEventBus is an in-process event bus implementation.
type InMemoryEventBus struct {
	subscribers map[string][]subscription
	mu          sync.RWMutex
}

type subscription struct {
	handler func(event any)
	async   bool
}

// NewEventBus creates a new InMemoryEventBus.
func NewEventBus() *InMemoryEventBus {
	return &InMemoryEventBus{
		subscribers: make(map[string][]subscription),
	}
}

// Publish publishes an event to a topic.
func (b *InMemoryEventBus) Publish(topic string, event any) error {
	b.mu.RLock()
	subs := b.subscribers[topic]
	b.mu.RUnlock()

	for _, sub := range subs {
		if sub.async {
			go sub.handler(event)
		} else {
			sub.handler(event)
		}
	}

	return nil
}

// Subscribe subscribes to a topic.
func (b *InMemoryEventBus) Subscribe(topic string, handler func(event any)) (func(), error) {
	return b.subscribe(topic, handler, false)
}

// SubscribeAsync subscribes with async handler.
func (b *InMemoryEventBus) SubscribeAsync(topic string, handler func(event any)) (func(), error) {
	return b.subscribe(topic, handler, true)
}

func (b *InMemoryEventBus) subscribe(topic string, handler func(event any), async bool) (func(), error) {
	b.mu.Lock()
	sub := subscription{handler: handler, async: async}
	b.subscribers[topic] = append(b.subscribers[topic], sub)
	index := len(b.subscribers[topic]) - 1
	b.mu.Unlock()

	// Return unsubscribe function
	return func() {
		b.mu.Lock()
		subs := b.subscribers[topic]
		if index < len(subs) {
			b.subscribers[topic] = append(subs[:index], subs[index+1:]...)
		}
		b.mu.Unlock()
	}, nil
}

// InMemoryPublisher is an in-memory publisher for testing.
type InMemoryPublisher struct {
	messages []*api.Message
	handlers map[string][]api.Handler
	mu       sync.RWMutex
}

// NewInMemoryPublisher creates a new InMemoryPublisher.
func NewInMemoryPublisher() *InMemoryPublisher {
	return &InMemoryPublisher{
		messages: make([]*api.Message, 0),
		handlers: make(map[string][]api.Handler),
	}
}

// Publish publishes a message.
func (p *InMemoryPublisher) Publish(ctx context.Context, message *api.Message) error {
	p.mu.Lock()
	p.messages = append(p.messages, message)
	handlers := p.handlers[message.Topic]
	p.mu.Unlock()

	// Notify handlers
	for _, handler := range handlers {
		if err := handler(ctx, message); err != nil {
			return err
		}
	}

	return nil
}

// PublishBatch publishes multiple messages.
func (p *InMemoryPublisher) PublishBatch(ctx context.Context, messages []*api.Message) error {
	for _, msg := range messages {
		if err := p.Publish(ctx, msg); err != nil {
			return err
		}
	}
	return nil
}

// Close closes the publisher.
func (p *InMemoryPublisher) Close() error {
	return nil
}

// GetMessages returns all published messages.
func (p *InMemoryPublisher) GetMessages() []*api.Message {
	p.mu.RLock()
	defer p.mu.RUnlock()
	return append([]*api.Message{}, p.messages...)
}

// GetMessagesByTopic returns messages for a specific topic.
func (p *InMemoryPublisher) GetMessagesByTopic(topic string) []*api.Message {
	p.mu.RLock()
	defer p.mu.RUnlock()

	var result []*api.Message
	for _, msg := range p.messages {
		if msg.Topic == topic {
			result = append(result, msg)
		}
	}
	return result
}

// Clear clears all messages.
func (p *InMemoryPublisher) Clear() {
	p.mu.Lock()
	p.messages = make([]*api.Message, 0)
	p.mu.Unlock()
}

// InMemorySubscriber is an in-memory subscriber for testing.
type InMemorySubscriber struct {
	handlers map[string]api.Handler
	mu       sync.RWMutex
}

// NewInMemorySubscriber creates a new InMemorySubscriber.
func NewInMemorySubscriber() *InMemorySubscriber {
	return &InMemorySubscriber{
		handlers: make(map[string]api.Handler),
	}
}

// Subscribe subscribes to a topic.
func (s *InMemorySubscriber) Subscribe(ctx context.Context, topic string, handler api.Handler) error {
	s.mu.Lock()
	s.handlers[topic] = handler
	s.mu.Unlock()
	return nil
}

// Unsubscribe unsubscribes from a topic.
func (s *InMemorySubscriber) Unsubscribe(topic string) error {
	s.mu.Lock()
	delete(s.handlers, topic)
	s.mu.Unlock()
	return nil
}

// Close closes the subscriber.
func (s *InMemorySubscriber) Close() error {
	return nil
}

// Deliver delivers a message to handlers (for testing).
func (s *InMemorySubscriber) Deliver(ctx context.Context, message *api.Message) error {
	s.mu.RLock()
	handler := s.handlers[message.Topic]
	s.mu.RUnlock()

	if handler != nil {
		return handler(ctx, message)
	}
	return nil
}

// Channel creates a typed pub/sub channel.
type Channel[T any] struct {
	bus   *InMemoryEventBus
	topic string
}

// NewChannel creates a new typed channel.
func NewChannel[T any](bus *InMemoryEventBus, topic string) *Channel[T] {
	return &Channel[T]{
		bus:   bus,
		topic: topic,
	}
}

// Publish publishes a typed event.
func (c *Channel[T]) Publish(event T) error {
	return c.bus.Publish(c.topic, event)
}

// Subscribe subscribes to typed events.
func (c *Channel[T]) Subscribe(handler func(event T)) (func(), error) {
	return c.bus.Subscribe(c.topic, func(event any) {
		if typed, ok := event.(T); ok {
			handler(typed)
		}
	})
}
