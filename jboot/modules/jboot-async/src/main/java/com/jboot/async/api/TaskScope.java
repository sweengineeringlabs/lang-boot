package com.jboot.async.api;

import java.time.Duration;
import java.util.concurrent.CompletableFuture;
import java.util.function.Supplier;

/**
 * Task scope for structured concurrency.
 */
public interface TaskScope extends AutoCloseable {

    /**
     * Forks a new task.
     */
    <T> CompletableFuture<T> fork(Supplier<T> task);

    /**
     * Waits for all forked tasks to complete.
     */
    void join() throws InterruptedException;

    /**
     * Waits for all forked tasks with a timeout.
     */
    void join(Duration timeout) throws InterruptedException;

    /**
     * Cancels all forked tasks.
     */
    void cancel();

    @Override
    void close();
}
