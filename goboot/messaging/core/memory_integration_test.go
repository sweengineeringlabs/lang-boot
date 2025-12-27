package core

import (
	"context"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"dev.engineeringlabs/goboot/messaging/api"
)

// Integration tests for messaging module

func TestEventBus_MultiTopicScenario(t *testing.T) {
	bus := NewEventBus()

	results := make(map[string][]string)
	var mu sync.Mutex

	record := func(topic, value string) {
		mu.Lock()
		results[topic] = append(results[topic], value)
		mu.Unlock()
	}

	// Subscribe to multiple topics
	unsub1, _ := bus.Subscribe("users.created", func(event any) {
		record("users.created", event.(string))
	})
	unsub2, _ := bus.Subscribe("users.updated", func(event any) {
		record("users.updated", event.(string))
	})
	unsub3, _ := bus.Subscribe("orders.placed", func(event any) {
		record("orders.placed", event.(string))
	})
	defer unsub1()
	defer unsub2()
	defer unsub3()

	// Publish events
	bus.Publish("users.created", "user-1")
	bus.Publish("users.updated", "user-2")
	bus.Publish("orders.placed", "order-1")
	bus.Publish("users.created", "user-3")

	// Allow time for processing
	time.Sleep(50 * time.Millisecond)

	mu.Lock()
	defer mu.Unlock()

	if len(results["users.created"]) != 2 {
		t.Errorf("Expected 2 users.created events, got %d", len(results["users.created"]))
	}
	if len(results["users.updated"]) != 1 {
		t.Errorf("Expected 1 users.updated event, got %d", len(results["users.updated"]))
	}
	if len(results["orders.placed"]) != 1 {
		t.Errorf("Expected 1 orders.placed event, got %d", len(results["orders.placed"]))
	}
}

func TestEventBus_BroadcastPattern(t *testing.T) {
	bus := NewEventBus()

	var count1, count2, count3 int32

	// Multiple subscribers to same topic
	bus.Subscribe("broadcast", func(event any) {
		atomic.AddInt32(&count1, 1)
	})
	bus.Subscribe("broadcast", func(event any) {
		atomic.AddInt32(&count2, 1)
	})
	bus.Subscribe("broadcast", func(event any) {
		atomic.AddInt32(&count3, 1)
	})

	// Broadcast 5 events
	for i := 0; i < 5; i++ {
		bus.Publish("broadcast", i)
	}

	time.Sleep(50 * time.Millisecond)

	// Each subscriber should receive 5 events
	if atomic.LoadInt32(&count1) != 5 || atomic.LoadInt32(&count2) != 5 || atomic.LoadInt32(&count3) != 5 {
		t.Errorf("Not all subscribers received all events: %d, %d, %d",
			count1, count2, count3)
	}
}

func TestEventBus_AsyncHandlers(t *testing.T) {
	bus := NewEventBus()

	var order []string
	var mu sync.Mutex

	record := func(s string) {
		mu.Lock()
		order = append(order, s)
		mu.Unlock()
	}

	// Sync subscriber
	bus.Subscribe("topic", func(event any) {
		time.Sleep(10 * time.Millisecond)
		record("sync")
	})

	// Async subscriber
	bus.SubscribeAsync("topic", func(event any) {
		time.Sleep(20 * time.Millisecond)
		record("async")
	})

	start := time.Now()
	bus.Publish("topic", "event")
	publishDuration := time.Since(start)

	// Publish should return after sync handler but not wait for async
	if publishDuration > 15*time.Millisecond {
		t.Error("Publish should return soon after sync handler completes")
	}

	// Wait for async to complete
	time.Sleep(50 * time.Millisecond)

	mu.Lock()
	if len(order) != 2 {
		t.Errorf("Expected 2 records, got %d", len(order))
	}
	mu.Unlock()
}

func TestPublisher_MessageOrdering(t *testing.T) {
	pub := NewInMemoryPublisher()
	ctx := context.Background()

	// Publish messages in order
	for i := 0; i < 100; i++ {
		msg := api.NewMessage("ordered.topic", []byte{byte(i)})
		pub.Publish(ctx, msg)
	}

	messages := pub.GetMessages()
	if len(messages) != 100 {
		t.Errorf("Expected 100 messages, got %d", len(messages))
	}

	// Verify ordering
	for i, msg := range messages {
		if msg.Payload[0] != byte(i) {
			t.Errorf("Message %d out of order", i)
		}
	}
}

func TestPublisher_WithHandler(t *testing.T) {
	pub := NewInMemoryPublisher()
	ctx := context.Background()

	var receivedMessages []*api.Message
	var mu sync.Mutex

	// Register handler
	pub.Publish(ctx, api.NewMessage("test.topic", []byte("1")))
	pub.Publish(ctx, api.NewMessage("test.topic", []byte("2")))

	// Get and process messages
	messages := pub.GetMessages()
	for _, msg := range messages {
		mu.Lock()
		receivedMessages = append(receivedMessages, msg)
		mu.Unlock()
	}

	if len(receivedMessages) != 2 {
		t.Errorf("Expected 2 messages, got %d", len(receivedMessages))
	}
}

