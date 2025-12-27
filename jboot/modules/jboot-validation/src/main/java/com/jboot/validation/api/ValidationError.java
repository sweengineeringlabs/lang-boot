package com.jboot.validation.api;

/**
 * Represents a single validation error.
 */
public record ValidationError(
        String field,
        String message,
        String code,
        Object invalidValue) {
    /**
     * Creates a validation error with field and message.
     */
    public static ValidationError of(String field, String message) {
        return new ValidationError(field, message, null, null);
    }

    /**
     * Creates a validation error with field, message, and code.
     */
    public static ValidationError of(String field, String message, String code) {
        return new ValidationError(field, message, code, null);
    }

    /**
     * Creates a full validation error.
     */
    public static ValidationError of(String field, String message, String code, Object invalidValue) {
        return new ValidationError(field, message, code, invalidValue);
    }
}
