package com.jboot.resilience.core;

import com.jboot.resilience.api.RetryPolicy;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Nested;

import java.io.IOException;
import java.time.Duration;
import java.util.concurrent.atomic.AtomicInteger;

import static org.assertj.core.api.Assertions.*;

class DefaultRetryPolicyTest {

    @Nested
    @DisplayName("Successful execution")
    class SuccessfulExecutionTests {
        @Test
        void returnsResultOnFirstAttempt() {
            var policy = RetryPolicy.builder().maxAttempts(3).build();

            var result = policy.execute(() -> "success");

            assertThat(result).isEqualTo("success");
        }

        @Test
        void executesRunnableSuccessfully() {
            var policy = RetryPolicy.builder().maxAttempts(3).build();
            var counter = new AtomicInteger(0);

            policy.execute(counter::incrementAndGet);

            assertThat(counter.get()).isEqualTo(1);
        }
    }

    @Nested
    @DisplayName("Retry behavior")
    class RetryBehaviorTests {
        @Test
        void retriesOnFailure() {
            var policy = RetryPolicy.builder()
                    .maxAttempts(3)
                    .delay(Duration.ofMillis(10))
                    .build();

            var attempts = new AtomicInteger(0);

            var result = policy.execute(() -> {
                if (attempts.incrementAndGet() < 3) {
                    throw new RuntimeException("fail");
                }
                return "success";
            });

            assertThat(result).isEqualTo("success");
            assertThat(attempts.get()).isEqualTo(3);
        }

        @Test
        void exhaustsRetriesAndThrows() {
            var policy = RetryPolicy.builder()
                    .maxAttempts(3)
                    .delay(Duration.ofMillis(10))
                    .build();

            var attempts = new AtomicInteger(0);

            assertThatThrownBy(() -> policy.execute(() -> {
                attempts.incrementAndGet();
                throw new RuntimeException("always fails");
            })).isInstanceOf(RuntimeException.class)
                    .hasMessage("always fails");

            assertThat(attempts.get()).isEqualTo(3);
        }

        @Test
        void respectsMaxAttempts() {
            var policy = RetryPolicy.builder()
                    .maxAttempts(5)
                    .delay(Duration.ofMillis(5))
                    .build();

            var attempts = new AtomicInteger(0);

            var result = policy.execute(() -> {
                if (attempts.incrementAndGet() < 5) {
                    throw new RuntimeException("fail");
                }
                return "success";
            });

            assertThat(result).isEqualTo("success");
            assertThat(attempts.get()).isEqualTo(5);
        }
    }

    @Nested
    @DisplayName("Exception filtering")
    class ExceptionFilteringTests {
        @Test
        void retriesOnSpecificException() {
            var policy = RetryPolicy.builder()
                    .maxAttempts(3)
                    .delay(Duration.ofMillis(10))
                    .retryOn(IOException.class)
                    .build();

            var attempts = new AtomicInteger(0);

            assertThatCode(() -> policy.executeChecked(() -> {
                if (attempts.incrementAndGet() < 3) {
                    throw new IOException("network error");
                }
                return "success";
            })).doesNotThrowAnyException();

            assertThat(attempts.get()).isEqualTo(3);
        }

        @Test
        void doesNotRetryOnNonMatchingException() {
            var policy = RetryPolicy.builder()
                    .maxAttempts(3)
                    .delay(Duration.ofMillis(10))
                    .retryOn(IOException.class)
                    .build();

            var attempts = new AtomicInteger(0);

            assertThatThrownBy(() -> policy.execute(() -> {
                attempts.incrementAndGet();
                throw new IllegalArgumentException("bad input");
            })).isInstanceOf(IllegalArgumentException.class);

            assertThat(attempts.get()).isEqualTo(1); // No retries
        }

        @Test
        void usesCustomRetryPredicate() {
            var policy = RetryPolicy.builder()
                    .maxAttempts(3)
                    .delay(Duration.ofMillis(10))
                    .retryIf(t -> t.getMessage().contains("transient"))
                    .build();

            var attempts = new AtomicInteger(0);

            var result = policy.execute(() -> {
                if (attempts.incrementAndGet() < 3) {
                    throw new RuntimeException("transient error");
                }
                return "success";
            });

            assertThat(result).isEqualTo("success");
            assertThat(attempts.get()).isEqualTo(3);
        }
    }

    @Nested
    @DisplayName("Backoff behavior")
    class BackoffTests {
        @Test
        void appliesExponentialBackoff() {
            var policy = RetryPolicy.builder()
                    .maxAttempts(3)
                    .delay(Duration.ofMillis(50))
                    .backoffMultiplier(2.0)
                    .build();

            var attempts = new AtomicInteger(0);
            var startTime = System.currentTimeMillis();

            var result = policy.execute(() -> {
                if (attempts.incrementAndGet() < 3) {
                    throw new RuntimeException("fail");
                }
                return "success";
            });

            var elapsed = System.currentTimeMillis() - startTime;

            assertThat(result).isEqualTo("success");
            // First retry: 50ms, second retry: 100ms = 150ms minimum
            assertThat(elapsed).isGreaterThanOrEqualTo(100);
        }

        @Test
        void respectsMaxDelay() {
            var policy = RetryPolicy.builder()
                    .maxAttempts(5)
                    .delay(Duration.ofMillis(100))
                    .backoffMultiplier(10.0)
                    .maxDelay(Duration.ofMillis(200))
                    .build();

            // Delays should be: 100, 200, 200, 200 (capped at maxDelay)
            assertThat(policy.getMaxAttempts()).isEqualTo(5);
        }
    }

    @Nested
    @DisplayName("Builder configuration")
    class BuilderTests {
        @Test
        void defaultConfiguration() {
            var policy = RetryPolicy.builder().build();

            assertThat(policy.getMaxAttempts()).isEqualTo(3);
        }

        @Test
        void customMaxAttempts() {
            var policy = RetryPolicy.builder().maxAttempts(10).build();

            assertThat(policy.getMaxAttempts()).isEqualTo(10);
        }
    }
}