func TestSubscriber_DeliverToCorrectHandler(t *testing.T) {
	sub := NewInMemorySubscriber()
	ctx := context.Background()

	results := make(map[string]int)
	var mu sync.Mutex

	sub.Subscribe(ctx, "topic.a", func(ctx context.Context, msg *api.Message) error {
		mu.Lock()
		results["a"]++
		mu.Unlock()
		return nil
	})

	sub.Subscribe(ctx, "topic.b", func(ctx context.Context, msg *api.Message) error {
		mu.Lock()
		results["b"]++
		mu.Unlock()
		return nil
	})

	// Deliver messages to different topics
	sub.Deliver(ctx, api.NewMessage("topic.a", []byte("1")))
	sub.Deliver(ctx, api.NewMessage("topic.a", []byte("2")))
	sub.Deliver(ctx, api.NewMessage("topic.b", []byte("3")))

	mu.Lock()
	if results["a"] != 2 {
		t.Errorf("Expected 2 messages for topic.a, got %d", results["a"])
	}
	if results["b"] != 1 {
		t.Errorf("Expected 1 message for topic.b, got %d", results["b"])
	}
	mu.Unlock()
}

func TestSubscriber_UnsubscribeStopsDelivery(t *testing.T) {
	sub := NewInMemorySubscriber()
	ctx := context.Background()

	var count int

	sub.Subscribe(ctx, "topic", func(ctx context.Context, msg *api.Message) error {
		count++
		return nil
	})

	// Deliver before unsubscribe
	sub.Deliver(ctx, api.NewMessage("topic", nil))
	if count != 1 {
		t.Error("Handler should be called")
	}

	// Unsubscribe
	sub.Unsubscribe("topic")

	// Deliver after unsubscribe
	sub.Deliver(ctx, api.NewMessage("topic", nil))
	if count != 1 {
		t.Error("Handler should not be called after unsubscribe")
	}
}

func TestTypedChannel_TypeSafety(t *testing.T) {
	type UserEvent struct {
		UserID string
		Action string
	}

	bus := NewEventBus()
	channel := NewChannel[UserEvent](bus, "user.events")

	received := make(chan UserEvent, 1)

	channel.Subscribe(func(event UserEvent) {
		received <- event
	})

	// Publish typed event
	channel.Publish(UserEvent{UserID: "123", Action: "login"})

	select {
	case event := <-received:
		if event.UserID != "123" || event.Action != "login" {
			t.Errorf("Unexpected event: %+v", event)
		}
	case <-time.After(time.Second):
		t.Error("Timeout waiting for event")
	}
}

func TestTypedChannel_IgnoresWrongTypes(t *testing.T) {
	bus := NewEventBus()
	channel := NewChannel[int](bus, "int.events")

	var received int32
	channel.Subscribe(func(event int) {
		atomic.AddInt32(&received, 1)
	})

	// Publish correct type
	channel.Publish(42)

	// Publish wrong type directly to bus (should be ignored by typed subscriber)
	bus.Publish("int.events", "string-value")

	time.Sleep(50 * time.Millisecond)

	if atomic.LoadInt32(&received) != 1 {
		t.Errorf("Should only receive 1 event, got %d", received)
	}
}

func TestPublisher_BatchPublish(t *testing.T) {
	pub := NewInMemoryPublisher()
	ctx := context.Background()

	messages := []*api.Message{
		api.NewMessage("batch.topic", []byte("1")),
		api.NewMessage("batch.topic", []byte("2")),
		api.NewMessage("batch.topic", []byte("3")),
	}

	err := pub.PublishBatch(ctx, messages)
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	if len(pub.GetMessages()) != 3 {
		t.Errorf("Expected 3 messages, got %d", len(pub.GetMessages()))
	}
}

// Benchmark tests

func BenchmarkEventBus_Publish(b *testing.B) {
	bus := NewEventBus()
	bus.Subscribe("topic", func(event any) {})

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		bus.Publish("topic", i)
	}
}

func BenchmarkEventBus_PublishAsync(b *testing.B) {
	bus := NewEventBus()
	bus.SubscribeAsync("topic", func(event any) {})

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		bus.Publish("topic", i)
	}
}

func BenchmarkPublisher_Publish(b *testing.B) {
	pub := NewInMemoryPublisher()
	ctx := context.Background()
	msg := api.NewMessage("topic", []byte("data"))

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		pub.Publish(ctx, msg)
	}
}

func BenchmarkTypedChannel_Publish(b *testing.B) {
	bus := NewEventBus()
	channel := NewChannel[int](bus, "int.events")
	channel.Subscribe(func(event int) {})

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		channel.Publish(i)
	}
}

func BenchmarkSubscriber_Deliver(b *testing.B) {
	sub := NewInMemorySubscriber()
	ctx := context.Background()
	sub.Subscribe(ctx, "topic", func(ctx context.Context, msg *api.Message) error {
		return nil
	})

	msg := api.NewMessage("topic", []byte("data"))
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		sub.Deliver(ctx, msg)
	}
}
