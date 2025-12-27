package com.jboot.validation.api;

/**
 * A validator that validates objects of type T.
 *
 * @param <T> The type of object being validated
 */
@FunctionalInterface
public interface Validator<T> {

    /**
     * Validates the given value.
     *
     * @param value The value to validate
     * @return The validation result
     */
    ValidationResult validate(T value);

    /**
     * Validates and throws if invalid.
     *
     * @param value The value to validate
     * @throws ValidationException if validation fails
     */
    default void validateOrThrow(T value) {
        validate(value).throwIfInvalid();
    }

    /**
     * Composes this validator with another.
     */
    default Validator<T> and(Validator<T> other) {
        return value -> this.validate(value).merge(other.validate(value));
    }

    /**
     * Creates a validator that always succeeds.
     */
    static <T> Validator<T> alwaysValid() {
        return value -> ValidationResult.success();
    }

    /**
     * Creates a validator that always fails with the given message.
     */
    static <T> Validator<T> alwaysInvalid(String field, String message) {
        return value -> ValidationResult.failure(field, message);
    }
}
