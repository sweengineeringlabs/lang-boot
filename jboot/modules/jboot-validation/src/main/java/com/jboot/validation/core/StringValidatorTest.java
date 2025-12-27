package com.jboot.validation.core;

import com.jboot.validation.api.ValidationException;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Nested;

import static org.assertj.core.api.Assertions.*;

class StringValidatorTest {

    @Nested
    @DisplayName("notEmpty validation")
    class NotEmptyTests {
        @Test
        void passesForNonEmptyString() {
            var validator = StringValidator.forField("name").notEmpty();

            var result = validator.validate("John");

            assertThat(result.isValid()).isTrue();
        }

        @Test
        void failsForEmptyString() {
            var validator = StringValidator.forField("name").notEmpty();

            var result = validator.validate("");

            assertThat(result.isValid()).isFalse();
            assertThat(result.getErrors()).hasSize(1);
            assertThat(result.getErrors().get(0).field()).isEqualTo("name");
        }

        @Test
        void failsForNull() {
            var validator = StringValidator.forField("name").notEmpty();

            var result = validator.validate(null);

            assertThat(result.isValid()).isFalse();
        }
    }

    @Nested
    @DisplayName("notBlank validation")
    class NotBlankTests {
        @Test
        void passesForNonBlankString() {
            var validator = StringValidator.forField("name").notBlank();

            var result = validator.validate("John");

            assertThat(result.isValid()).isTrue();
        }

        @Test
        void failsForBlankString() {
            var validator = StringValidator.forField("name").notBlank();

            var result = validator.validate("   ");

            assertThat(result.isValid()).isFalse();
        }
    }

    @Nested
    @DisplayName("length validation")
    class LengthTests {
        @Test
        void minLengthPassesForValidString() {
            var validator = StringValidator.forField("password").minLength(8);

            var result = validator.validate("password123");

            assertThat(result.isValid()).isTrue();
        }

        @Test
        void minLengthFailsForShortString() {
            var validator = StringValidator.forField("password").minLength(8);

            var result = validator.validate("short");

            assertThat(result.isValid()).isFalse();
            assertThat(result.getErrors().get(0).message()).contains("at least 8");
        }

        @Test
        void maxLengthPassesForValidString() {
            var validator = StringValidator.forField("name").maxLength(50);

            var result = validator.validate("John");

            assertThat(result.isValid()).isTrue();
        }

        @Test
        void maxLengthFailsForLongString() {
            var validator = StringValidator.forField("name").maxLength(5);

            var result = validator.validate("Jonathan");

            assertThat(result.isValid()).isFalse();
            assertThat(result.getErrors().get(0).message()).contains("at most 5");
        }

        @Test
        void lengthRangePassesForValidString() {
            var validator = StringValidator.forField("code").length(3, 10);

            var result = validator.validate("ABC123");

            assertThat(result.isValid()).isTrue();
        }

        @Test
        void lengthRangeFailsForOutOfRangeString() {
            var validator = StringValidator.forField("code").length(3, 10);

            var tooShort = validator.validate("AB");
            var tooLong = validator.validate("ABCDEFGHIJK");

            assertThat(tooShort.isValid()).isFalse();
            assertThat(tooLong.isValid()).isFalse();
        }
    }

    @Nested
    @DisplayName("email validation")
    class EmailTests {
        @Test
        void passesForValidEmail() {
            var validator = StringValidator.forField("email").email();

            assertThat(validator.validate("user@example.com").isValid()).isTrue();
            assertThat(validator.validate("john.doe@company.co.uk").isValid()).isTrue();
            assertThat(validator.validate("name+tag@domain.org").isValid()).isTrue();
        }

        @Test
        void failsForInvalidEmail() {
            var validator = StringValidator.forField("email").email();

            assertThat(validator.validate("not-an-email").isValid()).isFalse();
            assertThat(validator.validate("missing@domain").isValid()).isFalse();
            assertThat(validator.validate("@nodomain.com").isValid()).isFalse();
            assertThat(validator.validate("spaces in@email.com").isValid()).isFalse();
        }
    }

    @Nested
    @DisplayName("pattern validation")
    class PatternTests {
        @Test
        void passesForMatchingPattern() {
            var validator = StringValidator.forField("phone").pattern("^\\d{3}-\\d{4}$");

            var result = validator.validate("123-4567");

            assertThat(result.isValid()).isTrue();
        }

