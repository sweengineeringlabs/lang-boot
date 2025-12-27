# JBoot Examples

Comprehensive examples demonstrating JBoot framework usage.

## Quick Start Examples

### 1. Validation Example

```java
package com.jboot.examples.validation;

import com.jboot.validation.Validator;
import com.jboot.validation.ValidationResult;
import com.jboot.validation.constraints.*;

public class ValidationExample {
    
    public static void main(String[] args) {
        // Build a validator
        var validator = Validator.builder()
            .field("username")
                .notEmpty()
                .length(3, 50)
                .pattern("[a-zA-Z0-9_]+")
            .field("email")
                .notEmpty()
                .email()
            .field("age")
                .range(18, 120)
            .field("password")
                .notEmpty()
                .length(8, 100)
            .build();
        
        // Validate data
        var data = Map.of(
            "username", "john_doe",
            "email", "john@example.com",
            "age", 25,
            "password", "securePass123"
        );
        
        ValidationResult result = validator.validate(data);
        
        if (result.isValid()) {
            System.out.println("Validation passed!");
        } else {
            result.getErrors().forEach(error -> 
                System.err.println(error.getField() + ": " + error.getMessage())
            );
        }
    }
}
```

### 2. Caching Example

```java
package com.jboot.examples.cache;

import com.jboot.cache.Cache;
import com.jboot.cache.CacheConfig;
import java.time.Duration;

public class CacheExample {
    
    public static void main(String[] args) {
        // In-memory cache
        var cache = Cache.inMemory(CacheConfig.builder()
            .maxSize(1000)
            .defaultTtl(Duration.ofMinutes(5))
            .build());
        
        // Basic operations
        cache.set("user:123", new User("John", "john@example.com"));
        
        var user = cache.get("user:123", User.class);
        user.ifPresent(u -> System.out.println("Found: " + u.getName()));
        
        // With custom TTL
        cache.set("session:abc", sessionData, Duration.ofHours(1));
        
        // Compute if absent
        var cachedUser = cache.computeIfAbsent("user:456", 
            key -> userRepository.findById(456));
        
        // Delete
        cache.delete("user:123");
        
        // Clear all
        cache.clear();
    }
}
```

### 3. Circuit Breaker Example

```java
package com.jboot.examples.resilience;

import com.jboot.resilience.CircuitBreaker;
import com.jboot.resilience.CircuitBreakerConfig;
import java.time.Duration;

public class CircuitBreakerExample {
    
    private final CircuitBreaker circuitBreaker;
    private final ExternalService externalService;
    
    public CircuitBreakerExample() {
        this.circuitBreaker = CircuitBreaker.builder("external-api")
            .failureThreshold(5)
            .successThreshold(3)
            .timeout(Duration.ofSeconds(30))
            .halfOpenMaxCalls(3)
            .build();
        
        this.externalService = new ExternalService();
    }
    
    public String callExternalService(String request) {
        return circuitBreaker.execute(() -> {
            return externalService.call(request);
        });
    }
    
    public static void main(String[] args) {
        var example = new CircuitBreakerExample();
        
        // Normal execution
        try {
            String result = example.callExternalService("hello");
            System.out.println("Result: " + result);
        } catch (Exception e) {
            System.err.println("Failed: " + e.getMessage());
        }
        
        // Check circuit state
        System.out.println("Circuit state: " + example.circuitBreaker.getState());
    }
}
```

### 4. Retry Example

```java
package com.jboot.examples.resilience;

import com.jboot.resilience.RetryPolicy;
import com.jboot.resilience.BackoffStrategy;
import java.time.Duration;

public class RetryExample {
    
    public static void main(String[] args) {
        var retryPolicy = RetryPolicy.builder()
            .maxAttempts(3)
            .delay(Duration.ofMillis(100))
            .backoff(BackoffStrategy.EXPONENTIAL)
            .maxDelay(Duration.ofSeconds(5))
            .retryOn(IOException.class, TimeoutException.class)
            .build();
        
        String result = retryPolicy.execute(() -> {
            // This might fail and will be retried
            return httpClient.get("https://api.example.com/data");
        });
        
        System.out.println("Result: " + result);
    }
}
```

### 5. Rate Limiting Example

```java
package com.jboot.examples.ratelimit;

import com.jboot.ratelimit.RateLimiter;
import com.jboot.ratelimit.RateLimiterConfig;

public class RateLimitExample {
    
    public static void main(String[] args) {
        // Token bucket rate limiter
        var rateLimiter = RateLimiter.tokenBucket(RateLimiterConfig.builder()
            .capacity(100)           // Max 100 tokens
            .refillRate(10)          // 10 tokens per second
            .build());
        
        // Try to acquire
        if (rateLimiter.tryAcquire()) {
            processRequest();
        } else {
            throw new RateLimitExceededException("Too many requests");
        }
        
        // With permit count
        if (rateLimiter.tryAcquire(5)) {
            processBatchRequest();
        }
        
        // Blocking acquire
        rateLimiter.acquire(); // Blocks until permit available
        processRequest();
    }
}
```

