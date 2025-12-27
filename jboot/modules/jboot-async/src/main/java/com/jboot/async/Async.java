package com.jboot.async;

import com.jboot.async.api.*;
import com.jboot.async.core.*;
import java.time.Duration;
import java.util.List;
import java.util.concurrent.*;
import java.util.function.Function;
import java.util.function.Supplier;

/**
 * Async utilities for concurrent programming with virtual threads.
 * 
 * <p>
 * This module provides utilities for:
 * <ul>
 * <li>Structured concurrency with scoped tasks</li>
 * <li>Async/await patterns</li>
 * <li>Parallel execution utilities</li>
 * <li>Rate-limited async operations</li>
 * </ul>
 * 
 * <h2>Example Usage:</h2>
 * 
 * <pre>{@code
 * // Parallel execution
 * List<User> users = Async.parallel(
 *         () -> userService.findById(1),
 *         () -> userService.findById(2),
 *         () -> userService.findById(3));
 * 
 * // With timeout
 * var result = Async.withTimeout(Duration.ofSeconds(5), () -> {
 *     return slowService.fetchData();
 * });
 * 
 * // Structured concurrency
 * try (var scope = Async.scope()) {
 *     var future1 = scope.fork(() -> fetchUser(1));
 *     var future2 = scope.fork(() -> fetchOrders(1));
 * 
 *     scope.join();
 * 
 *     var user = future1.get();
 *     var orders = future2.get();
 * }
 * }</pre>
 */
public final class Async {

    private Async() {
    }

    /**
     * Executes tasks in parallel and collects results.
     */
    @SafeVarargs
    public static <T> List<T> parallel(Supplier<T>... tasks) {
        try (var executor = Executors.newVirtualThreadPerTaskExecutor()) {
            var futures = new ArrayList<Future<T>>();
            for (var task : tasks) {
                futures.add(executor.submit(task::get));
            }
            return futures.stream()
                    .map(Async::getUnchecked)
                    .toList();
        }
    }

    /**
     * Executes a task with a timeout.
     */
    public static <T> T withTimeout(Duration timeout, Supplier<T> task) {
        try (var executor = Executors.newVirtualThreadPerTaskExecutor()) {
            var future = executor.submit(task::get);
            return future.get(timeout.toMillis(), TimeUnit.MILLISECONDS);
        } catch (TimeoutException e) {
            throw new AsyncTimeoutException("Task timed out after " + timeout, e);
        } catch (Exception e) {
            throw new AsyncException("Task failed", e);
        }
    }

    /**
     * Creates a structured concurrency scope.
     */
    public static TaskScope scope() {
        return new VirtualThreadScope();
    }

    /**
     * Creates a rate-limited executor.
     */
    public static RateLimitedExecutor rateLimited(int maxConcurrent) {
        return new SemaphoreRateLimitedExecutor(maxConcurrent);
    }

    /**
     * Runs a task asynchronously and returns a CompletableFuture.
     */
    public static <T> CompletableFuture<T> async(Supplier<T> task) {
        return CompletableFuture.supplyAsync(task,
                Executors.newVirtualThreadPerTaskExecutor());
    }

    /**
     * Waits for all futures to complete.
     */
    public static <T> List<T> awaitAll(List<CompletableFuture<T>> futures) {
        return futures.stream()
                .map(CompletableFuture::join)
                .toList();
    }

    /**
     * Waits for any future to complete.
     */
    public static <T> T awaitAny(List<CompletableFuture<T>> futures) {
        return CompletableFuture.anyOf(futures.toArray(new CompletableFuture[0]))
                .thenApply(result -> (T) result)
                .join();
    }

    /**
     * Maps a list of items in parallel.
     */
    public static <T, R> List<R> mapParallel(List<T> items, Function<T, R> mapper) {
        return items.parallelStream()
                .map(mapper)
                .toList();
    }

    /**
     * Delays execution for a duration.
     */
    public static void delay(Duration duration) {
        try {
            Thread.sleep(duration);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            throw new AsyncException("Delay interrupted", e);
        }
    }

    private static <T> T getUnchecked(Future<T> future) {
        try {
            return future.get();
        } catch (Exception e) {
            throw new AsyncException("Task failed", e);
        }
    }
}
