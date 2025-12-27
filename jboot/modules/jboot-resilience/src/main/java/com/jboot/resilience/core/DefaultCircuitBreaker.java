package com.jboot.resilience.core;

import com.jboot.resilience.api.CircuitBreaker;
import com.jboot.resilience.api.CircuitBreakerOpenException;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.time.Duration;
import java.time.Instant;
import java.util.concurrent.Callable;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicReference;
import java.util.function.Supplier;

/**
 * Default implementation of CircuitBreaker.
 */
public class DefaultCircuitBreaker implements CircuitBreaker {

    private static final Logger log = LoggerFactory.getLogger(DefaultCircuitBreaker.class);

    private final String name;
    private final int failureThreshold;
    private final int successThreshold;
    private final Duration timeout;
    private final int slidingWindowSize;

    private final AtomicReference<State> state = new AtomicReference<>(State.CLOSED);
    private final AtomicInteger failureCount = new AtomicInteger(0);
    private final AtomicInteger successCount = new AtomicInteger(0);
    private final AtomicInteger totalCount = new AtomicInteger(0);
    private volatile Instant openedAt = null;

    public DefaultCircuitBreaker(
            String name,
            int failureThreshold,
            int successThreshold,
            Duration timeout,
            int slidingWindowSize) {
        this.name = name;
        this.failureThreshold = failureThreshold;
        this.successThreshold = successThreshold;
        this.timeout = timeout;
        this.slidingWindowSize = slidingWindowSize;
    }

    @Override
    public State getState() {
        // Check if we should transition from OPEN to HALF_OPEN
        if (state.get() == State.OPEN && openedAt != null) {
            if (Instant.now().isAfter(openedAt.plus(timeout))) {
                if (state.compareAndSet(State.OPEN, State.HALF_OPEN)) {
                    log.info("Circuit breaker '{}' transitioned to HALF_OPEN", name);
                    successCount.set(0);
                }
            }
        }
        return state.get();
    }

    @Override
    public String getName() {
        return name;
    }

    @Override
    public <T> T execute(Supplier<T> supplier) {
        checkState();
        try {
            T result = supplier.get();
            recordSuccess();
            return result;
        } catch (Exception e) {
            recordFailure();
            throw e;
        }
    }

    @Override
    public <T> T executeChecked(Callable<T> callable) throws Exception {
        checkState();
        try {
            T result = callable.call();
            recordSuccess();
            return result;
        } catch (Exception e) {
            recordFailure();
            throw e;
        }
    }

    @Override
    public void execute(Runnable action) {
        execute(() -> {
            action.run();
            return null;
        });
    }

    private void checkState() {
        State currentState = getState();
        if (currentState == State.OPEN) {
            throw new CircuitBreakerOpenException(name);
        }
    }

    private void recordSuccess() {
        totalCount.incrementAndGet();
        failureCount.set(0); // Reset consecutive failures

        if (state.get() == State.HALF_OPEN) {
            int successes = successCount.incrementAndGet();
            if (successes >= successThreshold) {
                if (state.compareAndSet(State.HALF_OPEN, State.CLOSED)) {
                    log.info("Circuit breaker '{}' transitioned to CLOSED", name);
                    reset();
                }
            }
        }
    }

    private void recordFailure() {
        totalCount.incrementAndGet();
        int failures = failureCount.incrementAndGet();

        if (state.get() == State.HALF_OPEN) {
            // Any failure in half-open reopens the circuit
            if (state.compareAndSet(State.HALF_OPEN, State.OPEN)) {
                openedAt = Instant.now();
                log.warn("Circuit breaker '{}' reopened after failure in HALF_OPEN", name);
            }
        } else if (state.get() == State.CLOSED && failures >= failureThreshold) {
            if (state.compareAndSet(State.CLOSED, State.OPEN)) {
                openedAt = Instant.now();
                log.warn("Circuit breaker '{}' opened after {} failures", name, failures);
            }
        }
    }

    @Override
    public void forceOpen() {
        state.set(State.OPEN);
        openedAt = Instant.now();
        log.info("Circuit breaker '{}' forced OPEN", name);
    }

    @Override
    public void forceClose() {
        state.set(State.CLOSED);
        reset();
        log.info("Circuit breaker '{}' forced CLOSED", name);
    }

    @Override
    public void reset() {
        failureCount.set(0);
        successCount.set(0);
        totalCount.set(0);
        openedAt = null;
    }

    @Override
    public double getFailureRate() {
        int total = totalCount.get();
        if (total == 0)
            return 0.0;
        return (double) failureCount.get() / Math.min(total, slidingWindowSize);
    }
}