### 6. State Machine Example

```java
package com.jboot.examples.statemachine;

import com.jboot.statemachine.StateMachine;
import com.jboot.statemachine.StateConfig;

public class OrderStateMachine {
    
    enum State { CREATED, PAID, SHIPPED, DELIVERED, CANCELLED }
    enum Event { PAY, SHIP, DELIVER, CANCEL }
    
    public static void main(String[] args) {
        var stateMachine = StateMachine.<State, Event>builder()
            .initialState(State.CREATED)
            
            // Define transitions
            .transition(State.CREATED, Event.PAY, State.PAID)
                .guard(order -> order.hasValidPayment())
                .action(order -> order.processPayment())
            
            .transition(State.PAID, Event.SHIP, State.SHIPPED)
                .guard(order -> order.hasInventory())
                .action(order -> order.createShipment())
            
            .transition(State.SHIPPED, Event.DELIVER, State.DELIVERED)
                .action(order -> order.markDelivered())
            
            // Cancel from any state except DELIVERED
            .transition(State.CREATED, Event.CANCEL, State.CANCELLED)
            .transition(State.PAID, Event.CANCEL, State.CANCELLED)
                .action(order -> order.processRefund())
            
            .build();
        
        // Use the state machine
        var order = new Order();
        stateMachine.fire(Event.PAY, order);
        System.out.println("State: " + stateMachine.getCurrentState()); // PAID
        
        stateMachine.fire(Event.SHIP, order);
        System.out.println("State: " + stateMachine.getCurrentState()); // SHIPPED
    }
}
```

### 7. Dependency Injection Example

```java
package com.jboot.examples.di;

import com.jboot.di.Container;
import com.jboot.di.Scope;

public class DIExample {
    
    public static void main(String[] args) {
        var container = Container.builder()
            // Register singleton
            .register(DatabasePool.class, Scope.SINGLETON)
            
            // Register with factory
            .register(UserRepository.class, () -> 
                new UserRepositoryImpl(container.resolve(DatabasePool.class)))
            
            // Register interface to implementation
            .register(UserService.class, UserServiceImpl.class)
            
            .build();
        
        // Resolve dependencies
        var userService = container.resolve(UserService.class);
        var user = userService.findById(123);
        
        // Child scope
        try (var requestScope = container.createScope()) {
            requestScope.register(RequestContext.class, new RequestContext(request));
            var handler = requestScope.resolve(RequestHandler.class);
            handler.handle();
        }
    }
}
```

### 8. HTTP Client Example

```java
package com.jboot.examples.http;

import com.jboot.http.HttpClient;
import com.jboot.http.HttpRequest;
import com.jboot.http.HttpResponse;

public class HttpClientExample {
    
    public static void main(String[] args) {
        var client = HttpClient.builder()
            .baseUrl("https://api.example.com")
            .timeout(Duration.ofSeconds(30))
            .header("Authorization", "Bearer " + token)
            .build();
        
        // GET request
        HttpResponse<User> response = client.get("/users/123", User.class);
        if (response.isSuccess()) {
            System.out.println("User: " + response.getBody().getName());
        }
        
        // POST request
        var newUser = new CreateUserRequest("John", "john@example.com");
        HttpResponse<User> created = client.post("/users", newUser, User.class);
        
        // With request customization
        HttpResponse<String> custom = client.request(HttpRequest.builder()
            .method("PATCH")
            .path("/users/123")
            .body(Map.of("name", "Jane"))
            .header("X-Custom", "value")
            .build(), String.class);
    }
}
```

## Running Examples

```bash
# Compile examples
mvn compile -pl examples

# Run specific example
mvn exec:java -pl examples -Dexec.mainClass="com.jboot.examples.validation.ValidationExample"

# Run all example tests
mvn test -pl examples
```

## Project Structure

```
examples/
├── src/main/java/com/jboot/examples/
│   ├── cache/
│   │   └── CacheExample.java
│   ├── di/
│   │   └── DIExample.java
│   ├── http/
│   │   └── HttpClientExample.java
│   ├── ratelimit/
│   │   └── RateLimitExample.java
│   ├── resilience/
│   │   ├── CircuitBreakerExample.java
│   │   └── RetryExample.java
│   ├── statemachine/
│   │   └── OrderStateMachine.java
│   └── validation/
│       └── ValidationExample.java
└── pom.xml
```
