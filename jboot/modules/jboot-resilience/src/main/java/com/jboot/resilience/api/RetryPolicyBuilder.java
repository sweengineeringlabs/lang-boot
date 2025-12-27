package com.jboot.resilience.api;

import com.jboot.resilience.core.DefaultRetryPolicy;

import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;

/**
 * Builder for RetryPolicy configuration.
 */
public class RetryPolicyBuilder {

    private int maxAttempts = 3;
    private Duration delay = Duration.ofMillis(100);
    private double backoffMultiplier = 1.0;
    private Duration maxDelay = Duration.ofSeconds(30);
    private List<Class<? extends Throwable>> retryableExceptions = new ArrayList<>();
    private Predicate<Throwable> retryPredicate = t -> true;

    /**
     * Sets the maximum number of attempts.
     */
    public RetryPolicyBuilder maxAttempts(int maxAttempts) {
        this.maxAttempts = maxAttempts;
        return this;
    }

    /**
     * Sets the initial delay between retries.
     */
    public RetryPolicyBuilder delay(Duration delay) {
        this.delay = delay;
        return this;
    }

    /**
     * Sets the backoff multiplier for exponential backoff.
     */
    public RetryPolicyBuilder backoffMultiplier(double multiplier) {
        this.backoffMultiplier = multiplier;
        return this;
    }

    /**
     * Sets the maximum delay between retries.
     */
    public RetryPolicyBuilder maxDelay(Duration maxDelay) {
        this.maxDelay = maxDelay;
        return this;
    }

    /**
     * Adds an exception type that should trigger a retry.
     */
    public RetryPolicyBuilder retryOn(Class<? extends Throwable> exceptionClass) {
        this.retryableExceptions.add(exceptionClass);
        return this;
    }

    /**
     * Sets a predicate to determine if an exception should trigger a retry.
     */
    public RetryPolicyBuilder retryIf(Predicate<Throwable> predicate) {
        this.retryPredicate = predicate;
        return this;
    }

    /**
     * Builds the retry policy.
     */
    public RetryPolicy build() {
        Predicate<Throwable> finalPredicate = retryableExceptions.isEmpty()
                ? retryPredicate
                : t -> retryableExceptions.stream().anyMatch(c -> c.isInstance(t)) && retryPredicate.test(t);

        return new DefaultRetryPolicy(
                maxAttempts,
                delay,
                backoffMultiplier,
                maxDelay,
                finalPredicate);
    }
}
