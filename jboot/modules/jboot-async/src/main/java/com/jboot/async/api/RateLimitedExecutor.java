package com.jboot.async.api;

/**
 * Rate-limited executor interface.
 */
public interface RateLimitedExecutor {

    /**
     * Submits a task for execution.
     */
    void submit(Runnable task);

    /**
     * Attempts to execute immediately without blocking.
     */
    boolean tryExecute(Runnable task);

    /**
     * Returns the maximum concurrent executions.
     */
    int maxConcurrent();

    /**
     * Returns the current number of active executions.
     */
    int active();
}
