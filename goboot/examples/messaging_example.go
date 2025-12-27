//go:build ignore

// Package main demonstrates the messaging module usage.
package main

import (
	"context"
	"fmt"
	"time"

	"dev.engineeringlabs/goboot/messaging"
)

// User is an example domain event.
type User struct {
	ID    string
	Name  string
	Email string
}

// OrderCreated is an example domain event.
type OrderCreated struct {
	OrderID string
	UserID  string
	Total   float64
}

func main() {
	fmt.Println("=== Goboot Messaging Module Example ===\n")
	ctx := context.Background()

	// Example 1: In-Memory Event Bus
	fmt.Println("1. In-Memory Event Bus:")
	bus := messaging.NewEventBus()

	// Subscribe to events
	unsubscribe, _ := bus.Subscribe("user.created", func(event any) {
		user := event.(User)
		fmt.Printf("   [Handler 1] User created: %s (%s)\n", user.Name, user.Email)
	})

	bus.Subscribe("user.created", func(event any) {
		user := event.(User)
		fmt.Printf("   [Handler 2] Sending welcome email to: %s\n", user.Email)
	})

	// Publish event
	bus.Publish("user.created", User{
		ID:    "user-123",
		Name:  "John Doe",
		Email: "john@example.com",
	})

	// Unsubscribe first handler
	unsubscribe()
	fmt.Println("   (First handler unsubscribed)")

	bus.Publish("user.created", User{
		ID:    "user-456",
		Name:  "Jane Doe",
		Email: "jane@example.com",
	})

	// Example 2: Typed Channels
	fmt.Println("\n2. Typed Channels:")
	orderChannel := messaging.NewChannel[OrderCreated](bus, "order.created")

	orderChannel.Subscribe(func(order OrderCreated) {
		fmt.Printf("   Order %s created for user %s: $%.2f\n",
			order.OrderID, order.UserID, order.Total)
	})

	orderChannel.Publish(OrderCreated{
		OrderID: "order-001",
		UserID:  "user-123",
		Total:   99.99,
	})

	// Example 3: In-Memory Publisher/Subscriber (for testing)
	fmt.Println("\n3. In-Memory Publisher/Subscriber:")
	publisher := messaging.NewInMemoryPublisher()
	subscriber := messaging.NewInMemorySubscriber()

	// Subscribe
	subscriber.Subscribe(ctx, "notifications", func(ctx context.Context, msg *messaging.Message) error {
		fmt.Printf("   Received: %s\n", string(msg.Payload))
		return nil
	})

	// Publish messages
	msg1 := messaging.NewMessage("notifications", []byte("Hello, World!"))
	msg1.WithKey("user-123").WithHeader("priority", "high")

	publisher.Publish(ctx, msg1)
	subscriber.Deliver(ctx, msg1)

	msg2 := messaging.NewMessage("notifications", []byte("Order shipped!"))
	publisher.Publish(ctx, msg2)
	subscriber.Deliver(ctx, msg2)

	// Check published messages
	allMessages := publisher.GetMessages()
	fmt.Printf("   Total messages published: %d\n", len(allMessages))

	// Example 4: Async Event Handling
	fmt.Println("\n4. Async Event Handling:")
	bus.SubscribeAsync("async.event", func(event any) {
		time.Sleep(100 * time.Millisecond)
		fmt.Printf("   Async handler completed: %v\n", event)
	})

	bus.Publish("async.event", "Processing in background...")
	fmt.Println("   Event published (handler runs async)")

	// Wait for async handler
	time.Sleep(200 * time.Millisecond)

	// Example 5: Message with metadata
	fmt.Println("\n5. Message with Metadata:")
	msg := messaging.NewMessage("orders", []byte(`{"order_id": "123"}`)).
		WithKey("order-123").
		WithContentType("application/json").
		WithHeader("correlation-id", "abc-123").
		WithHeader("source", "checkout-service")

	fmt.Printf("   Topic: %s\n", msg.Topic)
	fmt.Printf("   Key: %s\n", msg.Key)
	fmt.Printf("   ContentType: %s\n", msg.ContentType)
	fmt.Printf("   Headers: %v\n", msg.Headers)
}
