# Rustboot E2E Example Application

This is a **reference example** demonstrating how to build a complete web application using the Rustboot framework patterns.

## Features Demonstrated

- HTTP server with Axum integration
- Dependency injection container
- Input validation
- Resilience patterns (circuit breaker, retry)
- Rate limiting
- Caching
- Health checks
- State machine for order processing
- Structured logging and observability

## Running

This example is excluded from the main workspace to avoid API drift. To run it:

```bash
cd examples/e2e-app
cargo run
```

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | /health | Health check |
| GET | /ready | Readiness probe |
| POST | /api/users | Create user |
| GET | /api/users/:id | Get user |
| POST | /api/orders | Create order |
| GET | /api/orders/:id | Get order |
| POST | /api/orders/:id/:action | Update order status |
| GET | /api/external | External API (with resilience) |

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        HTTP Layer                           │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────────────┐│
│  │ Health  │  │ Users   │  │ Orders  │  │ External API    ││
│  │ Check   │  │ API     │  │ API     │  │ (with retry)    ││
│  └─────────┘  └─────────┘  └─────────┘  └─────────────────┘│
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Middleware Layer                        │
│  ┌────────────┐  ┌──────────────┐  ┌────────────────────┐  │
│  │ Rate       │  │ Validation   │  │ Circuit Breaker    │  │
│  │ Limiting   │  │              │  │                    │  │
│  └────────────┘  └──────────────┘  └────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Service Layer                           │
│  ┌────────────┐  ┌──────────────┐  ┌────────────────────┐  │
│  │ User       │  │ Order        │  │ State Machine      │  │
│  │ Service    │  │ Service      │  │ (Order Flow)       │  │
│  └────────────┘  └──────────────┘  └────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Data Layer                              │
│  ┌────────────┐  ┌──────────────┐  ┌────────────────────┐  │
│  │ In-Memory  │  │ Cache        │  │ DI Container       │  │
│  │ Storage    │  │ (TTL-based)  │  │                    │  │
│  └────────────┘  └──────────────┘  └────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Order State Machine

```
    ┌─────────┐
    │ Pending │
    └────┬────┘
         │ confirm
         ▼
   ┌───────────┐
   │ Confirmed │
   └─────┬─────┘
         │ process
         ▼
  ┌────────────┐
  │ Processing │
  └──────┬─────┘
         │ ship
         ▼
    ┌─────────┐
    │ Shipped │
    └────┬────┘
         │ deliver
         ▼
   ┌───────────┐
   │ Delivered │
   └───────────┘

   * Cancel transitions available from Pending, Confirmed, Processing
```

## Testing

```bash
# Create a user
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","email":"test@example.com","password":"password123"}'

# Create an order
curl -X POST http://localhost:8080/api/orders \
  -H "Content-Type: application/json" \
  -d '{"user_id":"<uuid>","items":[{"product_id":"PROD-1","quantity":2,"price":29.99}]}'

# Update order status
curl -X POST http://localhost:8080/api/orders/<order-id>/confirm

# Health check
curl http://localhost:8080/health
```
