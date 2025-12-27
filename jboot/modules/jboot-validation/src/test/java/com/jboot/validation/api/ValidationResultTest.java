package com.jboot.validation.api;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;

import java.util.List;

import static org.assertj.core.api.Assertions.*;

class ValidationResultTest {

    @Test
    @DisplayName("success creates valid result")
    void successCreatesValidResult() {
        var result = ValidationResult.success();

        assertThat(result.isValid()).isTrue();
        assertThat(result.getErrors()).isEmpty();
    }

    @Test
    @DisplayName("failure creates invalid result")
    void failureCreatesInvalidResult() {
        var errors = List.of(
                ValidationError.of("field1", "error1"),
                ValidationError.of("field2", "error2"));

        var result = ValidationResult.failure(errors);

        assertThat(result.isValid()).isFalse();
        assertThat(result.getErrors()).hasSize(2);
    }

    @Test
    @DisplayName("failure with field and message creates single error")
    void failureWithFieldAndMessage() {
        var result = ValidationResult.failure("email", "invalid format");

        assertThat(result.isValid()).isFalse();
        assertThat(result.getErrors()).hasSize(1);
        assertThat(result.getErrors().get(0).field()).isEqualTo("email");
        assertThat(result.getErrors().get(0).message()).isEqualTo("invalid format");
    }

    @Test
    @DisplayName("getErrorsForField returns only matching errors")
    void getErrorsForFieldReturnsMatching() {
        var errors = List.of(
                ValidationError.of("email", "error1"),
                ValidationError.of("email", "error2"),
                ValidationError.of("name", "error3"));

        var result = ValidationResult.failure(errors);
        var emailErrors = result.getErrorsForField("email");

        assertThat(emailErrors).hasSize(2);
        assertThat(emailErrors).allMatch(e -> e.field().equals("email"));
    }

    @Test
    @DisplayName("throwIfInvalid throws for invalid result")
    void throwIfInvalidThrows() {
        var result = ValidationResult.failure("field", "error");

        assertThatThrownBy(result::throwIfInvalid)
                .isInstanceOf(ValidationException.class)
                .satisfies(e -> {
                    var ex = (ValidationException) e;
                    assertThat(ex.getErrors()).hasSize(1);
                });
    }

    @Test
    @DisplayName("throwIfInvalid does not throw for valid result")
    void throwIfInvalidDoesNotThrowForValid() {
        var result = ValidationResult.success();

        assertThatCode(result::throwIfInvalid).doesNotThrowAnyException();
    }

    @Test
    @DisplayName("merge combines two valid results")
    void mergeTwoValidResults() {
        var result1 = ValidationResult.success();
        var result2 = ValidationResult.success();

        var merged = result1.merge(result2);

        assertThat(merged.isValid()).isTrue();
    }

    @Test
    @DisplayName("merge combines valid and invalid results")
    void mergeValidAndInvalidResults() {
        var valid = ValidationResult.success();
        var invalid = ValidationResult.failure("field", "error");

        var merged = valid.merge(invalid);

        assertThat(merged.isValid()).isFalse();
        assertThat(merged.getErrors()).hasSize(1);
    }

    @Test
    @DisplayName("merge combines two invalid results")
    void mergeTwoInvalidResults() {
        var invalid1 = ValidationResult.failure("field1", "error1");
        var invalid2 = ValidationResult.failure("field2", "error2");

        var merged = invalid1.merge(invalid2);

        assertThat(merged.isValid()).isFalse();
        assertThat(merged.getErrors()).hasSize(2);
    }
}
