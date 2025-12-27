package com.jboot.async;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Timeout;

import java.time.Duration;
import java.util.List;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.atomic.AtomicInteger;

import static org.junit.jupiter.api.Assertions.*;

class AsyncTest {

    @Test
    void parallel_shouldExecuteAllTasks() {
        List<Integer> results = Async.parallel(
                () -> 1,
                () -> 2,
                () -> 3);

        assertEquals(3, results.size());
        assertTrue(results.containsAll(List.of(1, 2, 3)));
    }

    @Test
    void parallel_shouldExecuteConcurrently() {
        long start = System.currentTimeMillis();

        Async.parallel(
                () -> {
                    Thread.sleep(100);
                    return 1;
                },
                () -> {
                    Thread.sleep(100);
                    return 2;
                },
                () -> {
                    Thread.sleep(100);
                    return 3;
                });

        long elapsed = System.currentTimeMillis() - start;
        // Should take ~100ms, not ~300ms if running in parallel
        assertTrue(elapsed < 250, "Should run in parallel, took " + elapsed + "ms");
    }

    @Test
    @Timeout(2)
    void withTimeout_shouldReturnValue() {
        String result = Async.withTimeout(Duration.ofSeconds(1), () -> "success");
        assertEquals("success", result);
    }

    @Test
    void withTimeout_shouldThrowOnTimeout() {
        assertThrows(AsyncTimeoutException.class, () -> {
            Async.withTimeout(Duration.ofMillis(50), () -> {
                Thread.sleep(200);
                return "too late";
            });
        });
    }

    @Test
    void async_shouldReturnCompletableFuture() {
        CompletableFuture<String> future = Async.async(() -> "async result");

        assertNotNull(future);
        assertEquals("async result", future.join());
    }

    @Test
    void awaitAll_shouldWaitForAllFutures() {
        List<CompletableFuture<Integer>> futures = List.of(
                CompletableFuture.completedFuture(1),
                CompletableFuture.completedFuture(2),
                CompletableFuture.completedFuture(3));

        List<Integer> results = Async.awaitAll(futures);

        assertEquals(List.of(1, 2, 3), results);
    }

    @Test
    void awaitAny_shouldReturnFirstCompleted() {
        List<CompletableFuture<String>> futures = List.of(
                CompletableFuture.supplyAsync(() -> {
                    try {
                        Thread.sleep(100);
                    } catch (InterruptedException e) {
                    }
                    return "slow";
                }),
                CompletableFuture.completedFuture("fast"),
                CompletableFuture.supplyAsync(() -> {
                    try {
                        Thread.sleep(50);
                    } catch (InterruptedException e) {
                    }
                    return "medium";
                }));

        String result = Async.awaitAny(futures);
        assertEquals("fast", result);
    }

    @Test
    void mapParallel_shouldTransformItems() {
        List<Integer> items = List.of(1, 2, 3, 4, 5);

        List<Integer> results = Async.mapParallel(items, x -> x * 2);

        assertEquals(List.of(2, 4, 6, 8, 10), results);
    }

    @Test
    void rateLimited_shouldLimitConcurrency() throws InterruptedException {
        RateLimitedExecutor executor = Async.rateLimited(2);
        AtomicInteger maxConcurrent = new AtomicInteger(0);
        AtomicInteger currentConcurrent = new AtomicInteger(0);

        for (int i = 0; i < 10; i++) {
            executor.submit(() -> {
                int current = currentConcurrent.incrementAndGet();
                maxConcurrent.updateAndGet(max -> Math.max(max, current));
                try {
                    Thread.sleep(50);
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                }
                currentConcurrent.decrementAndGet();
            });
        }

        Thread.sleep(500);

        assertTrue(maxConcurrent.get() <= 2,
                "Max concurrent should be <= 2, was " + maxConcurrent.get());
    }

    @Test
    void delay_shouldPauseExecution() {
        long start = System.currentTimeMillis();

        Async.delay(Duration.ofMillis(100));

        long elapsed = System.currentTimeMillis() - start;
        assertTrue(elapsed >= 100, "Should delay at least 100ms");
    }
}
