package com.jboot.validation.api;

import java.util.List;

/**
 * Exception thrown when validation fails.
 */
public class ValidationException extends RuntimeException {

    private final List<ValidationError> errors;

    public ValidationException(List<ValidationError> errors) {
        super(buildMessage(errors));
        this.errors = List.copyOf(errors);
    }

    public ValidationException(String field, String message) {
        this(List.of(ValidationError.of(field, message)));
    }

    public List<ValidationError> getErrors() {
        return errors;
    }

    private static String buildMessage(List<ValidationError> errors) {
        if (errors.isEmpty()) {
            return "Validation failed";
        }
        var sb = new StringBuilder("Validation failed: ");
        for (int i = 0; i < errors.size(); i++) {
            if (i > 0)
                sb.append(", ");
            var error = errors.get(i);
            sb.append(error.field()).append(": ").append(error.message());
        }
        return sb.toString();
    }
}
