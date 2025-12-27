package core

import (
	"context"
	"sync"
	"testing"
	"time"

	"dev.engineeringlabs/goboot/messaging/api"
)

func TestInMemoryEventBus_PublishSubscribe(t *testing.T) {
	bus := NewEventBus()

	received := make(chan string, 1)

	unsubscribe, err := bus.Subscribe("test.topic", func(event any) {
		received <- event.(string)
	})
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}
	defer unsubscribe()

	bus.Publish("test.topic", "hello")

	select {
	case msg := <-received:
		if msg != "hello" {
			t.Errorf("Expected 'hello', got '%s'", msg)
		}
	case <-time.After(time.Second):
		t.Error("Timeout waiting for message")
	}
}

func TestInMemoryEventBus_MultipleSubscribers(t *testing.T) {
	bus := NewEventBus()

	var count int
	var mu sync.Mutex

	for i := 0; i < 3; i++ {
		bus.Subscribe("topic", func(event any) {
			mu.Lock()
			count++
			mu.Unlock()
		})
	}

	bus.Publish("topic", "event")

	time.Sleep(50 * time.Millisecond)

	mu.Lock()
	if count != 3 {
		t.Errorf("Expected 3 handlers called, got %d", count)
	}
	mu.Unlock()
}

func TestInMemoryEventBus_Unsubscribe(t *testing.T) {
	bus := NewEventBus()

	called := false
	unsubscribe, _ := bus.Subscribe("topic", func(event any) {
		called = true
	})

	// Unsubscribe before publish
	unsubscribe()

	bus.Publish("topic", "event")

	time.Sleep(50 * time.Millisecond)

	if called {
		t.Error("Handler should not be called after unsubscribe")
	}
}

func TestInMemoryEventBus_AsyncSubscribe(t *testing.T) {
	bus := NewEventBus()

	done := make(chan bool, 1)

	bus.SubscribeAsync("topic", func(event any) {
		time.Sleep(50 * time.Millisecond)
		done <- true
	})

	start := time.Now()
	bus.Publish("topic", "event")
	publishDuration := time.Since(start)

	// Publish should return immediately (async)
	if publishDuration > 20*time.Millisecond {
		t.Error("Publish should be non-blocking for async subscribers")
	}

	// Wait for async completion
	select {
	case <-done:
		// OK
	case <-time.After(time.Second):
		t.Error("Async handler did not complete")
	}
}

func TestInMemoryPublisher_Publish(t *testing.T) {
	pub := NewInMemoryPublisher()
	ctx := context.Background()

	msg := api.NewMessage("test.topic", []byte("hello"))

	err := pub.Publish(ctx, msg)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	messages := pub.GetMessages()
	if len(messages) != 1 {
		t.Errorf("Expected 1 message, got %d", len(messages))
	}
}

func TestInMemoryPublisher_GetMessagesByTopic(t *testing.T) {
	pub := NewInMemoryPublisher()
	ctx := context.Background()

	pub.Publish(ctx, api.NewMessage("topic1", []byte("a")))
	pub.Publish(ctx, api.NewMessage("topic2", []byte("b")))
	pub.Publish(ctx, api.NewMessage("topic1", []byte("c")))

	topic1Messages := pub.GetMessagesByTopic("topic1")
	if len(topic1Messages) != 2 {
		t.Errorf("Expected 2 messages for topic1, got %d", len(topic1Messages))
	}
}

func TestInMemoryPublisher_Clear(t *testing.T) {
	pub := NewInMemoryPublisher()
	ctx := context.Background()

	pub.Publish(ctx, api.NewMessage("topic", []byte("a")))
	pub.Publish(ctx, api.NewMessage("topic", []byte("b")))

	pub.Clear()

	messages := pub.GetMessages()
	if len(messages) != 0 {
		t.Errorf("Expected 0 messages after clear, got %d", len(messages))
	}
}

func TestInMemorySubscriber_Subscribe(t *testing.T) {
	sub := NewInMemorySubscriber()
	ctx := context.Background()

	var receivedPayload []byte

	sub.Subscribe(ctx, "topic", func(ctx context.Context, msg *api.Message) error {
		receivedPayload = msg.Payload
		return nil
	})

	// Deliver a message
	msg := api.NewMessage("topic", []byte("test"))
	sub.Deliver(ctx, msg)

	if string(receivedPayload) != "test" {
		t.Errorf("Expected 'test', got '%s'", string(receivedPayload))
	}
}

func TestInMemorySubscriber_Unsubscribe(t *testing.T) {
	sub := NewInMemorySubscriber()
	ctx := context.Background()

	called := false
	sub.Subscribe(ctx, "topic", func(ctx context.Context, msg *api.Message) error {
		called = true
		return nil
	})

	sub.Unsubscribe("topic")

	msg := api.NewMessage("topic", []byte("test"))
	sub.Deliver(ctx, msg)

	if called {
		t.Error("Handler should not be called after unsubscribe")
	}
}

func TestTypedChannel(t *testing.T) {
	bus := NewEventBus()
	channel := NewChannel[string](bus, "string.events")

	received := make(chan string, 1)

	unsubscribe, _ := channel.Subscribe(func(event string) {
		received <- event
	})
	defer unsubscribe()

	channel.Publish("typed message")

	select {
	case msg := <-received:
		if msg != "typed message" {
			t.Errorf("Expected 'typed message', got '%s'", msg)
		}
	case <-time.After(time.Second):
		t.Error("Timeout waiting for typed message")
	}
}

type TestEvent struct {
	ID   string
	Data string
}

func TestTypedChannel_Struct(t *testing.T) {
	bus := NewEventBus()
	channel := NewChannel[TestEvent](bus, "test.events")

	received := make(chan TestEvent, 1)

	channel.Subscribe(func(event TestEvent) {
		received <- event
	})

	channel.Publish(TestEvent{ID: "123", Data: "hello"})

	select {
	case event := <-received:
		if event.ID != "123" || event.Data != "hello" {
			t.Errorf("Unexpected event: %+v", event)
		}
	case <-time.After(time.Second):
		t.Error("Timeout waiting for struct event")
	}
}