        @Test
        void failsForNonMatchingPattern() {
            var validator = StringValidator.forField("phone").pattern("^\\d{3}-\\d{4}$");

            var result = validator.validate("1234567");

            assertThat(result.isValid()).isFalse();
        }
    }

    @Nested
    @DisplayName("alphanumeric validation")
    class AlphanumericTests {
        @Test
        void passesForAlphanumericString() {
            var validator = StringValidator.forField("code").alphanumeric();

            assertThat(validator.validate("ABC123").isValid()).isTrue();
            assertThat(validator.validate("test").isValid()).isTrue();
            assertThat(validator.validate("12345").isValid()).isTrue();
        }

        @Test
        void failsForNonAlphanumericString() {
            var validator = StringValidator.forField("code").alphanumeric();

            assertThat(validator.validate("abc-123").isValid()).isFalse();
            assertThat(validator.validate("hello world").isValid()).isFalse();
            assertThat(validator.validate("test@123").isValid()).isFalse();
        }
    }

    @Nested
    @DisplayName("numeric validation")
    class NumericTests {
        @Test
        void passesForNumericString() {
            var validator = StringValidator.forField("id").numeric();

            assertThat(validator.validate("12345").isValid()).isTrue();
            assertThat(validator.validate("0").isValid()).isTrue();
        }

        @Test
        void failsForNonNumericString() {
            var validator = StringValidator.forField("id").numeric();

            assertThat(validator.validate("123abc").isValid()).isFalse();
            assertThat(validator.validate("-123").isValid()).isFalse();
        }
    }

    @Nested
    @DisplayName("oneOf validation")
    class OneOfTests {
        @Test
        void passesForAllowedValue() {
            var validator = StringValidator.forField("status").oneOf("ACTIVE", "INACTIVE", "PENDING");

            assertThat(validator.validate("ACTIVE").isValid()).isTrue();
            assertThat(validator.validate("PENDING").isValid()).isTrue();
        }

        @Test
        void failsForDisallowedValue() {
            var validator = StringValidator.forField("status").oneOf("ACTIVE", "INACTIVE", "PENDING");

            var result = validator.validate("UNKNOWN");

            assertThat(result.isValid()).isFalse();
            assertThat(result.getErrors().get(0).message()).contains("must be one of");
        }
    }

    @Nested
    @DisplayName("custom validation")
    class CustomTests {
        @Test
        void passesForCustomPredicate() {
            var validator = StringValidator.forField("code")
                    .custom(s -> s.startsWith("PRE-"), "must start with PRE-");

            var result = validator.validate("PRE-12345");

            assertThat(result.isValid()).isTrue();
        }

        @Test
        void failsForCustomPredicate() {
            var validator = StringValidator.forField("code")
                    .custom(s -> s.startsWith("PRE-"), "must start with PRE-");

            var result = validator.validate("POST-12345");

            assertThat(result.isValid()).isFalse();
            assertThat(result.getErrors().get(0).message()).isEqualTo("must start with PRE-");
        }
    }

    @Nested
    @DisplayName("chained validation")
    class ChainedTests {
        @Test
        void passesAllRules() {
            var validator = StringValidator.forField("username")
                    .notEmpty()
                    .minLength(3)
                    .maxLength(20)
                    .alphanumeric();

            var result = validator.validate("user123");

            assertThat(result.isValid()).isTrue();
        }

        @Test
        void reportsAllFailures() {
            var validator = StringValidator.forField("username")
                    .notEmpty()
                    .minLength(5)
                    .alphanumeric();

            var result = validator.validate("ab!");

            assertThat(result.isValid()).isFalse();
            assertThat(result.getErrors()).hasSize(2); // minLength and alphanumeric
        }

        @Test
        void throwsOnInvalid() {
            var validator = StringValidator.forField("email")
                    .notEmpty()
                    .email();

            assertThatThrownBy(() -> validator.validateOrThrow("not-an-email"))
                    .isInstanceOf(ValidationException.class)
                    .satisfies(e -> {
                        var ex = (ValidationException) e;
                        assertThat(ex.getErrors()).hasSize(1);
                    });
        }
    }
}
