package com.jboot.resilience.api;

import java.time.Duration;
import java.util.concurrent.Callable;
import java.util.function.Predicate;
import java.util.function.Supplier;

/**
 * Retry policy for transient failure handling.
 */
public interface RetryPolicy {

    /**
     * Executes the given supplier with retry logic.
     *
     * @param supplier The operation to execute
     * @param <T>      The return type
     * @return The result of the operation
     */
    <T> T execute(Supplier<T> supplier);

    /**
     * Executes the given callable with retry logic.
     *
     * @param callable The operation to execute
     * @param <T>      The return type
     * @return The result of the operation
     * @throws Exception if all retries are exhausted
     */
    <T> T executeChecked(Callable<T> callable) throws Exception;

    /**
     * Executes the given action with retry logic.
     *
     * @param action The action to execute
     */
    void execute(Runnable action);

    /**
     * Gets the maximum number of attempts.
     */
    int getMaxAttempts();

    /**
     * Creates a builder for retry policy configuration.
     */
    static RetryPolicyBuilder builder() {
        return new RetryPolicyBuilder();
    }
}
