# Messaging Module Overview

## WHAT: Message Queue Abstractions

Event bus and pub/sub patterns.

Key capabilities:
- **Event Bus** - In-process events
- **Pub/Sub** - Publish/subscribe
- **Queues** - Message queues
- **Dead Letter** - Failed messages

## WHY: Decoupled Communication

**Problems Solved**: Tight coupling, async processing

**When to Use**: Microservices, event-driven

## HOW: Usage Guide

```go
bus := messaging.NewEventBus()

bus.Subscribe("user.created", func(event messaging.Event) {
    user := event.Data.(User)
    sendWelcomeEmail(user)
})

bus.Publish("user.created", user)
```

---

**Status**: Stable
