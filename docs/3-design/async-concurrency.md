# Async & Concurrency Conventions

Idiomatic async and concurrency patterns for each language.

---

## 🦀 Rust

### Async/Await

```rust
use tokio;

async fn fetch_user(id: i64) -> Result<User, Error> {
    let response = client.get(&format!("/users/{}", id)).await?;
    let user: User = response.json().await?;
    Ok(user)
}

#[tokio::main]
async fn main() {
    let user = fetch_user(1).await.unwrap();
}
```

### Parallel Execution

```rust
use tokio::join;

async fn fetch_all() -> (User, Orders) {
    let (user, orders) = join!(
        fetch_user(1),
        fetch_orders(1)
    );
    (user.unwrap(), orders.unwrap())
}
```

### Conventions

- Use `async fn` and `.await`
- Choose runtime: `tokio` (most common) or `async-std`
- `Send + Sync` required for thread safety
- Use `join!` for concurrent execution
- Use channels (`mpsc`) for communication

---

## 🦫 Go

### Goroutines & Channels

```go
func fetchUser(id int64) (*User, error) {
    // sync function - Go doesn't have async/await
    resp, err := http.Get(fmt.Sprintf("/users/%d", id))
    // ...
}

// Concurrent execution
func fetchAll() (*User, *Orders, error) {
    userCh := make(chan *User)
    ordersCh := make(chan *Orders)
    
    go func() {
        user, _ := fetchUser(1)
        userCh <- user
    }()
    
    go func() {
        orders, _ := fetchOrders(1)
        ordersCh <- orders
    }()
    
    return <-userCh, <-ordersCh, nil
}
```

### WaitGroup

```go
var wg sync.WaitGroup

for _, id := range ids {
    wg.Add(1)
    go func(id int64) {
        defer wg.Done()
        processUser(id)
    }(id)
}

wg.Wait()  // Wait for all goroutines
```

### Conventions

- Use `go func()` to spawn goroutines
- Use channels for communication
- Use `sync.WaitGroup` for synchronization
- Use `context.Context` for cancellation
- No async/await - blocking I/O is fine (goroutines are cheap)

---

## ☕ Java

### Virtual Threads (Java 21+)

```java
// Virtual threads - lightweight, can have millions
try (var executor = Executors.newVirtualThreadPerTaskExecutor()) {
    Future<User> userFuture = executor.submit(() -> fetchUser(1));
    Future<Orders> ordersFuture = executor.submit(() -> fetchOrders(1));
    
    User user = userFuture.get();
    Orders orders = ordersFuture.get();
}
```

### CompletableFuture

```java
CompletableFuture<User> userFuture = CompletableFuture.supplyAsync(() -> 
    fetchUser(1)
);

CompletableFuture<Orders> ordersFuture = CompletableFuture.supplyAsync(() -> 
    fetchOrders(1)
);

// Wait for both
CompletableFuture.allOf(userFuture, ordersFuture).join();
User user = userFuture.get();
Orders orders = ordersFuture.get();
```

### Conventions

- Use **Virtual Threads** (Java 21+) for I/O-bound work
- Use `CompletableFuture` for async composition
- Use `ExecutorService` for thread pools
- Avoid raw `Thread` creation
- Use `synchronized` or `Lock` for shared state

---

## 🐍 Python

### Asyncio

```python
import asyncio

async def fetch_user(user_id: int) -> User:
    response = await client.get(f"/users/{user_id}")
    return User(**response.json())

async def fetch_all():
    user, orders = await asyncio.gather(
        fetch_user(1),
        fetch_orders(1)
    )
    return user, orders

# Run
asyncio.run(fetch_all())
```

### Threading (CPU-bound with GIL)

```python
from concurrent.futures import ThreadPoolExecutor

with ThreadPoolExecutor(max_workers=4) as executor:
    futures = [executor.submit(process, item) for item in items]
    results = [f.result() for f in futures]
```

### Conventions

- Use `async def` and `await` for I/O-bound
- Use `asyncio.gather()` for concurrent execution
- Use `ThreadPoolExecutor` for blocking I/O in async code
- Use `ProcessPoolExecutor` for CPU-bound (bypasses GIL)
- Libraries must be async-compatible

---

## Comparison

| Aspect | Rust | Go | Java | Python |
|--------|------|-----|------|--------|
| Model | async/await | Goroutines | Virtual Threads | asyncio |
| Syntax | `async fn`, `.await` | `go func()` | `submit()` | `async def`, `await` |
| Parallel | `join!` | Channels | `CompletableFuture` | `gather()` |
| Sync | Channels, Mutex | Channels | `synchronized` | `asyncio.Lock` |
| Runtime | tokio, async-std | Built-in | JVM | asyncio |

---

## When to Use What

| Use Case | Rust | Go | Java | Python |
|----------|------|-----|------|--------|
| I/O-bound | async/await | goroutines | Virtual Threads | asyncio |
| CPU-bound | Rayon, threads | goroutines | ForkJoinPool | multiprocessing |
| Parallelism | `join!`, Rayon | goroutines | parallel streams | ProcessPool |
| Communication | channels | channels | BlockingQueue | asyncio.Queue |
