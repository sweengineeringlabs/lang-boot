package com.jboot.resilience.core;

import com.jboot.resilience.api.RetryPolicy;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.time.Duration;
import java.util.concurrent.Callable;
import java.util.function.Predicate;
import java.util.function.Supplier;

/**
 * Default implementation of RetryPolicy.
 */
public class DefaultRetryPolicy implements RetryPolicy {

    private static final Logger log = LoggerFactory.getLogger(DefaultRetryPolicy.class);

    private final int maxAttempts;
    private final Duration delay;
    private final double backoffMultiplier;
    private final Duration maxDelay;
    private final Predicate<Throwable> retryPredicate;

    public DefaultRetryPolicy(
            int maxAttempts,
            Duration delay,
            double backoffMultiplier,
            Duration maxDelay,
            Predicate<Throwable> retryPredicate) {
        this.maxAttempts = maxAttempts;
        this.delay = delay;
        this.backoffMultiplier = backoffMultiplier;
        this.maxDelay = maxDelay;
        this.retryPredicate = retryPredicate;
    }

    @Override
    public <T> T execute(Supplier<T> supplier) {
        try {
            return executeChecked(() -> supplier.get());
        } catch (RuntimeException e) {
            throw e;
        } catch (Exception e) {
            throw new RuntimeException("Retry exhausted", e);
        }
    }

    @Override
    public <T> T executeChecked(Callable<T> callable) throws Exception {
        Exception lastException = null;
        long currentDelay = delay.toMillis();

        for (int attempt = 1; attempt <= maxAttempts; attempt++) {
            try {
                T result = callable.call();
                if (attempt > 1) {
                    log.info("Retry succeeded on attempt {}", attempt);
                }
                return result;
            } catch (Exception e) {
                lastException = e;

                if (attempt >= maxAttempts || !retryPredicate.test(e)) {
                    log.warn("Retry exhausted after {} attempts", attempt);
                    throw e;
                }

                log.debug("Attempt {} failed, retrying in {}ms: {}",
                        attempt, currentDelay, e.getMessage());

                try {
                    Thread.sleep(currentDelay);
                } catch (InterruptedException ie) {
                    Thread.currentThread().interrupt();
                    throw e;
                }

                // Calculate next delay with backoff
                currentDelay = Math.min(
                        (long) (currentDelay * backoffMultiplier),
                        maxDelay.toMillis());
            }
        }

        throw lastException;
    }

    @Override
    public void execute(Runnable action) {
        execute(() -> {
            action.run();
            return null;
        });
    }

    @Override
    public int getMaxAttempts() {
        return maxAttempts;
    }
}
