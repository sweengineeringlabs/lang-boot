package com.jboot.resilience.api;

import com.jboot.resilience.core.DefaultCircuitBreaker;

import java.time.Duration;

/**
 * Builder for CircuitBreaker configuration.
 */
public class CircuitBreakerBuilder {

    private final String name;
    private int failureThreshold = 5;
    private int successThreshold = 3;
    private Duration timeout = Duration.ofSeconds(30);
    private int slidingWindowSize = 10;

    public CircuitBreakerBuilder(String name) {
        this.name = name;
    }

    /**
     * Sets the number of failures before the circuit opens.
     */
    public CircuitBreakerBuilder failureThreshold(int threshold) {
        this.failureThreshold = threshold;
        return this;
    }

    /**
     * Sets the number of successes needed to close the circuit from half-open.
     */
    public CircuitBreakerBuilder successThreshold(int threshold) {
        this.successThreshold = threshold;
        return this;
    }

    /**
     * Sets the time the circuit stays open before transitioning to half-open.
     */
    public CircuitBreakerBuilder timeout(Duration timeout) {
        this.timeout = timeout;
        return this;
    }

    /**
     * Sets the sliding window size for calculating failure rate.
     */
    public CircuitBreakerBuilder slidingWindowSize(int size) {
        this.slidingWindowSize = size;
        return this;
    }

    /**
     * Builds the circuit breaker.
     */
    public CircuitBreaker build() {
        return new DefaultCircuitBreaker(
                name,
                failureThreshold,
                successThreshold,
                timeout,
                slidingWindowSize);
    }
}
