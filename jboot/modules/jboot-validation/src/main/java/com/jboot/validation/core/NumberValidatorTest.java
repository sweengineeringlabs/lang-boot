package com.jboot.validation.core;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Nested;

import static org.assertj.core.api.Assertions.*;

class NumberValidatorTest {

    @Nested
    @DisplayName("notNull validation")
    class NotNullTests {
        @Test
        void passesForNonNullValue() {
            var validator = NumberValidator.<Integer>forField("age").notNull();

            assertThat(validator.validate(25).isValid()).isTrue();
        }

        @Test
        void failsForNullValue() {
            var validator = NumberValidator.<Integer>forField("age").notNull();

            assertThat(validator.validate(null).isValid()).isFalse();
        }
    }

    @Nested
    @DisplayName("min validation")
    class MinTests {
        @Test
        void passesForValueAboveMin() {
            var validator = NumberValidator.<Integer>forField("age").min(18);

            assertThat(validator.validate(25).isValid()).isTrue();
        }

        @Test
        void passesForValueEqualToMin() {
            var validator = NumberValidator.<Integer>forField("age").min(18);

            assertThat(validator.validate(18).isValid()).isTrue();
        }

        @Test
        void failsForValueBelowMin() {
            var validator = NumberValidator.<Integer>forField("age").min(18);

            var result = validator.validate(16);
            assertThat(result.isValid()).isFalse();
            assertThat(result.getErrors().get(0).message()).contains("at least 18");
        }
    }

    @Nested
    @DisplayName("max validation")
    class MaxTests {
        @Test
        void passesForValueBelowMax() {
            var validator = NumberValidator.<Integer>forField("quantity").max(100);

            assertThat(validator.validate(50).isValid()).isTrue();
        }

        @Test
        void passesForValueEqualToMax() {
            var validator = NumberValidator.<Integer>forField("quantity").max(100);

            assertThat(validator.validate(100).isValid()).isTrue();
        }

        @Test
        void failsForValueAboveMax() {
            var validator = NumberValidator.<Integer>forField("quantity").max(100);

            var result = validator.validate(150);
            assertThat(result.isValid()).isFalse();
            assertThat(result.getErrors().get(0).message()).contains("at most 100");
        }
    }

    @Nested
    @DisplayName("range validation")
    class RangeTests {
        @Test
        void passesForValueInRange() {
            var validator = NumberValidator.<Integer>forField("percentage").range(0, 100);

            assertThat(validator.validate(50).isValid()).isTrue();
            assertThat(validator.validate(0).isValid()).isTrue();
            assertThat(validator.validate(100).isValid()).isTrue();
        }

        @Test
        void failsForValueOutOfRange() {
            var validator = NumberValidator.<Integer>forField("percentage").range(0, 100);

            assertThat(validator.validate(-1).isValid()).isFalse();
            assertThat(validator.validate(101).isValid()).isFalse();
        }
    }

    @Nested
    @DisplayName("positive validation")
    class PositiveTests {
        @Test
        void passesForPositiveValue() {
            var validator = NumberValidator.<Double>forField("price").positive();

            assertThat(validator.validate(9.99).isValid()).isTrue();
        }

        @Test
        void failsForZero() {
            var validator = NumberValidator.<Double>forField("price").positive();

            assertThat(validator.validate(0.0).isValid()).isFalse();
        }

        @Test
        void failsForNegativeValue() {
            var validator = NumberValidator.<Double>forField("price").positive();

            assertThat(validator.validate(-5.0).isValid()).isFalse();
        }
    }

    @Nested
    @DisplayName("nonNegative validation")
    class NonNegativeTests {
        @Test
        void passesForPositiveValue() {
            var validator = NumberValidator.<Integer>forField("count").nonNegative();

            assertThat(validator.validate(10).isValid()).isTrue();
        }

        @Test
        void passesForZero() {
            var validator = NumberValidator.<Integer>forField("count").nonNegative();

            assertThat(validator.validate(0).isValid()).isTrue();
        }

        @Test
        void failsForNegativeValue() {
            var validator = NumberValidator.<Integer>forField("count").nonNegative();

            assertThat(validator.validate(-1).isValid()).isFalse();
        }
    }

    @Nested
    @DisplayName("negative validation")
    class NegativeTests {
        @Test
        void passesForNegativeValue() {
            var validator = NumberValidator.<Integer>forField("temperature").negative();

            assertThat(validator.validate(-10).isValid()).isTrue();
        }

        @Test
        void failsForZero() {
            var validator = NumberValidator.<Integer>forField("temperature").negative();

            assertThat(validator.validate(0).isValid()).isFalse();
        }

        @Test
        void failsForPositiveValue() {
            var validator = NumberValidator.<Integer>forField("temperature").negative();

            assertThat(validator.validate(5).isValid()).isFalse();
        }
    }

    @Nested
    @DisplayName("custom validation")
    class CustomTests {
        @Test
        void passesForCustomPredicate() {
            var validator = NumberValidator.<Integer>forField("value")
                    .custom(n -> n % 2 == 0, "must be even");

            assertThat(validator.validate(4).isValid()).isTrue();
        }

        @Test
        void failsForCustomPredicate() {
            var validator = NumberValidator.<Integer>forField("value")
                    .custom(n -> n % 2 == 0, "must be even");

            var result = validator.validate(3);
            assertThat(result.isValid()).isFalse();
            assertThat(result.getErrors().get(0).message()).isEqualTo("must be even");
        }
    }

    @Nested
    @DisplayName("chained validation")
    class ChainedTests {
        @Test
        void passesAllRules() {
            var validator = NumberValidator.<Integer>forField("age")
                    .notNull()
                    .min(0)
                    .max(150);

            assertThat(validator.validate(25).isValid()).isTrue();
        }

        @Test
        void reportsAllFailures() {
            var validator = NumberValidator.<Integer>forField("value")
                    .min(10)
                    .max(5); // Intentionally impossible to satisfy

            var result = validator.validate(7);

            assertThat(result.isValid()).isFalse();
            assertThat(result.getErrors()).hasSize(2); // Fails both min and max
        }
    }

    @Nested
    @DisplayName("different number types")
    class DifferentTypesTests {
        @Test
        void worksWithDouble() {
            var validator = NumberValidator.<Double>forField("rate").range(0.0, 1.0);

            assertThat(validator.validate(0.5).isValid()).isTrue();
            assertThat(validator.validate(1.5).isValid()).isFalse();
        }

        @Test
        void worksWithLong() {
            var validator = NumberValidator.<Long>forField("id").positive();

            assertThat(validator.validate(12345L).isValid()).isTrue();
            assertThat(validator.validate(-1L).isValid()).isFalse();
        }
    }
}
