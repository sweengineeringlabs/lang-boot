package com.jboot.resilience.api;

import java.time.Duration;
import java.util.concurrent.Callable;
import java.util.function.Supplier;

/**
 * Circuit breaker pattern for fault tolerance.
 * <p>
 * Prevents repeated calls to a failing service by opening the circuit.
 */
public interface CircuitBreaker {

    /**
     * The current state of the circuit.
     */
    enum State {
        /** Circuit is closed, requests flow normally */
        CLOSED,
        /** Circuit is open, requests fail fast */
        OPEN,
        /** Circuit is testing if service has recovered */
        HALF_OPEN
    }

    /**
     * Gets the current state of the circuit.
     */
    State getState();

    /**
     * Gets the name of this circuit breaker.
     */
    String getName();

    /**
     * Executes the given supplier within the circuit breaker.
     *
     * @param supplier The operation to execute
     * @param <T>      The return type
     * @return The result of the operation
     * @throws CircuitBreakerOpenException if the circuit is open
     */
    <T> T execute(Supplier<T> supplier);

    /**
     * Executes the given callable within the circuit breaker.
     *
     * @param callable The operation to execute
     * @param <T>      The return type
     * @return The result of the operation
     * @throws Exception if the operation fails or circuit is open
     */
    <T> T executeChecked(Callable<T> callable) throws Exception;

    /**
     * Executes an action within the circuit breaker.
     *
     * @param action The action to execute
     * @throws CircuitBreakerOpenException if the circuit is open
     */
    void execute(Runnable action);

    /**
     * Forces the circuit to open.
     */
    void forceOpen();

    /**
     * Forces the circuit to close.
     */
    void forceClose();

    /**
     * Resets the circuit breaker statistics.
     */
    void reset();

    /**
     * Gets the failure rate (0.0 to 1.0).
     */
    double getFailureRate();

    /**
     * Creates a builder for circuit breaker configuration.
     */
    static CircuitBreakerBuilder builder(String name) {
        return new CircuitBreakerBuilder(name);
    }
}
