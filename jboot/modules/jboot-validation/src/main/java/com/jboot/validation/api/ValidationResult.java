package com.jboot.validation.api;

import java.util.List;

/**
 * Represents the result of a validation operation.
 */
public interface ValidationResult {

    /**
     * Returns true if validation passed with no errors.
     */
    boolean isValid();

    /**
     * Returns the list of validation errors.
     */
    List<ValidationError> getErrors();

    /**
     * Returns errors for a specific field.
     */
    List<ValidationError> getErrorsForField(String fieldName);

    /**
     * Throws a ValidationException if validation failed.
     */
    void throwIfInvalid() throws ValidationException;

    /**
     * Merges this result with another.
     */
    ValidationResult merge(ValidationResult other);

    /**
     * Creates a successful validation result.
     */
    static ValidationResult success() {
        return new DefaultValidationResult(List.of());
    }

    /**
     * Creates a failed validation result.
     */
    static ValidationResult failure(List<ValidationError> errors) {
        return new DefaultValidationResult(errors);
    }

    /**
     * Creates a failed validation result with a single error.
     */
    static ValidationResult failure(String field, String message) {
        return new DefaultValidationResult(List.of(
                ValidationError.of(field, message)));
    }
}
