package com.jboot.resilience.core;

import com.jboot.resilience.api.CircuitBreaker;
import com.jboot.resilience.api.CircuitBreakerOpenException;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Nested;

import java.time.Duration;
import java.util.concurrent.atomic.AtomicInteger;

import static org.assertj.core.api.Assertions.*;

class DefaultCircuitBreakerTest {

    private CircuitBreaker circuitBreaker;

    @BeforeEach
    void setUp() {
        circuitBreaker = CircuitBreaker.builder("test")
                .failureThreshold(3)
                .successThreshold(2)
                .timeout(Duration.ofMillis(100))
                .build();
    }

    @Nested
    @DisplayName("Initial State")
    class InitialStateTests {
        @Test
        void startsInClosedState() {
            assertThat(circuitBreaker.getState()).isEqualTo(CircuitBreaker.State.CLOSED);
        }

        @Test
        void hasCorrectName() {
            assertThat(circuitBreaker.getName()).isEqualTo("test");
        }

        @Test
        void failureRateIsZeroInitially() {
            assertThat(circuitBreaker.getFailureRate()).isEqualTo(0.0);
        }
    }

    @Nested
    @DisplayName("Successful Execution")
    class SuccessfulExecutionTests {
        @Test
        void executeReturnsResult() {
            var result = circuitBreaker.execute(() -> "success");

            assertThat(result).isEqualTo("success");
        }

        @Test
        void executeRunnableCompletes() {
            var counter = new AtomicInteger(0);

            circuitBreaker.execute(counter::incrementAndGet);

            assertThat(counter.get()).isEqualTo(1);
        }

        @Test
        void staysClosedAfterSuccess() {
            circuitBreaker.execute(() -> "ok");

            assertThat(circuitBreaker.getState()).isEqualTo(CircuitBreaker.State.CLOSED);
        }
    }

    @Nested
    @DisplayName("Failure Handling")
    class FailureHandlingTests {
        @Test
        void propagatesException() {
            assertThatThrownBy(() -> circuitBreaker.execute(() -> {
                throw new RuntimeException("fail");
            })).isInstanceOf(RuntimeException.class)
                    .hasMessage("fail");
        }

        @Test
        void opensAfterThresholdFailures() {
            // Cause 3 failures (our threshold)
            for (int i = 0; i < 3; i++) {
                try {
                    circuitBreaker.execute(() -> {
                        throw new RuntimeException("fail");
                    });
                } catch (RuntimeException ignored) {
                }
            }

            assertThat(circuitBreaker.getState()).isEqualTo(CircuitBreaker.State.OPEN);
        }

        @Test
        void throwsCircuitBreakerOpenException() {
            // Open the circuit
            for (int i = 0; i < 3; i++) {
                try {
                    circuitBreaker.execute(() -> {
                        throw new RuntimeException("fail");
                    });
                } catch (RuntimeException ignored) {
                }
            }

            assertThatThrownBy(() -> circuitBreaker.execute(() -> "should fail fast"))
                    .isInstanceOf(CircuitBreakerOpenException.class)
                    .satisfies(e -> {
                        var cboe = (CircuitBreakerOpenException) e;
                        assertThat(cboe.getCircuitBreakerName()).isEqualTo("test");
                    });
        }
    }

    @Nested
    @DisplayName("State Transitions")
    class StateTransitionTests {
        @Test
        void transitionsToHalfOpenAfterTimeout() throws InterruptedException {
            // Open the circuit
            for (int i = 0; i < 3; i++) {
                try {
                    circuitBreaker.execute(() -> {
                        throw new RuntimeException("fail");
                    });
                } catch (RuntimeException ignored) {
                }
            }

            assertThat(circuitBreaker.getState()).isEqualTo(CircuitBreaker.State.OPEN);

            // Wait for timeout
            Thread.sleep(150);

            assertThat(circuitBreaker.getState()).isEqualTo(CircuitBreaker.State.HALF_OPEN);
        }

        @Test
        void closesAfterSuccessInHalfOpen() throws InterruptedException {
            // Open the circuit
            for (int i = 0; i < 3; i++) {
                try {
                    circuitBreaker.execute(() -> {
                        throw new RuntimeException("fail");
                    });
                } catch (RuntimeException ignored) {
                }
            }

            // Wait for transition to half-open
            Thread.sleep(150);

            // Successful calls to close
            circuitBreaker.execute(() -> "success1");
            circuitBreaker.execute(() -> "success2");

            assertThat(circuitBreaker.getState()).isEqualTo(CircuitBreaker.State.CLOSED);
        }

        @Test
        void reopensOnFailureInHalfOpen() throws InterruptedException {
            // Open the circuit
            for (int i = 0; i < 3; i++) {
                try {
                    circuitBreaker.execute(() -> {
                        throw new RuntimeException("fail");
                    });
                } catch (RuntimeException ignored) {
                }
            }

            // Wait for transition to half-open
            Thread.sleep(150);
            assertThat(circuitBreaker.getState()).isEqualTo(CircuitBreaker.State.HALF_OPEN);

            // Failure in half-open reopens
            try {
                circuitBreaker.execute(() -> {
                    throw new RuntimeException("fail");
                });
            } catch (RuntimeException ignored) {
            }

            assertThat(circuitBreaker.getState()).isEqualTo(CircuitBreaker.State.OPEN);
        }
    }

    @Nested
    @DisplayName("Manual Control")
    class ManualControlTests {
        @Test
        void forceOpenOpensCircuit() {
            circuitBreaker.forceOpen();

            assertThat(circuitBreaker.getState()).isEqualTo(CircuitBreaker.State.OPEN);
        }

        @Test
        void forceCloseClosesCircuit() {
            circuitBreaker.forceOpen();
            circuitBreaker.forceClose();

            assertThat(circuitBreaker.getState()).isEqualTo(CircuitBreaker.State.CLOSED);
        }

        @Test
        void resetClearsStatistics() {
            // Cause some failures
            for (int i = 0; i < 2; i++) {
                try {
                    circuitBreaker.execute(() -> {
                        throw new RuntimeException("fail");
                    });
                } catch (RuntimeException ignored) {
                }
            }

            circuitBreaker.reset();

            assertThat(circuitBreaker.getFailureRate()).isEqualTo(0.0);
        }
    }

    @Nested
    @DisplayName("Builder Configuration")
    class BuilderTests {
        @Test
        void customFailureThreshold() {
            var cb = CircuitBreaker.builder("custom")
                    .failureThreshold(5)
                    .build();

            // 4 failures shouldn't open
            for (int i = 0; i < 4; i++) {
                try {
                    cb.execute(() -> {
                        throw new RuntimeException();
                    });
                } catch (RuntimeException ignored) {
                }
            }
            assertThat(cb.getState()).isEqualTo(CircuitBreaker.State.CLOSED);

            // 5th failure opens
            try {
                cb.execute(() -> {
                    throw new RuntimeException();
                });
            } catch (RuntimeException ignored) {
            }
            assertThat(cb.getState()).isEqualTo(CircuitBreaker.State.OPEN);
        }
    }
}
